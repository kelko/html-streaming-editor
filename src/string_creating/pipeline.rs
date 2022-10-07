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
    use crate::{
        CssSelector, CssSelectorList, CssSelectorPath, HtmlContent, StringValueCreatingPipeline,
    };

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
    fn get_attr_returns_empty_on_missing_attr() {
        let pipeline = StringValueCreatingPipeline::new(
            ElementSelectingCommand::UseElement,
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

    #[test]
    fn get_attr_from_parent_returns_correct_value() {
        let pipeline = StringValueCreatingPipeline::new(
            ElementSelectingCommand::UseParent,
            ValueExtractingCommand::GetAttribute("data-test"),
        );

        let dom = tl::parse(
            r#"<div id="parent" data-test="foo"><div class="bar" data-test="fubar"></div></div>"#,
            tl::ParserOptions::default(),
        )
        .unwrap();
        let starting_element = HtmlContent::import(dom).unwrap();
        let target_node = starting_element.first_child().unwrap();

        let mut result = pipeline.run_on(&target_node).unwrap();

        assert_eq!(result.len(), 1);
        let first_result = result.pop().unwrap();
        assert_eq!(first_result, String::from("foo"));
    }

    #[test]
    fn get_attr_returns_empty_for_use_parent_on_root() {
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

    #[test]
    fn get_attr_querying_parent_returns_correct_value() {
        let pipeline = StringValueCreatingPipeline::new(
            ElementSelectingCommand::QueryParent(CssSelectorList::new(vec![
                CssSelectorPath::single(CssSelector::for_class("test-source")),
            ])),
            ValueExtractingCommand::GetAttribute("data-test"),
        );

        let dom = tl::parse(
            r#"<div id="parent"><div class="bar" data-test="fubar"></div><aside class="test-source" data-test="foo"></aside</div>"#,
            tl::ParserOptions::default(),
        )
            .unwrap();
        let starting_element = HtmlContent::import(dom).unwrap();
        let target_node = starting_element.first_child().unwrap();

        let mut result = pipeline.run_on(&target_node).unwrap();

        assert_eq!(result.len(), 1);
        let first_result = result.pop().unwrap();
        assert_eq!(first_result, String::from("foo"));
    }

    #[test]
    fn get_attr_returns_empty_for_query_parent_on_root() {
        let pipeline = StringValueCreatingPipeline::new(
            ElementSelectingCommand::QueryParent(CssSelectorList::new(vec![
                CssSelectorPath::single(CssSelector::for_class("test-source")),
            ])),
            ValueExtractingCommand::GetAttribute("data-test"),
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

    #[test]
    fn get_attr_returns_empty_for_querying_nonexistent_el_on_parent() {
        let pipeline = StringValueCreatingPipeline::new(
            ElementSelectingCommand::QueryParent(CssSelectorList::new(vec![
                CssSelectorPath::single(CssSelector::for_class("test-source")),
            ])),
            ValueExtractingCommand::GetAttribute("data-test"),
        );

        let dom = tl::parse(
            r#"<div id="parent"><div class="bar" data-test="fubar"></div><aside data-test="foo"></aside</div>"#,
            tl::ParserOptions::default(),
        )
            .unwrap();
        let starting_element = HtmlContent::import(dom).unwrap();
        let target_node = starting_element.first_child().unwrap();

        let result = pipeline.run_on(&target_node).unwrap();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn get_attr_querying_root_returns_correct_value() {
        let pipeline = StringValueCreatingPipeline::new(
            ElementSelectingCommand::QueryRoot(CssSelectorList::new(vec![
                CssSelectorPath::single(CssSelector::for_class("test-source")),
            ])),
            ValueExtractingCommand::GetAttribute("data-test"),
        );

        let dom = tl::parse(
            r#"<div id="parent"><div><div><div class="bar" data-test="fubar"></div></div></div><aside class="test-source" data-test="foo"></aside></div>"#,
            tl::ParserOptions::default(),
        )
            .unwrap();
        let starting_element = HtmlContent::import(dom).unwrap();
        let target_node = starting_element
            .first_child()
            .unwrap()
            .first_child()
            .unwrap()
            .first_child()
            .unwrap();

        let mut result = pipeline.run_on(&target_node).unwrap();

        assert_eq!(result.len(), 1);
        let first_result = result.pop().unwrap();
        assert_eq!(first_result, String::from("foo"));
    }

    #[test]
    fn query_root_on_root_queries_itself() {
        let pipeline = StringValueCreatingPipeline::new(
            ElementSelectingCommand::QueryRoot(CssSelectorList::new(vec![
                CssSelectorPath::single(CssSelector::for_class("test-source")),
            ])),
            ValueExtractingCommand::GetAttribute("data-test"),
        );

        let dom = tl::parse(
            r#"<div data-test="fubar" class="bar"><aside class="test-source" data-test="foo"></aside></div>"#,
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
    fn get_attr_returns_empty_for_querying_nonexistent_el_on_root() {
        let pipeline = StringValueCreatingPipeline::new(
            ElementSelectingCommand::QueryRoot(CssSelectorList::new(vec![
                CssSelectorPath::single(CssSelector::for_class("test-source")),
            ])),
            ValueExtractingCommand::GetAttribute("data-test"),
        );

        let dom = tl::parse(
            r#"<div id="parent"><div><div><div class="bar" data-test="fubar"></div></div></div><aside data-test="foo"></aside></div>"#,
            tl::ParserOptions::default(),
        )
            .unwrap();
        let starting_element = HtmlContent::import(dom).unwrap();
        let target_node = starting_element
            .first_child()
            .unwrap()
            .first_child()
            .unwrap()
            .first_child()
            .unwrap();

        let result = pipeline.run_on(&target_node).unwrap();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn get_text_content_from_element_returns_correct_value() {
        let pipeline = StringValueCreatingPipeline::new(
            ElementSelectingCommand::UseElement,
            ValueExtractingCommand::GetTextContent,
        );

        let dom = tl::parse(
            r#"<div data-test="foo" class="bar">The content</div>"#,
            tl::ParserOptions::default(),
        )
        .unwrap();
        let starting_element = HtmlContent::import(dom).unwrap();

        let mut result = pipeline.run_on(&starting_element).unwrap();

        assert_eq!(result.len(), 1);
        let first_result = result.pop().unwrap();
        assert_eq!(first_result, String::from("The content"));
    }

    #[test]
    fn get_text_content_from_empty_input_returns_empty_string() {
        let pipeline = StringValueCreatingPipeline::new(
            ElementSelectingCommand::UseParent,
            ValueExtractingCommand::GetTextContent,
        );

        let dom = tl::parse(
            r#"<div data-test="foo" class="bar">The content</div>"#,
            tl::ParserOptions::default(),
        )
        .unwrap();
        let starting_element = HtmlContent::import(dom).unwrap();

        let result = pipeline.run_on(&starting_element).unwrap();

        assert_eq!(result.len(), 0);
    }
}
