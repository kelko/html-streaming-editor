use crate::{CssSelectorList, HtmlIndex};
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use tl::NodeHandle;

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
        index: &'a HtmlIndex<'a>,
    ) -> Result<HashSet<NodeHandle>, ()> {
        match self {
            Command::Only(selector) => Self::only(input, index, selector),
            Command::Filter(selector) => Self::filter(input, index, selector),
        }
    }
    fn only(
        input: &HashSet<NodeHandle>,
        index: &'a HtmlIndex<'a>,
        selector: &CssSelectorList<'a>,
    ) -> Result<HashSet<NodeHandle>, ()> {
        Ok(selector.query(index, input))
    }

    fn filter(
        input: &HashSet<NodeHandle>,
        index: &'a HtmlIndex<'a>,
        selector: &CssSelectorList<'a>,
    ) -> Result<HashSet<NodeHandle>, ()> {
        let findings = selector.query(index, input);

        //TODO: code below seems to not change the actual nodes in the DOM. Needs different approach
        let parser = index.dom.parser();
        for node in findings.iter() {
            (*node.get(parser).unwrap().inner_html(parser).to_mut()).clear()
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
