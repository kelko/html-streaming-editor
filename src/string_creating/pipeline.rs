use crate::string_creating::{ElementSelectingCommand, ValueExtractingCommand};
use crate::{CommandFailedSnafu, HtmlContent, PipelineError};
use snafu::ResultExt;

#[derive(Debug, PartialEq, Clone)]
pub struct StringValueCreatingPipeline<'a> {
    element_selector: ElementSelectingCommand<'a>,
    value_extractor: ValueExtractingCommand<'a>,
    //todo: value_processing: Vec<ValueProcessingCommand<'a>>
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
        self.value_extractor
            .execute(&element)
            .context(CommandFailedSnafu { index: 1_usize })
    }
}

#[cfg(test)]
mod test {
    use crate::string_creating::{ElementSelectingCommand, ValueExtractingCommand};
    use crate::{HtmlContent, StringValueCreatingPipeline};

    #[test]
    fn get_attr_from_element_returns_correct_value() {
        let pipeline = StringValueCreatingPipeline::new(
            ElementSelectingCommand::UseElement,
            ValueExtractingCommand::GetAttribute("data-test"),
        );

        let dom = tl::parse(
            r#"<div data-test="foo" class="bar"></div>"#,
            tl::ParserOptions::default(),
        )
        .unwrap();
        let starting_element = HtmlContent::import(dom).unwrap();

        let mut result = pipeline.run_on(&starting_element).unwrap();

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

        let dom = tl::parse(
            r#"<div data-test="foo" class="bar"></div>"#,
            tl::ParserOptions::default(),
        )
        .unwrap();
        let starting_element = HtmlContent::import(dom).unwrap();

        let result = pipeline.run_on(&starting_element).unwrap();

        assert_eq!(result.len(), 0);
    }
}
