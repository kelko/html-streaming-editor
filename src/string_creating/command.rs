use crate::{CommandError, CssSelectorList, HtmlContent, HtmlRenderable};
use rctree::Node;

#[derive(Debug, PartialEq, Clone)]
pub enum ElementSelectingCommand<'a> {
    /// Returns the previously selected element
    UseElement,
    /// Returns the parent of the previously selected element (if exists)
    UseParent,
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

    fn query_parent(
        input: &Node<HtmlContent>,
        selector: &CssSelectorList<'a>,
    ) -> Result<Vec<Node<HtmlContent>>, CommandError> {
        if let Some(parent) = input.parent() {
            return Ok(selector.query(&vec![parent]));
        }

        Ok(vec![])
    }

    fn query_root(
        input: &Node<HtmlContent>,
        selector: &CssSelectorList<'a>,
    ) -> Result<Vec<Node<HtmlContent>>, CommandError> {
        let mut root = Node::clone(input);

        loop {
            if let Some(parent) = root.parent() {
                root = parent;
            } else {
                break;
            }
        }

        Ok(selector.query(&vec![root]))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ValueExtractingCommand<'a> {
    /// Returns the previously selected element
    GetAttribute(&'a str),
    GetTextContent,
}

impl<'a> ValueExtractingCommand<'a> {
    /// perform the action defined by the command on the set of nodes
    /// and return the calculated results.
    /// For some command the output can be equal to the input,
    /// others change the result-set
    pub(crate) fn execute(
        &self,
        input: &Vec<rctree::Node<HtmlContent>>,
    ) -> Result<Vec<String>, CommandError> {
        match self {
            ValueExtractingCommand::GetAttribute(attr_name) => {
                Self::get_attribute(input, attr_name)
            }
            ValueExtractingCommand::GetTextContent => Self::get_text_content(input),
        }
    }

    fn get_attribute(
        input: &Vec<Node<HtmlContent>>,
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

    fn get_text_content(input: &Vec<Node<HtmlContent>>) -> Result<Vec<String>, CommandError> {
        Ok(input.iter().map(|n| n.text_content()).collect::<Vec<_>>())
    }
}
