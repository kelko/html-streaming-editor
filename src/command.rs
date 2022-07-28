use crate::{CssSelectorList, HtmlIndex};
use std::collections::HashSet;
use tl::NodeHandle;

pub enum Command<'a> {
    Only(CssSelectorList<'a>),
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
        //TODO: Fix. too many nodes returned
        let findings = selector.query(index, input);

        let parser = index.dom.parser();
        for node in findings.iter() {
            node.get(parser)
                .unwrap()
                .inner_html(parser)
                .to_mut()
                .clear()
        }

        Ok(input.clone())
    }
}
