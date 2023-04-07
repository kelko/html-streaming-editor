use rctree::{Children, Node};
use snafu::{Backtrace, Snafu};
use std::collections::BTreeMap;

use crate::CssSelector;
use tl::{HTMLTag, HTMLVersion, NodeHandle, Parser, VDom};

#[cfg(test)]
mod tests;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum HtmlDomError {
    #[snafu(display("Nothing Imported from tl"))]
    NothingImported { backtrace: Backtrace },
    #[snafu(display("Node not resolved by Parser"))]
    InvalidParserState { backtrace: Backtrace },
    #[snafu(display("HTML Document has invalid structure: {}", message))]
    InvalidHtmlDocument {
        message: &'static str,
        backtrace: Backtrace,
    },
}

const HTML_VOID_ELEMENTS: [&str; 16] = [
    "area", "base", "br", "col", "command", "embed", "hr", "img", "input", "keygen", "link",
    "meta", "param", "source", "track", "wbr",
];

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct HtmlDocument {
    pub doctype: Option<HTMLVersion>,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct HtmlTag {
    pub name: String,
    pub attributes: BTreeMap<String, String>,
}

impl HtmlTag {
    pub(crate) fn of_name(name: impl Into<String>) -> Self {
        HtmlTag {
            name: name.into(),
            attributes: BTreeMap::<String, String>::new(),
        }
    }

    pub(crate) fn build_start_tag(&self, mut add_string: impl FnMut(String)) {
        add_string(format!("<{}", self.name));
        self.attributes
            .iter()
            .for_each(|(key, value)| add_string(format!(r#" {}="{}""#, key, value)));
        add_string(String::from(">"));
    }

    pub(crate) fn build_end_tag(&self, mut add_string: impl FnMut(String)) {
        if HTML_VOID_ELEMENTS.contains(&self.name.as_ref()) {
            return;
        }

        add_string(format!("</{}>", self.name));
    }

    fn matches_selector(&self, selector: &CssSelector) -> bool {
        if let Some(element) = selector.element {
            if element.as_bytes() != self.name.as_bytes() {
                return false;
            }
        }

        if let Some(id) = selector.id {
            if let Some(tag_id) = self.attributes.get(&String::from("id")) {
                if id.as_bytes() != tag_id.as_bytes() {
                    return false;
                }
            } else {
                return false;
            }
        }

        for class in &selector.classes {
            if !self.is_class_member(class) {
                return false;
            }
        }

        for _pseudo_class in &selector.pseudo_classes {
            todo!("Implement pseudo-class support")
        }

        for attribute in &selector.attributes {
            if let Some(attribute_value) = self.attributes.get(&String::from(attribute.attribute)) {
                if !attribute.matches(attribute_value) {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    fn is_class_member(&self, class: &str) -> bool {
        if let Some(classes) = self.attributes.get(&String::from("class")) {
            classes.split(' ').any(|c| c == class)
        } else {
            false
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum HtmlContent {
    Document(HtmlDocument),
    Tag(HtmlTag),
    Text(String),
    Comment(String),
}

impl HtmlContent {
    pub(crate) fn is_tag(&self) -> bool {
        matches!(self, HtmlContent::Tag(_))
    }

    pub(crate) fn import(dom: VDom) -> Result<Node<HtmlContent>, HtmlDomError> {
        let (root_tag, root_tag_name) = Self::find_root_tag(&dom)?;

        if root_tag_name == *"html" {
            let document = Node::new(HtmlContent::Document(HtmlDocument {
                doctype: dom.version(),
            }));
            document.append(root_tag);

            Ok(document)
        } else {
            Ok(root_tag)
        }
    }

    fn find_root_tag(dom: &VDom) -> Result<(Node<HtmlContent>, String), HtmlDomError> {
        let parser = dom.parser();

        for child in dom.children() {
            if let Some(node) = child.get(parser) {
                if let Some(tag) = node.as_tag() {
                    let name = String::from(tag.name().as_utf8_str());
                    let converted = Self::convert_tag(tag, parser)?;
                    return Ok((converted, name));
                }
            }
        }

        NothingImportedSnafu {}.fail()
    }

    fn convert_tag(tag: &HTMLTag, parser: &Parser) -> Result<Node<HtmlContent>, HtmlDomError> {
        let name = String::from(tag.name().as_utf8_str());
        let mut attributes = BTreeMap::new();

        for (key, value) in tag.attributes().iter() {
            let value_string = if let Some(value_content) = value {
                String::from(value_content)
            } else {
                String::new()
            };

            attributes.insert(String::from(key), value_string);
        }

        let converted = Node::<HtmlContent>::new(HtmlContent::Tag(HtmlTag { name, attributes }));

        for child in tag.children().top().iter() {
            converted.append(Self::convert_node(child, parser)?)
        }

        Ok(converted)
    }

    fn convert_node(
        node_handle: &NodeHandle,
        parser: &Parser,
    ) -> Result<Node<HtmlContent>, HtmlDomError> {
        if let Some(node) = node_handle.get(parser) {
            return match node {
                tl::Node::Tag(tag) => Self::convert_tag(tag, parser),
                tl::Node::Raw(text) => Self::convert_text(text.as_utf8_str()),
                tl::Node::Comment(comment) => Self::convert_comment(comment.as_utf8_str()),
            };
        }

        InvalidParserStateSnafu {}.fail()
    }

    fn convert_text(text: impl Into<String>) -> Result<Node<HtmlContent>, HtmlDomError> {
        Ok(Node::new(HtmlContent::Text(text.into())))
    }

    fn convert_comment(comment: impl Into<String>) -> Result<Node<HtmlContent>, HtmlDomError> {
        let comment = comment.into();
        let comment = comment.trim_start_matches("<!--");
        let comment = comment.trim_end_matches("-->");
        let comment = comment.trim();
        Ok(Node::new(HtmlContent::Comment(comment.into())))
    }

    fn inner_html(&self, children: Children<HtmlContent>) -> String {
        match self {
            HtmlContent::Comment(_) => String::new(),
            HtmlContent::Text(s) => s.clone(),
            HtmlContent::Document(d) => {
                let mut inner_content = children
                    .into_iter()
                    .map(|c| c.outer_html())
                    .collect::<Vec<_>>();
                if let Some(doctype) = &d.doctype {
                    inner_content.insert(0, doctype.outer_html());
                    inner_content.insert(1, String::from('\n'));
                }

                inner_content.join("")
            }
            HtmlContent::Tag(_t) => children
                .into_iter()
                .map(|c| c.outer_html())
                .collect::<Vec<_>>()
                .join(""),
        }
    }

    fn outer_html(&self, children: Children<HtmlContent>) -> String {
        match self {
            HtmlContent::Comment(s) => format!("<!-- {} -->", s),
            HtmlContent::Text(s) => s.clone(),
            HtmlContent::Document(_) => self.inner_html(children),
            HtmlContent::Tag(t) => {
                let mut parts = Vec::<String>::new();
                t.build_start_tag(|content| parts.push(content));

                for child in children {
                    parts.push(child.outer_html());
                }

                t.build_end_tag(|content| parts.push(content));
                parts.join("")
            }
        }
    }

    fn text_content(&self, children: Children<HtmlContent>) -> String {
        match self {
            HtmlContent::Comment(_) => String::new(),
            HtmlContent::Text(s) => s.clone(),
            HtmlContent::Tag(_) | HtmlContent::Document(_) => children
                .into_iter()
                .filter_map(|c| {
                    let child_render = c.text_content();

                    if child_render.is_empty() {
                        None
                    } else {
                        Some(child_render)
                    }
                })
                .collect::<Vec<_>>()
                .join(" "),
        }
    }

    fn matches_selector(&self, selector: &CssSelector) -> bool {
        match self {
            HtmlContent::Comment(_) | HtmlContent::Text(_) | HtmlContent::Document(_) => false,
            HtmlContent::Tag(t) => t.matches_selector(selector),
        }
    }

    pub(crate) fn clear_attribute(&mut self, attribute: &String) {
        match self {
            HtmlContent::Comment(_) | HtmlContent::Text(_) | HtmlContent::Document(_) => (),
            HtmlContent::Tag(tag) => {
                tag.attributes.remove(attribute);
            }
        }
    }

    pub(crate) fn set_attribute(&mut self, attribute: impl Into<String>, value: impl Into<String>) {
        match self {
            HtmlContent::Comment(_) | HtmlContent::Text(_) | HtmlContent::Document(_) => (),
            HtmlContent::Tag(tag) => {
                tag.attributes.insert(attribute.into(), value.into());
            }
        }
    }

    pub(crate) fn get_attribute(&self, attribute: &String) -> Option<String> {
        match self {
            HtmlContent::Comment(_) | HtmlContent::Text(_) | HtmlContent::Document(_) => None,
            HtmlContent::Tag(tag) => tag.attributes.get(attribute).cloned(),
        }
    }
}

pub(crate) trait HtmlRenderable {
    /// Returns the markup of all child elements
    ///
    /// ## Limitations
    /// - The tag attributes are written in alphabetical order
    /// - Spaces within the tag are not preserved (i.e. `<img      src="">` may become `<img src="">`)
    /// - Does no escaping to change as little as possible of the input HTML. Input has to be correctly escaped
    ///
    /// Equivalent to [Element#innerHTML](https://developer.mozilla.org/en-US/docs/Web/API/Element/innerHTML) in browsers
    fn inner_html(&self) -> String;
    /// Returns the markup building up this element an all children
    ///
    /// ## Limitations
    /// - The tag attributes are written in alphabetical order
    /// - Spaces within the tag are not preserved (i.e. `<img      src="">` may become `<img src="">`)
    /// - Does no escaping to change as little as possible of the input HTML. Input has to be correctly escaped
    ///
    /// Equivalent to [Element#outerHTML](https://developer.mozilla.org/en-US/docs/Web/API/Element/outerHTML) in browsers
    fn outer_html(&self) -> String;
    /// Returns the contained (non-comment) text of this element, excluding any markup.
    /// - Does no escaping to change as little as possible of the input HTML. Input has to be correctly escaped
    ///
    /// Equivalent to [Element#outerHTML](https://developer.mozilla.org/en-US/docs/Web/API/Node/textContent) in browsers
    fn text_content(&self) -> String;
}

impl HtmlRenderable for Node<HtmlContent> {
    fn inner_html(&self) -> String {
        let children = self.children();
        let inner = self.borrow();

        inner.inner_html(children)
    }

    fn outer_html(&self) -> String {
        let children = self.children();
        let inner = self.borrow();

        inner.outer_html(children)
    }

    fn text_content(&self) -> String {
        let children = self.children();
        let inner = self.borrow();

        inner.text_content(children)
    }
}

impl HtmlRenderable for HTMLVersion {
    fn inner_html(&self) -> String {
        String::new()
    }

    fn outer_html(&self) -> String {
        match self {
            HTMLVersion::HTML5 => String::from("<!DOCTYPE html>"),
            HTMLVersion::StrictHTML401 => String::from(
                r#"<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01//EN" "http://www.w3.org/TR/html4/strict.dtd">"#,
            ),
            HTMLVersion::TransitionalHTML401 => String::from(
                r#"<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01 Transitional//EN" "http://www.w3.org/TR/1999/REC-html401-19991224/loose.dtd">"#,
            ),
            HTMLVersion::FramesetHTML401 => String::from(
                r#"<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01 Frameset//EN" "http://www.w3.org/TR/1999/REC-html401-19991224/frameset.dtd">"#,
            ),
        }
    }

    fn text_content(&self) -> String {
        String::new()
    }
}

pub(crate) trait HtmlQueryable {
    fn matches_selector(&self, selector: &CssSelector) -> bool;
}

impl HtmlQueryable for Node<HtmlContent> {
    fn matches_selector(&self, selector: &CssSelector) -> bool {
        let inner = self.borrow();
        inner.matches_selector(selector)
    }
}
