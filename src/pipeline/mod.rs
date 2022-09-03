use log::{trace, warn};
use snafu::{ResultExt, Snafu};
use std::collections::HashSet;
use std::fmt::Debug;
use tl::NodeHandle;

use crate::command::Command;
use crate::HtmlIndex;

#[cfg(test)]
mod tests;

#[derive(Debug, Snafu)]
pub enum PipelineError {
    #[snafu(display("Command at index {index} failed"))]
    CommandFailed {
        index: usize,
        #[snafu(backtrace)]
        source: crate::command::CommandError,
    },
}

#[derive(Debug, PartialEq)]
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
    ) -> Result<HashSet<NodeHandle>, PipelineError> {
        let mut intermediate = nodes;
        let mut command_index: usize = 0;
        for command in self.0.iter() {
            trace!("Running Next: {:#?}", &command);
            trace!("Current Element Set: {:#?}", &intermediate);

            intermediate = command
                .execute(&intermediate, index)
                .context(CommandFailedSnafu {
                    index: command_index,
                })?;
            command_index += 1;

            if intermediate.len() == 0 {
                warn!("Command resulted in an empty result set");
            }
        }

        return Ok(intermediate);
    }
}
