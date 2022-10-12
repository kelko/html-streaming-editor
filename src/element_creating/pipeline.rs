use crate::element_creating::ElementCreatingCommand;
use crate::element_processing::ElementProcessingCommand;
use crate::{CommandFailedSnafu, HtmlContent, PipelineError};
use log::{trace, warn};
use snafu::ResultExt;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ElementCreatingPipeline<'a>(
    ElementCreatingCommand<'a>,
    Vec<ElementProcessingCommand<'a>>,
);

/// The command pipeline: a list of individual commands
/// each to execute on the result of the previous command
impl<'a> ElementCreatingPipeline<'a> {
    pub fn new(
        creation: ElementCreatingCommand<'a>,
        processing: Option<Vec<ElementProcessingCommand<'a>>>,
    ) -> Self {
        ElementCreatingPipeline(creation, processing.unwrap_or_default())
    }

    /// execute the pipeline on the given nodes by
    /// running the first commands on those nodes and all the following commands
    /// on their predecessors result.
    /// The result of the last command is the result of this pipeline
    pub(crate) fn run_on(
        &self,
        nodes: Vec<rctree::Node<HtmlContent>>,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, PipelineError> {
        let mut intermediate = self
            .0
            .execute(&nodes)
            .context(CommandFailedSnafu { index: 0_usize })?;

        if intermediate.is_empty() {
            warn!("Command resulted in an empty result set");
        }

        let mut command_index: usize = 1;
        for command in self.1.iter() {
            trace!("Running Next: {:#?}", &command);
            trace!("Current Element Set: {:#?}", &intermediate);

            intermediate = command.execute(&intermediate).context(CommandFailedSnafu {
                index: command_index,
            })?;
            command_index += 1;

            if intermediate.is_empty() {
                warn!("Command resulted in an empty result set");
            }
        }

        Ok(intermediate)
    }
}
