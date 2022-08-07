use snafu::{ResultExt, Snafu};
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};

use tl::NodeHandle;

use crate::{CssSelectorList, HtmlIndex};

#[derive(Debug, Snafu)]
pub enum CommandError {
    #[snafu(display("Failed run FILTER"))]
    FilterFailed {
        #[snafu(backtrace)]
        source: FilterError,
    },
}

#[derive(Debug, Snafu)]
pub enum FilterError {
    #[snafu(display("Failed to remove HTML node"))]
    RemovingNodeFailed {
        #[snafu(backtrace)]
        source: crate::html::IndexError,
    },
}

pub enum Command<'a> {
    /// Find all nodes, beginning at the input, that match the given CSS selector
    /// and return only those
    Only(CssSelectorList<'a>),
    /// Find all nodes, beginning at the input, that match the given CSS selector
    /// and remove them from their parent nodes.
    /// Returns the input as result.
    Filter(CssSelectorList<'a>),
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
            Command::Filter(selector) => {
                Self::filter(input, index, selector).context(FilterFailedSnafu)
            }
        }
    }

    fn only(
        input: &HashSet<NodeHandle>,
        index: &'_ HtmlIndex<'a>,
        selector: &CssSelectorList<'a>,
    ) -> Result<HashSet<NodeHandle>, CommandError> {
        Ok(selector.query(index, input))
    }

    fn filter(
        input: &HashSet<NodeHandle>,
        index: &'_ HtmlIndex<'a>,
        selector: &CssSelectorList<'a>,
    ) -> Result<HashSet<NodeHandle>, FilterError> {
        let findings = selector.query(index, input);

        for node in findings.iter() {
            index.remove(node).context(RemovingNodeFailedSnafu)?
        }

        Ok(input.clone())
    }
}

impl<'a> Debug for Command<'a> {
    //TODO: Actually implement it
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("TODO")
    }
}

impl<'a> PartialEq for Command<'a> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Command::Only(selector) => {
                if let Command::Only(other_selector) = other {
                    selector == other_selector
                } else {
                    false
                }
            }
            Command::Filter(selector) => {
                if let Command::Filter(other_selector) = other {
                    selector == other_selector
                } else {
                    false
                }
            }
        }
    }
}
