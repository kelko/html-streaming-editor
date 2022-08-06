use crate::command::Command;
use crate::HtmlIndex;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use tl::NodeHandle;

pub struct Pipeline<'a>(Vec<Command<'a>>);

/// The command pipeline: a list of individual commands
/// each to execute on the result of the previous command
impl<'a> Pipeline<'a> {
    pub fn new(content: Vec<Command<'a>>) -> Self {
        Pipeline(content)
    }

    /// execute the pipeline on the given nodes by
    /// running the first commands on those nodes and all the following commands
    /// on their predecessors result.
    /// The result of the last command is the result of this pipeline
    pub(crate) fn run_on(
        &self,
        nodes: HashSet<NodeHandle>,
        index: &'_ HtmlIndex<'a>,
    ) -> Result<HashSet<NodeHandle>, ()> {
        let mut intermediate = nodes;
        for command in self.0.iter() {
            intermediate = command.execute(&intermediate, index)?;
        }

        return Ok(intermediate);
    }
}

impl<'a> Debug for Pipeline<'a> {
    //TODO: Actually implement it
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("TODO")
    }
}

impl<'a> PartialEq for Pipeline<'a> {
    fn eq(&self, other: &Self) -> bool {
        return self.0.eq(&other.0);
    }
}
