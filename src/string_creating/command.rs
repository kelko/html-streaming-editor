use crate::{CommandError, CssSelectorList, HtmlContent, HtmlRenderable, ParsingRegexFailedSnafu};
use rctree::Node;
use regex::Regex;
use snafu::ResultExt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ElementSelectingCommand<'a> {
    /// Returns the previously selected element
    UseElement,
    /// Returns the parent of the previously selected element (if exists)
    UseParent,
    /// Run a CSS selector on the previously selected element
    QueryElement(CssSelectorList<'a>),
    /// Run a CSS selector on the parent of the previously selected element (if exists)
    QueryParent(CssSelectorList<'a>),
    /// Run a CSS selector on the root of the tree the previously selected element belongs to
    /// If the previously selected element is the root, the selector is run against that
    QueryRoot(CssSelectorList<'a>),
}

impl<'a> ElementSelectingCommand<'a> {
    /// perform the action defined by the command on the set of nodes
    /// and return the calculated results.
    /// For some command the output can be equal to the input,
    /// others change the result-set
    pub(crate) fn execute(
        &self,
        input: &rctree::Node<HtmlContent>,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        match self {
            ElementSelectingCommand::UseElement => Self::use_element(input),
            ElementSelectingCommand::UseParent => Self::use_parent(input),
            ElementSelectingCommand::QueryElement(selector) => Self::query_element(input, selector),
            ElementSelectingCommand::QueryParent(selector) => Self::query_parent(input, selector),
            ElementSelectingCommand::QueryRoot(selector) => Self::query_root(input, selector),
        }
    }

    fn use_element(input: &Node<HtmlContent>) -> Result<Vec<Node<HtmlContent>>, CommandError> {
        Ok(vec![rctree::Node::clone(input)])
    }

    fn use_parent(input: &Node<HtmlContent>) -> Result<Vec<Node<HtmlContent>>, CommandError> {
        if let Some(parent) = input.parent() {
            return Ok(vec![parent]);
        }

        Ok(vec![])
    }

    fn query_element(
        input: &Node<HtmlContent>,
        selector: &CssSelectorList<'a>,
    ) -> Result<Vec<Node<HtmlContent>>, CommandError> {
        Ok(selector.query(&[rctree::Node::clone(input)]))
    }

    fn query_parent(
        input: &Node<HtmlContent>,
        selector: &CssSelectorList<'a>,
    ) -> Result<Vec<Node<HtmlContent>>, CommandError> {
        if let Some(parent) = input.parent() {
            return Ok(selector.query(&[parent]));
        }

        Ok(vec![])
    }

    fn query_root(
        input: &Node<HtmlContent>,
        selector: &CssSelectorList<'a>,
    ) -> Result<Vec<Node<HtmlContent>>, CommandError> {
        let mut root = Node::clone(input);

        while let Some(parent) = root.parent() {
            root = parent;
        }

        Ok(selector.query(&[root]))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ValueExtractingCommand<'a> {
    /// returns the content of a named attribute
    GetAttribute(&'a str),
    /// return the text content of that element
    GetTextContent,
}

impl<'a> ValueExtractingCommand<'a> {
    /// perform the action defined by the command on the set of nodes
    /// and return the calculated results.
    pub(crate) fn execute(&self, input: &[Node<HtmlContent>]) -> Result<Vec<String>, CommandError> {
        match self {
            ValueExtractingCommand::GetAttribute(attr_name) => {
                Self::get_attribute(input, attr_name)
            }
            ValueExtractingCommand::GetTextContent => Self::get_text_content(input),
        }
    }

    fn get_attribute(
        input: &[Node<HtmlContent>],
        attr_name: &str,
    ) -> Result<Vec<String>, CommandError> {
        let attribute = String::from(attr_name);
        Ok(input
            .iter()
            .filter_map(|n| {
                let data = n.borrow();

                data.get_attribute(&attribute)
            })
            .collect::<Vec<_>>())
    }

    fn get_text_content(input: &[Node<HtmlContent>]) -> Result<Vec<String>, CommandError> {
        Ok(input
            .iter()
            .filter_map(|n| {
                let content = n.text_content();
                if content.is_empty() {
                    None
                } else {
                    Some(content)
                }
            })
            .collect::<Vec<_>>())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ValueProcessingCommand<'a> {
    RegexReplace(&'a str, &'a str),
    /// returns an all-lower-case version of the input
    ToLower,
    /// returns an all-upper-case version of the input
    ToUpper,
}

impl<'a> ValueProcessingCommand<'a> {
    /// perform the action defined by the command on the set of nodes
    /// and return the calculated results.
    pub(crate) fn execute(&self, input: &[String]) -> Result<Vec<String>, CommandError> {
        match self {
            ValueProcessingCommand::RegexReplace(regex, replace) => {
                Self::regex_replace(input, regex, replace)
            }
            ValueProcessingCommand::ToLower => Self::to_lower(input),
            ValueProcessingCommand::ToUpper => Self::to_upper(input),
        }
    }

    fn regex_replace(
        input: &[String],
        regex: &str,
        replace: &str,
    ) -> Result<Vec<String>, CommandError> {
        let re = Regex::new(regex).context(ParsingRegexFailedSnafu)?;

        Ok(input
            .iter()
            .map(|v| re.replace_all(v, replace))
            .map(|v| String::from(v))
            .collect::<Vec<_>>())
    }

    fn to_lower(input: &[String]) -> Result<Vec<String>, CommandError> {
        Ok(input.iter().map(|v| v.to_lowercase()).collect::<Vec<_>>())
    }

    fn to_upper(input: &[String]) -> Result<Vec<String>, CommandError> {
        Ok(input.iter().map(|v| v.to_uppercase()).collect::<Vec<_>>())
    }
}

#[cfg(test)]
mod test {
    use crate::string_creating::command::ValueProcessingCommand;
    use crate::string_creating::{ElementSelectingCommand, ValueExtractingCommand};
    use crate::{load_inline_html, CssSelector, CssSelectorList, CssSelectorPath};

    #[test]
    fn use_element_returns_self() {
        let root = load_inline_html(r#"<div data-test="foo" class="bar"></div>"#);
        let command = ElementSelectingCommand::UseElement;

        let mut result = command.execute(&root).unwrap();

        assert_eq!(result.len(), 1);
        let first_result = result.pop().unwrap();
        assert_eq!(first_result, root);
    }

    #[test]
    fn use_parent_returns_parent_on_existing_parent() {
        let root = load_inline_html(
            r#"<div id="parent" data-test="foo"><div class="bar" data-test="fubar"></div></div>"#,
        );
        let target_node = root.first_child().unwrap();
        let command = ElementSelectingCommand::UseParent;

        let mut result = command.execute(&target_node).unwrap();

        assert_eq!(result.len(), 1);
        let first_result = result.pop().unwrap();
        assert_eq!(first_result, root);
    }

    #[test]
    fn use_parent_returns_empty_on_root() {
        let root = load_inline_html(
            r#"<div id="parent" data-test="foo"><div class="bar" data-test="fubar"></div></div>"#,
        );
        let command = ElementSelectingCommand::UseParent;

        let result = command.execute(&root).unwrap();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn querying_element_returns_matching_element() {
        let root = load_inline_html(
            r#"<div class="bar" data-test="fubar"><aside class="test-source" data-test="foo"></aside></div>"#,
        );
        let command = ElementSelectingCommand::QueryElement(CssSelectorList::new(vec![
            CssSelectorPath::single(CssSelector::for_class("test-source")),
        ]));

        let mut result = command.execute(&root).unwrap();

        assert_eq!(result.len(), 1);
        let first_result = result.pop().unwrap();
        assert_eq!(first_result, root.first_child().unwrap());
    }

    #[test]
    fn querying_element_returns_multiple_matching_elements() {
        let root = load_inline_html(
            r#"<div class="bar" data-test="fubar"><aside class="test-source" data-test="foo"></aside><div><p class="test-source"></p></div></div>"#,
        );
        let command = ElementSelectingCommand::QueryElement(CssSelectorList::new(vec![
            CssSelectorPath::single(CssSelector::for_class("test-source")),
        ]));

        let result = command.execute(&root).unwrap();

        assert_eq!(result.len(), 2);
        assert!(result.contains(&root.first_child().unwrap()));
        assert!(result.contains(&root.last_child().unwrap().first_child().unwrap()));
    }

    #[test]
    fn query_element_returns_empty_on_querying_nonexistent_el() {
        let root = load_inline_html(
            r#"<div class="bar" data-test="fubar"><aside data-test="foo"></aside></div>"#,
        );
        let command = ElementSelectingCommand::QueryElement(CssSelectorList::new(vec![
            CssSelectorPath::single(CssSelector::for_class("test-source")),
        ]));

        let result = command.execute(&root).unwrap();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn querying_parent_returns_matching_element() {
        let root = load_inline_html(
            r#"<div id="parent"><div class="bar" data-test="fubar"></div><aside class="test-source" data-test="foo"></aside></div>"#,
        );
        let target_node = root.first_child().unwrap();
        let command = ElementSelectingCommand::QueryParent(CssSelectorList::new(vec![
            CssSelectorPath::single(CssSelector::for_class("test-source")),
        ]));

        let mut result = command.execute(&target_node).unwrap();

        assert_eq!(result.len(), 1);
        let first_result = result.pop().unwrap();
        assert_eq!(first_result, target_node.next_sibling().unwrap());
    }

    #[test]
    fn querying_parent_returns_multiple_matching_elements() {
        let root = load_inline_html(
            r#"<div id="parent"><div class="bar" data-test="fubar"></div><aside class="test-source" data-test="foo"><p class="test-source"></p></aside></div>"#,
        );
        let target_node = root.first_child().unwrap();
        let command = ElementSelectingCommand::QueryParent(CssSelectorList::new(vec![
            CssSelectorPath::single(CssSelector::for_class("test-source")),
        ]));

        let result = command.execute(&target_node).unwrap();

        assert_eq!(result.len(), 2);
        assert!(result.contains(&target_node.next_sibling().unwrap()));
        assert!(result.contains(&target_node.next_sibling().unwrap().first_child().unwrap()));
    }

    #[test]
    fn query_parent_returns_empty_on_root() {
        let root = load_inline_html(
            r#"<div id="parent"><div class="bar" data-test="fubar"></div><aside class="test-source" data-test="foo"></aside></div>"#,
        );
        let command = ElementSelectingCommand::QueryParent(CssSelectorList::new(vec![
            CssSelectorPath::single(CssSelector::for_class("test-source")),
        ]));

        let result = command.execute(&root).unwrap();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn query_parent_returns_empty_on_querying_nonexistent_el() {
        let root = load_inline_html(
            r#"<div id="parent"><div class="bar" data-test="fubar"></div><aside data-test="foo"></aside></div>"#,
        );
        let target_node = root.first_child().unwrap();
        let command = ElementSelectingCommand::QueryParent(CssSelectorList::new(vec![
            CssSelectorPath::single(CssSelector::for_class("test-source")),
        ]));

        let result = command.execute(&target_node).unwrap();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn query_parent_returns_empty_on_matching_element_outside_parent() {
        let root = load_inline_html(
            r#"<div><div id="parent"><div class="bar" data-test="fubar"></div><aside data-test="foo"></aside></div><aside class="test-source"></aside></div>"#,
        );
        let target_node = root.first_child().unwrap().first_child().unwrap();
        let command = ElementSelectingCommand::QueryParent(CssSelectorList::new(vec![
            CssSelectorPath::single(CssSelector::for_class("test-source")),
        ]));

        let result = command.execute(&target_node).unwrap();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn query_root_returns_matching_element() {
        let root = load_inline_html(
            r#"<div id="root"><div><div><div class="bar" data-test="fubar"></div></div></div><aside class="test-source" data-test="foo"></aside></div>"#,
        );
        let target_node = root
            .first_child()
            .unwrap()
            .first_child()
            .unwrap()
            .first_child()
            .unwrap();
        let command = ElementSelectingCommand::QueryRoot(CssSelectorList::new(vec![
            CssSelectorPath::single(CssSelector::for_class("test-source")),
        ]));

        let mut result = command.execute(&target_node).unwrap();

        assert_eq!(result.len(), 1);
        let first_result = result.pop().unwrap();
        assert_eq!(first_result, root.last_child().unwrap());
    }

    #[test]
    fn query_root_returns_multiple_matching_elements() {
        let root = load_inline_html(
            r#"<div id="root"><div><div><div class="bar" data-test="fubar"></div><aside class="test-source"></aside></div></div><aside class="test-source" data-test="foo"></aside></div>"#,
        );
        let target_node = root
            .first_child()
            .unwrap()
            .first_child()
            .unwrap()
            .first_child()
            .unwrap();
        let command = ElementSelectingCommand::QueryRoot(CssSelectorList::new(vec![
            CssSelectorPath::single(CssSelector::for_class("test-source")),
        ]));

        let result = command.execute(&target_node).unwrap();

        assert_eq!(result.len(), 2);
        assert!(result.contains(&root.last_child().unwrap()));
        assert!(result.contains(&target_node.next_sibling().unwrap()));
    }

    #[test]
    fn query_root_on_root_queries_itself() {
        let root = load_inline_html(
            r#"<div data-test="fubar" class="bar"><aside class="test-source" data-test="foo"></aside></div>"#,
        );
        let command = ElementSelectingCommand::QueryRoot(CssSelectorList::new(vec![
            CssSelectorPath::single(CssSelector::for_class("test-source")),
        ]));

        let mut result = command.execute(&root).unwrap();

        assert_eq!(result.len(), 1);
        let first_result = result.pop().unwrap();
        assert_eq!(first_result, root.last_child().unwrap());
    }

    #[test]
    fn query_root_return_empty_on_nonexistent_el() {
        let root = load_inline_html(
            r#"<div id="root"><div><div><div class="bar" data-test="fubar"></div></div></div><aside data-test="foo"></aside></div>"#,
        );
        let target_node = root
            .first_child()
            .unwrap()
            .first_child()
            .unwrap()
            .first_child()
            .unwrap();
        let command = ElementSelectingCommand::QueryRoot(CssSelectorList::new(vec![
            CssSelectorPath::single(CssSelector::for_class("test-source")),
        ]));

        let result = command.execute(&target_node).unwrap();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn get_attr_returns_value_on_existing_attr() {
        let root = load_inline_html(r#"<div data-test="foo" class="bar"></div>"#);
        let command = ValueExtractingCommand::GetAttribute("data-test");

        let mut result = command.execute(&vec![root]).unwrap();

        assert_eq!(result.len(), 1);
        let first_result = result.pop().unwrap();
        assert_eq!(first_result, String::from("foo"));
    }

    #[test]
    fn get_attr_returns_empty_on_missing_attr() {
        let root = load_inline_html(r#"<div class="bar"></div>"#);
        let command = ValueExtractingCommand::GetAttribute("data-test");

        let result = command.execute(&vec![root]).unwrap();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn get_attr_returns_empty_on_empty_input() {
        let command = ValueExtractingCommand::GetAttribute("data-test");

        let result = command.execute(&vec![]).unwrap();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn get_text_content_returns_correct_value_on_existing_content() {
        let root = load_inline_html(r#"<div>The content</div>"#);
        let command = ValueExtractingCommand::GetTextContent;

        let mut result = command.execute(&vec![root]).unwrap();

        assert_eq!(result.len(), 1);

        let first_result = result.pop().unwrap();
        assert_eq!(first_result, String::from("The content"));
    }

    #[test]
    fn get_text_content_returns_empty_string_on_empty_content() {
        let root = load_inline_html(r#"<div></div>"#);
        let command = ValueExtractingCommand::GetTextContent;

        let result = command.execute(&vec![root]).unwrap();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn get_text_content_returns_empty_string_on_empty_input() {
        let command = ValueExtractingCommand::GetTextContent;

        let result = command.execute(&vec![]).unwrap();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn regex_replaces_uses_indexed_group_correctly() {
        let command = ValueProcessingCommand::RegexReplace("(He)(l+)o", "-> $2 <-");

        let mut result = command.execute(&[String::from("Hello")]).unwrap();

        assert_eq!(result.len(), 1);

        let first_result = result.pop().unwrap();
        assert_eq!(first_result, String::from("-> ll <-"));
    }

    #[test]
    fn regex_replaces_uses_named_groups_correctly() {
        let command = ValueProcessingCommand::RegexReplace("(?P<s>He)(?P<m>l+)o", "-> $m <-");

        let mut result = command.execute(&[String::from("Hello")]).unwrap();

        assert_eq!(result.len(), 1);

        let first_result = result.pop().unwrap();
        assert_eq!(first_result, String::from("-> ll <-"));
    }

    #[test]
    fn regex_replaces_uses_number_classes_correctly() {
        let command = ValueProcessingCommand::RegexReplace("\\d", "##");

        let mut result = command
            .execute(&[String::from("And one, 2, three, 4")])
            .unwrap();

        assert_eq!(result.len(), 1);

        let first_result = result.pop().unwrap();
        assert_eq!(first_result, String::from("And one, ##, three, ##"));
    }

    #[test]
    fn regex_replaces_uses_whitespace_classes_correctly() {
        let command = ValueProcessingCommand::RegexReplace("\\s", "_");

        let mut result = command
            .execute(&[String::from("And one, 2, three, 4")])
            .unwrap();

        assert_eq!(result.len(), 1);

        let first_result = result.pop().unwrap();
        assert_eq!(first_result, String::from("And_one,_2,_three,_4"));
    }

    #[test]
    fn regex_replaces_uses_nonword_classes_correctly() {
        let command = ValueProcessingCommand::RegexReplace("\\W", "_");

        let mut result = command
            .execute(&[String::from("And one, 2, three, 4")])
            .unwrap();

        assert_eq!(result.len(), 1);

        let first_result = result.pop().unwrap();
        assert_eq!(first_result, String::from("And_one__2__three__4"));
    }

    #[test]
    fn regex_replaces_changes_values_correctly() {
        let command = ValueProcessingCommand::RegexReplace("a", "e");

        let mut result = command.execute(&[String::from("Hallo")]).unwrap();

        assert_eq!(result.len(), 1);

        let first_result = result.pop().unwrap();
        assert_eq!(first_result, String::from("Hello"));
    }

    #[test]
    fn regex_replaces_changes_only_occurrences_correctly() {
        let command = ValueProcessingCommand::RegexReplace("a", "e");

        let mut result = command.execute(&[String::from("Apples are good")]).unwrap();

        assert_eq!(result.len(), 1);

        let first_result = result.pop().unwrap();
        assert_eq!(first_result, String::from("Apples ere good"));
    }

    #[test]
    fn regex_replaces_changes_all_occurrences_correctly() {
        let command = ValueProcessingCommand::RegexReplace("A", "E");

        let mut result = command.execute(&[String::from("Apples Are Good")]).unwrap();

        assert_eq!(result.len(), 1);

        let first_result = result.pop().unwrap();
        assert_eq!(first_result, String::from("Epples Ere Good"));
    }

    #[test]
    fn regex_replaces_changes_all_inputs_correctly() {
        let command = ValueProcessingCommand::RegexReplace("a", "e");

        let mut result = command
            .execute(&[String::from("Hallo"), String::from("apples are good")])
            .unwrap();

        assert_eq!(result.len(), 2);

        let second_result = result.pop().unwrap();
        let first_result = result.pop().unwrap();
        assert_eq!(second_result, String::from("epples ere good"));
        assert_eq!(first_result, String::from("Hello"));
    }

    #[test]
    fn regex_replaces_returns_empty_string_on_empty_input() {
        let command = ValueProcessingCommand::RegexReplace("a", "e");

        let result = command.execute(&vec![]).unwrap();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn to_lower_lowercases_ascii_characters_correctly() {
        let command = ValueProcessingCommand::ToLower;

        let mut result = command
            .execute(&[String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ")])
            .unwrap();

        assert_eq!(result.len(), 1);

        let first_result = result.pop().unwrap();
        assert_eq!(first_result, String::from("abcdefghijklmnopqrstuvwxyz"));
    }

    #[test]
    fn to_lower_lowercases_german_umlauts_correctly() {
        let command = ValueProcessingCommand::ToLower;

        let mut result = command.execute(&[String::from("ÄÖÜ")]).unwrap();

        assert_eq!(result.len(), 1);

        let first_result = result.pop().unwrap();
        assert_eq!(first_result, String::from("äöü"));
    }

    #[test]
    fn to_lower_returns_empty_on_empty_input() {
        let command = ValueProcessingCommand::ToLower;

        let result = command.execute(&vec![]).unwrap();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn to_upper_uppercases_ascii_characters_correctly() {
        let command = ValueProcessingCommand::ToUpper;

        let mut result = command
            .execute(&[String::from("abcdefghijklmnopqrstuvwxyz")])
            .unwrap();

        assert_eq!(result.len(), 1);

        let first_result = result.pop().unwrap();
        assert_eq!(first_result, String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ"));
    }

    #[test]
    fn to_upper_uppercases_german_umlauts_correctly() {
        let command = ValueProcessingCommand::ToUpper;

        let mut result = command.execute(&[String::from("äöü")]).unwrap();

        assert_eq!(result.len(), 1);

        let first_result = result.pop().unwrap();
        assert_eq!(first_result, String::from("ÄÖÜ"));
    }

    #[test]
    fn to_upper_returns_empty_on_empty_input() {
        let command = ValueProcessingCommand::ToUpper;

        let result = command.execute(&vec![]).unwrap();

        assert_eq!(result.len(), 0);
    }
}
