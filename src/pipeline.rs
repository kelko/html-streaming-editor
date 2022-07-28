use crate::command::Command;
use crate::HtmlIndex;
use std::collections::HashSet;
use tl::NodeHandle;

pub struct Pipeline<'a>(Vec<Command<'a>>);

impl<'a> Pipeline<'a> {
    pub fn new(content: Vec<Command<'a>>) -> Self {
        Pipeline(content)
    }

    pub(crate) fn run_on(
        &self,
        nodes: HashSet<NodeHandle>,
        index: &'a HtmlIndex<'a>,
    ) -> Result<HashSet<NodeHandle>, ()> {
        let mut intermediate = nodes;
        for command in self.0.iter() {
            intermediate = command.execute(&intermediate, index)?;
        }

        return Ok(intermediate);
    }
}
