use log::warn;
use rctree::{Children, Node};
use snafu::{Backtrace, Snafu};
use std::collections::BTreeMap;

use crate::{CssSelector, ValueSource};
use tl::{HTMLTag, NodeHandle, Parser, VDom};

#[cfg(test)]
mod tests;

const HTML_VOID_ELEMENTS: [&str; 16] = [
    "area", "base", "br", "col", "command", "embed", "hr", "img", "input", "keygen", "link",
    "meta", "param", "source", "track", "wbr",
];

#[derive(Debug, PartialEq)]
pub(crate) struct HtmlTag {
    pub name: String,
    pub attributes: BTreeMap<String, String>,
}

impl HtmlTag {
    #[cfg(test)]
    fn of_name(name: impl Into<String>) -> Self {
        HtmlTag {
            name: name.into(),
            attributes: BTreeMap::<String, String>::new(),
        }
    }

    pub(crate) fn build_start_tag(&self, mut add_string: impl FnMut(String) -> ()) {
        add_string(format!("<{}", self.name));
        self.attributes
            .iter()
            .for_each(|(key, value)| add_string(format!(r#" {}="{}""#, key, value)));
        add_string(String::from(">"));
    }

    pub(crate) fn build_end_tag(&self, mut add_string: impl FnMut(String) -> ()) {
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

    fn is_class_member(&self, class: &&str) -> bool {
        if let Some(classes) = self.attributes.get(&String::from("class")) {
            if let Some(_) = classes.split(" ").into_iter().find(|c| c == class) {
                return true;
            }
        }

        false
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum HtmlContent {
    Tag(HtmlTag),
    Text(String),
    Comment(String),
}

impl HtmlContent {
    pub(crate) fn is_tag(&self) -> bool {
        match self {
            HtmlContent::Tag(_) => true,
            _ => false,
        }
    }

    pub(crate) fn import(dom: VDom) -> Result<Node<HtmlContent>, ()> {
        let parser = dom.parser();
        let mut converted = dom
            .children()
            .iter()
            .filter_map(|node_handle| {
                if let Some(node) = node_handle.get(parser) {
                    if let Some(tag) = node.as_tag() {
                        return Some(Self::convert_tag(tag, parser));
                    }
                }

                None
            })
            .collect::<Vec<_>>();

        if converted.len() > 1 {
            warn!("VDom seemed to have more then one root tag")
        }

        if let Some(root_result) = converted.pop() {
            root_result
        } else {
            Err(())
        }
    }

    fn convert_tag(tag: &HTMLTag, parser: &Parser) -> Result<Node<HtmlContent>, ()> {
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

        let mut converted =
            Node::<HtmlContent>::new(HtmlContent::Tag(HtmlTag { name, attributes }));

        for child in tag.children().top().iter() {
            converted.append(Self::convert_node(child, parser)?)
        }

        Ok(converted)
    }

    fn convert_node(node_handle: &NodeHandle, parser: &Parser) -> Result<Node<HtmlContent>, ()> {
        if let Some(node) = node_handle.get(parser) {
            return match node {
                tl::Node::Tag(tag) => Self::convert_tag(tag, parser),
                tl::Node::Raw(text) => Self::convert_text(text.as_utf8_str()),
                tl::Node::Comment(comment) => Self::convert_comment(comment.as_utf8_str()),
            };
        }

        Err(())
    }

    fn convert_text(text: impl Into<String>) -> Result<Node<HtmlContent>, ()> {
        Ok(Node::new(HtmlContent::Text(text.into())))
    }

    fn convert_comment(comment: impl Into<String>) -> Result<Node<HtmlContent>, ()> {
        let comment = comment.into();
        let comment = comment.trim_start_matches("<!-- ");
        let comment = comment.trim_end_matches(" -->");
        Ok(Node::new(HtmlContent::Comment(comment.into())))
    }

    fn inner_html(&self, children: Children<HtmlContent>) -> String {
        match self {
            HtmlContent::Comment(_) => String::new(),
            HtmlContent::Text(s) => s.clone(),
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
            HtmlContent::Tag(_t) => children
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
            HtmlContent::Comment(_) | HtmlContent::Text(_) => false,
            HtmlContent::Tag(t) => t.matches_selector(selector),
        }
    }

    pub(crate) fn clear_attribute(&mut self, attribute: &String) {
        match self {
            HtmlContent::Comment(_) | HtmlContent::Text(_) => (),
            HtmlContent::Tag(tag) => {
                tag.attributes.remove(attribute);
            }
        }
    }

    pub(crate) fn set_attribute(&mut self, attribute: &String, value_source: &ValueSource) {
        match self {
            HtmlContent::Comment(_) | HtmlContent::Text(_) => (),
            HtmlContent::Tag(tag) => {
                tag.attributes
                    .insert(attribute.clone(), value_source.render());
            }
        }
    }
}

pub(crate) trait HtmlRenderable {
    fn inner_html(&self) -> String;
    fn outer_html(&self) -> String;
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

#[derive(Debug, Snafu)]
pub enum IndexError {
    #[snafu(display("Index seems out of date. NodeHandle couldn't be found in Parser"))]
    OutdatedIndex { backtrace: Backtrace },
    #[snafu(display("HTML seems broken: {operation} failed"))]
    InvalidHtml {
        operation: String,
        backtrace: Backtrace,
    },
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
