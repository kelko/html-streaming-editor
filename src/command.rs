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

/// Is the value directly defined or is it a sub-pipeline?
#[derive(Debug, PartialEq, Clone)]
pub enum ValueSource {
    StringValue(String),
}

impl ValueSource {
    pub(crate) fn render(&self) -> String {
        match self {
            ValueSource::StringValue(value) => value.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Command<'a> {
    /// Find all nodes, beginning at the input, that match the given CSS selector
    /// and return only those
    Only(CssSelectorList<'a>),
    /// Find all nodes, beginning at the input, that match the given CSS selector
    /// and remove them from their parent nodes.
    /// Returns the input as result.
    Without(CssSelectorList<'a>),
    /// Remove the given attribute from all currently selected nodes
    /// Returns the input as result.
    ClearAttribute(String),
    /// Remove all children of the currently selected nodes
    /// Returns the input as result
    ClearContent,
    /// Add or Reset a given attribute with a new value
    /// Returns the input as result.
    SetAttribute(String, ValueSource),
    /// Remove all children of the currently selected nodes and add a new text as child instead
    /// Returns the input as result.
    SetTextContent(ValueSource),
    /// adds a new text as child
    /// Returns the input as result.
    AddTextContent(ValueSource),
    /// adds a new comment as child
    /// Returns the input as result.
    AddComment(ValueSource),
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
            Command::SetAttribute(attribute, value_source) => {
                Self::set_attr(input, attribute, value_source)
            }
            Command::SetTextContent(value_source) => Self::set_text_content(input, value_source),
            Command::AddTextContent(value_source) => Self::add_text_content(input, value_source),
            Command::AddComment(value_source) => Self::add_comment(input, value_source),
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

        Ok(input.clone())
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

        Ok(input.clone())
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

        Ok(input.clone())
    }

    fn set_attr(
        input: &Vec<rctree::Node<HtmlContent>>,
        attribute: &String,
        value_source: &ValueSource,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!(
            "Running SET-ATTR command for attr: {:#?} with value: {:#?}",
            attribute,
            value_source
        );

        for node in input {
            let mut working_copy = rctree::Node::clone(node);
            let mut data = working_copy.borrow_mut();
            data.set_attribute(attribute, value_source);
        }

        Ok(input.clone())
    }

    fn set_text_content(
        input: &Vec<rctree::Node<HtmlContent>>,
        value_source: &ValueSource,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!(
            "Running SET-TEXT-CONTENT command with value: {:#?}",
            value_source
        );

        for node in input {
            // first clear everything that was there before
            for mut child in node.children() {
                child.detach()
            }

            let mut working_copy = rctree::Node::clone(node);
            working_copy.append(rctree::Node::new(HtmlContent::Text(value_source.render())));
        }

        Ok(input.clone())
    }

    fn add_text_content(
        input: &Vec<rctree::Node<HtmlContent>>,
        value_source: &ValueSource,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!(
            "Running ADD-TEXT-CONTENT command with value: {:#?}",
            value_source
        );

        for node in input {
            let mut working_copy = rctree::Node::clone(node);
            working_copy.append(rctree::Node::new(HtmlContent::Text(value_source.render())));
        }

        Ok(input.clone())
    }

    fn add_comment(
        input: &Vec<rctree::Node<HtmlContent>>,
        value_source: &ValueSource,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!(
            "Running ADD-COMMENT command with value: {:#?}",
            value_source
        );

        for node in input {
            let mut working_copy = rctree::Node::clone(node);
            working_copy.append(rctree::Node::new(HtmlContent::Comment(
                value_source.render(),
            )));
        }

        Ok(input.clone())
    }
}
