use crate::element_processing::command::ElementProcessingCommand;
use log::{trace, warn};
use snafu::ResultExt;
use std::fmt::Debug;

use crate::html::HtmlContent;
use crate::{CommandFailedSnafu, PipelineError};

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct ElementProcessingPipeline<'a>(Vec<ElementProcessingCommand<'a>>);

/// The command pipeline: a list of individual commands
/// each to execute on the result of the previous command
impl<'a> ElementProcessingPipeline<'a> {
    pub fn new(content: Vec<ElementProcessingCommand<'a>>) -> Self {
        ElementProcessingPipeline(content)
    }

    /// execute the pipeline on the given nodes by
    /// running the first commands on those nodes and all the following commands
    /// on their predecessors result.
    /// The result of the last command is the result of this pipeline
    pub(crate) fn run_on(
        &self,
        nodes: Vec<rctree::Node<HtmlContent>>,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, PipelineError> {
        let mut intermediate = nodes;
        let mut command_index: usize = 0;
        for command in self.0.iter() {
            trace!("Running Next: {:#?}", &command);
            trace!("Current Element Set: {:#?}", &intermediate);

            intermediate = command.execute(&intermediate).context(CommandFailedSnafu {
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
