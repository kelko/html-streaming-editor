use log::trace;
use snafu::{ResultExt, Snafu};
use std::collections::HashSet;
use std::fmt::Debug;

use tl::NodeHandle;

use crate::{CssSelectorList, HtmlIndex};

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
    // Map(String, Pipeline),
    // GetAttribute(String),
    // SetAttribute(String, Pipeline),
    // RemoveAttribute(String),
    // GetText(),
    // SetText(Pipeline),
    // RemoveText(),
}

impl<'a> Command<'a> {
    /// perform the action defined by the command on the set of nodes
    /// and return the calculated results.
    /// For some command the output can be equal to the input,
    /// others change the result-set
    pub(crate) fn execute(
        &self,
        input: &HashSet<NodeHandle>,
        index: &'_ HtmlIndex<'a>,
    ) -> Result<HashSet<NodeHandle>, CommandError> {
        match self {
            Command::Only(selector) => Self::only(input, index, selector),
            Command::Without(selector) => {
                Self::without(input, index, selector).context(WithoutFailedSnafu)
            }
        }
    }

    fn only(
        input: &HashSet<NodeHandle>,
        index: &'_ HtmlIndex<'a>,
        selector: &CssSelectorList<'a>,
    ) -> Result<HashSet<NodeHandle>, CommandError> {
        trace!("Running ONLY command using selector: {:#?}", selector);
        Ok(selector.query(index, input))
    }

    fn without(
        input: &HashSet<NodeHandle>,
        index: &'_ HtmlIndex<'a>,
        selector: &CssSelectorList<'a>,
    ) -> Result<HashSet<NodeHandle>, WithoutError> {
        trace!("Running WITHOUT command using selector: {:#?}", selector);
        let findings = selector.query(index, input);

        for node in findings.iter() {
            index.remove(node).context(RemovingNodeFailedSnafu)?
        }

        Ok(input.clone())
    }
}
