use crate::element_processing::command::ElementProcessingCommand;
use log::{trace, warn};
use snafu::ResultExt;
use std::fmt::Debug;

use crate::html::HtmlContent;
use crate::{CommandFailedSnafu, PipelineError};

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
        for (command_index, command) in self.0.iter().enumerate() {
            trace!("Running Next: {:#?}", &command);
            trace!("Current Element Set: {:#?}", &intermediate);

            intermediate = command.execute(&intermediate).context(CommandFailedSnafu {
                index: command_index,
            })?;

            if intermediate.is_empty() {
                warn!("Command resulted in an empty result set");
            }
        }

        Ok(intermediate)
    }
}

#[cfg(test)]
mod tests {
    use crate::html::HtmlRenderable;
    use crate::{
        element_processing::{
            command::ElementProcessingCommand, pipeline::ElementProcessingPipeline,
        },
        CssSelector, CssSelectorList, CssSelectorPath, CssSelectorStep, HtmlContent,
    };

    const TEST_HTML_DOCUMENT: &str = r#"<html>
    <head></head>
    <body>
        <h1>Title</h1>
        <p id="first-para">Some <em class="fancy">first</em> text</p>
        <p id="second-para">Some more text, even with an <img src=""></p>
        <p id="third-para">Third text of <abbr>HTML</abbr>, but no <abbr>CSS</abbr></p>
        <ul id="list">
            <li id="item-1">1</li>
            <li id="item-2">2</li>
            <li id="item-3">3</li>
        </ul>
    </body>
</html>"#;

    #[test]
    fn run_on_single_command() {
        let pipeline =
            ElementProcessingPipeline::new(vec![ElementProcessingCommand::ExtractElement(
                CssSelectorList::new(vec![CssSelectorPath::new(
                    CssSelector::for_element("h1"),
                    vec![CssSelectorStep::adjacent_sibling(CssSelector::for_element(
                        "p",
                    ))],
                )]),
            )]);

        let dom = tl::parse(TEST_HTML_DOCUMENT, tl::ParserOptions::default()).unwrap();
        let starting_elements = HtmlContent::import(dom).unwrap();

        let mut result = pipeline
            .run_on(vec![rctree::Node::clone(&starting_elements)])
            .unwrap();

        assert_eq!(result.len(), 1);
        let first_result = result.pop().unwrap();
        assert_eq!(
            first_result.outer_html(),
            String::from(r#"<p id="first-para">Some <em class="fancy">first</em> text</p>"#)
        );
    }

    #[test]
    fn run_on_two_commands() {
        let pipeline = ElementProcessingPipeline::new(vec![
            ElementProcessingCommand::ExtractElement(CssSelectorList::new(vec![
                CssSelectorPath::new(
                    CssSelector::for_element("h1"),
                    vec![CssSelectorStep::adjacent_sibling(CssSelector::for_element(
                        "p",
                    ))],
                ),
            ])),
            ElementProcessingCommand::RemoveElement(CssSelectorList::new(vec![
                CssSelectorPath::single(CssSelector::for_element("em")),
            ])),
        ]);

        let dom = tl::parse(TEST_HTML_DOCUMENT, tl::ParserOptions::default()).unwrap();
        let starting_elements = HtmlContent::import(dom).unwrap();
        let mut result = pipeline.run_on(vec![starting_elements]).unwrap();

        assert_eq!(result.len(), 1);
        let first_result = result.pop().unwrap();
        assert_eq!(
            first_result.outer_html(),
            String::from(r#"<p id="first-para">Some  text</p>"#)
        );
    }
}
