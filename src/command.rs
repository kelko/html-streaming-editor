use log::trace;
use snafu::{ResultExt, Snafu};
use std::fmt::Debug;

use crate::html::HtmlContent;
use crate::CssSelectorList;

#[derive(Debug, Snafu)]
pub enum CommandError {
    #[snafu(display("Failed run WITHOUT"))]
    WithoutFailed {
        #[snafu(backtrace)]
        source: WithoutError,
    },
}

#[derive(Debug, Snafu)]
pub enum WithoutError {
    #[snafu(display("Failed to remove HTML node"))]
    RemovingNodeFailed {
        #[snafu(backtrace)]
        source: crate::html::IndexError,
    },
}

#[derive(Debug, PartialEq)]
pub enum Command<'a> {
    /// Find all nodes, beginning at the input, that match the given CSS selector
    /// and return only those
    Only(CssSelectorList<'a>),
    /// Find all nodes, beginning at the input, that match the given CSS selector
    /// and remove them from their parent nodes.
    /// Returns the input as result.
    Without(CssSelectorList<'a>),
    ClearAttribute(String),
    ClearContent,
    //SetAttribute
    //SetTextContent
    //AddTextContent
    //AddElement
    //AddComment
}

impl<'a> Command<'a> {
    /// perform the action defined by the command on the set of nodes
    /// and return the calculated results.
    /// For some command the output can be equal to the input,
    /// others change the result-set
    pub(crate) fn execute(
        &self,
        input: &Vec<rctree::Node<HtmlContent>>,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        match self {
            Command::Only(selector) => Self::only(input, selector),
            Command::Without(selector) => {
                Self::without(input, selector).context(WithoutFailedSnafu)
            }
            Command::ClearAttribute(attribute) => Self::clear_attr(input, attribute),
            Command::ClearContent => Self::clear_content(input),
        }
    }

    fn only(
        input: &Vec<rctree::Node<HtmlContent>>,
        selector: &CssSelectorList<'a>,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!("Running ONLY command using selector: {:#?}", selector);
        Ok(selector.query(input))
    }

    fn without(
        input: &Vec<rctree::Node<HtmlContent>>,
        selector: &CssSelectorList<'a>,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, WithoutError> {
        trace!("Running WITHOUT command using selector: {:#?}", selector);
        let findings = selector.query(input);

        for mut node in findings {
            node.detach();
        }

        Ok(input
            .iter()
            .map(|n| rctree::Node::clone(n))
            .collect::<Vec<_>>())
    }

    fn clear_attr(
        input: &Vec<rctree::Node<HtmlContent>>,
        attribute: &String,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!("Running CLEAR-ATTR command for attr: {:#?}", attribute);

        for node in input {
            let mut working_copy = rctree::Node::clone(node);
            let mut data = working_copy.borrow_mut();
            data.clear_attribute(attribute);
        }

        Ok(input
            .iter()
            .map(|n| rctree::Node::clone(n))
            .collect::<Vec<_>>())
    }

    fn clear_content(
        input: &Vec<rctree::Node<HtmlContent>>,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!("Running CLEAR-CONTENT command");

        for node in input {
            for mut child in node.children() {
                child.detach()
            }
        }

        Ok(input
            .iter()
            .map(|n| rctree::Node::clone(n))
            .collect::<Vec<_>>())
    }
}
