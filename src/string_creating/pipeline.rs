use crate::string_creating::command::ValueProcessingCommand;
use crate::string_creating::{ElementSelectingCommand, ValueExtractingCommand};
use crate::{CommandFailedSnafu, HtmlContent, PipelineError};
use snafu::ResultExt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StringValueCreatingPipeline<'a> {
    element_selector: ElementSelectingCommand<'a>,
    value_extractor: ValueExtractingCommand<'a>,
    value_processing: Vec<ValueProcessingCommand<'a>>,
}

/// The command pipeline: a list of individual commands
/// each to execute on the result of the previous command
impl<'a> StringValueCreatingPipeline<'a> {
    pub const fn new(
        element_selector: ElementSelectingCommand<'a>,
        value_extractor: ValueExtractingCommand<'a>,
    ) -> Self {
        StringValueCreatingPipeline {
            element_selector,
            value_extractor,
            value_processing: vec![],
        }
    }

    pub const fn with_value_processing(
        element_selector: ElementSelectingCommand<'a>,
        value_extractor: ValueExtractingCommand<'a>,
        value_processing: Vec<ValueProcessingCommand<'a>>,
    ) -> Self {
        StringValueCreatingPipeline {
            element_selector,
            value_extractor,
            value_processing,
        }
    }

    /// execute the pipeline on the given nodes by
    /// running the first commands on those nodes and all the following commands
    /// on their predecessors result.
    /// The result of the last command is the result of this pipeline
    pub(crate) fn run_on(
        &self,
        node: &rctree::Node<HtmlContent>,
    ) -> Result<Vec<String>, PipelineError> {
        let element = self
            .element_selector
            .execute(node)
            .context(CommandFailedSnafu { index: 0_usize })?;

        let mut intermediate = self
            .value_extractor
            .execute(&element)
            .context(CommandFailedSnafu { index: 1_usize })?;

        for (command_index, processing_command) in self.value_processing.iter().enumerate() {
            intermediate =
                processing_command
                    .execute(&intermediate)
                    .context(CommandFailedSnafu {
                        index: command_index + 2,
                    })?
        }

        Ok(intermediate)
    }
}

#[cfg(test)]
mod test {
    use crate::string_creating::command::ValueProcessingCommand;
    use crate::string_creating::{ElementSelectingCommand, ValueExtractingCommand};
    use crate::{load_inline_html, StringValueCreatingPipeline};

    #[test]
    fn get_attr_from_element_returns_correct_value() {
        let pipeline = StringValueCreatingPipeline::new(
            ElementSelectingCommand::UseElement,
            ValueExtractingCommand::GetAttribute("data-test"),
        );

        let root = load_inline_html(r#"<div data-test="foo" class="bar"></div>"#);

        let mut result = pipeline.run_on(&root).unwrap();

        assert_eq!(result.len(), 1);
        let first_result = result.pop().unwrap();
        assert_eq!(first_result, String::from("foo"));
    }

    #[test]
    fn get_attr_returns_empty_for_empty_selection() {
        let pipeline = StringValueCreatingPipeline::new(
            ElementSelectingCommand::UseParent,
            ValueExtractingCommand::GetAttribute("data-other"),
        );

        let root = load_inline_html(r#"<div data-test="foo" class="bar"></div>"#);

        let result = pipeline.run_on(&root).unwrap();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn get_attr_from_element_regex_replace_returns_correct_value() {
        let pipeline = StringValueCreatingPipeline::with_value_processing(
            ElementSelectingCommand::UseElement,
            ValueExtractingCommand::GetAttribute("data-test"),
            vec![ValueProcessingCommand::RegexReplace("f", "z")],
        );

        let root = load_inline_html(r#"<div data-test="foo" class="bar"></div>"#);

        let mut result = pipeline.run_on(&root).unwrap();

        assert_eq!(result.len(), 1);
        let first_result = result.pop().unwrap();
        assert_eq!(first_result, String::from("zoo"));
    }

    #[test]
    fn get_attr_from_element_2_regex_replaces_returns_correct_value() {
        let pipeline = StringValueCreatingPipeline::with_value_processing(
            ElementSelectingCommand::UseElement,
            ValueExtractingCommand::GetAttribute("data-test"),
            vec![
                ValueProcessingCommand::RegexReplace("f", "z"),
                ValueProcessingCommand::RegexReplace("o", "a"),
            ],
        );

        let root = load_inline_html(r#"<div data-test="foo" class="bar"></div>"#);

        let mut result = pipeline.run_on(&root).unwrap();

        assert_eq!(result.len(), 1);
        let first_result = result.pop().unwrap();
        assert_eq!(first_result, String::from("zaa"));
    }
}
