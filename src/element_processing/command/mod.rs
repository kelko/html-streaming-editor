#[cfg(test)]
mod tests;

use html_escape::{encode_double_quoted_attribute, encode_text};
use log::trace;
use snafu::ResultExt;
use std::fmt::Debug;
use std::ops::Add;

use super::pipeline::ElementProcessingPipeline;
use crate::element_creating::ElementCreatingPipeline;
use crate::html::HtmlContent;
use crate::{CommandError, CssSelectorList, SubpipelineFailedSnafu, ValueSource};

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum ElementProcessingCommand<'a> {
    /// Find all nodes, beginning at the input, that match the given CSS selector and detach them
    /// and return only those
    ExtractElement(CssSelectorList<'a>),
    /// Find all nodes, beginning at the input, that match the given CSS selector
    /// and remove them from their parent nodes.
    /// Returns the input as result.
    RemoveElement(CssSelectorList<'a>),
    /// runs a sub-pipeline on each element matching the given CSS selector
    /// Returns the input as result.
    ForEach(CssSelectorList<'a>, ElementProcessingPipeline<'a>),
    /// runs a sub-pipeline and replaces each element matching the given CSS selector with the result of the pipeline
    /// Returns the input as result.
    ReplaceElement(CssSelectorList<'a>, ElementCreatingPipeline<'a>),
    /// Remove the given attribute from all currently selected nodes
    /// Returns the input as result.
    ClearAttribute(&'a str),
    /// Remove all children of the currently selected nodes
    /// Returns the input as result
    ClearContent,
    /// Add or Reset a given attribute with a new value
    /// Returns the input as result.
    SetAttribute(&'a str, ValueSource<'a>),
    /// Remove all children of the currently selected nodes and add a new text as child instead
    /// Returns the input as result.
    SetTextContent(ValueSource<'a>),
    /// adds a new text as last child
    /// Returns the input as result.
    AppendTextContent(ValueSource<'a>),
    /// adds a new comment as last child
    /// Returns the input as result.
    AppendComment(ValueSource<'a>),
    /// runs a sub-pipeline and adds the result as last child
    /// Returns the input as result.
    AppendElement(ElementCreatingPipeline<'a>),
    /// adds a new text as first child
    /// Returns the input as result.
    PrependTextContent(ValueSource<'a>),
    /// adds a new comment as first child
    /// Returns the input as result.
    PrependComment(ValueSource<'a>),
    /// runs a sub-pipeline and adds the result as first child
    /// Returns the input as result.
    PrependElement(ElementCreatingPipeline<'a>),
}

impl<'a> ElementProcessingCommand<'a> {
    /// perform the action defined by the command on the set of nodes
    /// and return the calculated results.
    /// For some command the output can be equal to the input,
    /// others change the result-set
    pub(crate) fn execute(
        &self,
        input: &Vec<rctree::Node<HtmlContent>>,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        match self {
            ElementProcessingCommand::ForEach(selector, pipeline) => {
                Self::for_each(input, selector, pipeline)
            }
            ElementProcessingCommand::ReplaceElement(selector, pipeline) => {
                Self::replace_element(input, selector, pipeline)
            }
            ElementProcessingCommand::ExtractElement(selector) => {
                Self::extract_element(input, selector)
            }
            ElementProcessingCommand::RemoveElement(selector) => {
                Self::remove_element(input, selector)
            }
            ElementProcessingCommand::ClearAttribute(attribute) => {
                Self::clear_attr(input, attribute)
            }
            ElementProcessingCommand::SetAttribute(attribute, value_source) => {
                Self::set_attr(input, attribute, value_source)
            }
            ElementProcessingCommand::ClearContent => Self::clear_content(input),
            ElementProcessingCommand::SetTextContent(value_source) => {
                Self::set_text_content(input, value_source)
            }
            ElementProcessingCommand::AppendTextContent(value_source) => {
                Self::append_text_content(input, value_source)
            }
            ElementProcessingCommand::AppendComment(value_source) => {
                Self::append_comment(input, value_source)
            }
            ElementProcessingCommand::AppendElement(pipeline) => {
                Self::append_element(input, pipeline)
            }
            ElementProcessingCommand::PrependTextContent(value_source) => {
                Self::prepend_text_content(input, value_source)
            }
            ElementProcessingCommand::PrependComment(value_source) => {
                Self::prepend_comment(input, value_source)
            }
            ElementProcessingCommand::PrependElement(pipeline) => {
                Self::prepend_element(input, pipeline)
            }
        }
    }

    fn for_each(
        input: &[rctree::Node<HtmlContent>],
        selector: &CssSelectorList<'a>,
        pipeline: &ElementProcessingPipeline,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        let queried_elements = selector.query(input);
        let _ = pipeline.run_on(queried_elements);

        Ok(input.to_owned())
    }

    fn extract_element(
        input: &[rctree::Node<HtmlContent>],
        selector: &CssSelectorList<'a>,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!(
            "Running EXTRACT-ELEMENT command using selector: {:#?}",
            selector
        );

        Ok(selector
            .query(input)
            .iter()
            .map(|e| rctree::Node::clone(e).make_deep_copy())
            .collect::<Vec<_>>())
    }

    fn remove_element(
        input: &[rctree::Node<HtmlContent>],
        selector: &CssSelectorList<'a>,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!("Running WITHOUT command using selector: {:#?}", selector);

        let findings = selector.query(input);

        for node in findings {
            node.detach();
        }

        Ok(input.to_owned())
    }

    fn replace_element(
        input: &[rctree::Node<HtmlContent>],
        selector: &CssSelectorList<'a>,
        pipeline: &ElementCreatingPipeline,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!("Running REPLACE command using selector: {:#?}", selector);

        let queried_elements = selector.query(input);

        for element_for_replacement in queried_elements {
            let mut created_elements = pipeline
                .run_on(vec![rctree::Node::clone(&element_for_replacement)])
                .context(SubpipelineFailedSnafu)?;
            for new_element in &mut created_elements {
                let copy = new_element.make_deep_copy();
                element_for_replacement.insert_before(copy);
            }
            element_for_replacement.detach();
        }

        Ok(input.to_owned())
    }

    fn clear_attr(
        input: &Vec<rctree::Node<HtmlContent>>,
        attr_name: &str,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!("Running CLEAR-ATTR command for attr: {:#?}", attr_name);
        let attribute = String::from(attr_name);

        for node in input {
            let working_copy = rctree::Node::clone(node);
            let mut data = working_copy.borrow_mut();
            data.clear_attribute(&attribute);
        }

        Ok(input.clone())
    }

    fn clear_content(
        input: &Vec<rctree::Node<HtmlContent>>,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!("Running CLEAR-CONTENT command");

        for node in input {
            for child in node.children() {
                child.detach()
            }
        }

        Ok(input.clone())
    }

    fn set_attr(
        input: &Vec<rctree::Node<HtmlContent>>,
        attribute: &str,
        value_source: &ValueSource,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!(
            "Running SET-ATTR command for attr: {:#?} with value: {:#?}",
            attribute,
            value_source
        );

        for node in input {
            let rendered_value = value_source.render(node).context(SubpipelineFailedSnafu)?;
            let rendered_value = rendered_value.join("");
            let rendered_value = String::from(encode_double_quoted_attribute(&rendered_value));
            let rendered_value = rendered_value.replace('\n', "\\n");

            let working_copy = rctree::Node::clone(node);
            let mut data = working_copy.borrow_mut();
            data.set_attribute(attribute, rendered_value);
        }

        Ok(input.clone())
    }

    fn set_text_content(
        input: &Vec<rctree::Node<HtmlContent>>,
        value_source: &ValueSource,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!(
            "Running SET-TEXT-CONTENT command with value: {:#?}",
            value_source
        );

        for node in input {
            let rendered_value = value_source.render(node).context(SubpipelineFailedSnafu)?;
            let rendered_value = rendered_value.join("");
            let rendered_value = String::from(encode_text(&rendered_value));

            let working_copy = rctree::Node::clone(node);
            // first clear everything that was there before
            for child in node.children() {
                child.detach()
            }

            working_copy.append(rctree::Node::new(HtmlContent::Text(rendered_value)));
        }

        Ok(input.clone())
    }

    fn append_text_content(
        input: &Vec<rctree::Node<HtmlContent>>,
        value_source: &ValueSource,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!(
            "Running APPEND-TEXT-CONTENT command with value: {:#?}",
            value_source
        );

        for node in input {
            let rendered_value = value_source.render(node).context(SubpipelineFailedSnafu)?;
            let rendered_value = rendered_value.join("");
            let rendered_value = String::from(encode_text(&rendered_value));

            let working_copy = rctree::Node::clone(node);
            working_copy.append(rctree::Node::new(HtmlContent::Text(rendered_value)));
        }

        Ok(input.clone())
    }

    fn append_comment(
        input: &Vec<rctree::Node<HtmlContent>>,
        value_source: &ValueSource,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!(
            "Running APPEND-COMMENT command with value: {:#?}",
            value_source
        );

        for node in input {
            let rendered_value = value_source.render(node).context(SubpipelineFailedSnafu)?;
            let rendered_value = rendered_value.join("");
            let rendered_value = rendered_value.replace("--", "\\x2D\\x2D");

            let working_copy = rctree::Node::clone(node);
            working_copy.append(rctree::Node::new(HtmlContent::Comment(rendered_value)));
        }

        Ok(input.clone())
    }

    fn append_element(
        input: &Vec<rctree::Node<HtmlContent>>,
        pipeline: &ElementCreatingPipeline,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!("Running APPEND-ELEMENT command");

        for node in input {
            if let Some(new_element) = pipeline
                .run_on(vec![])
                .context(SubpipelineFailedSnafu)?
                .pop()
            {
                let working_copy = rctree::Node::clone(node);
                working_copy.append(new_element);
            }
        }

        Ok(input.clone())
    }

    fn prepend_text_content(
        input: &Vec<rctree::Node<HtmlContent>>,
        value_source: &ValueSource,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!(
            "Running APPEND-TEXT-CONTENT command with value: {:#?}",
            value_source
        );

        for node in input {
            let rendered_value = value_source.render(node).context(SubpipelineFailedSnafu)?;
            let rendered_value = rendered_value.join("");
            let rendered_value = String::from(encode_text(&rendered_value));

            let working_copy = rctree::Node::clone(node);
            working_copy.prepend(rctree::Node::new(HtmlContent::Text(rendered_value)));
        }

        Ok(input.clone())
    }

    fn prepend_comment(
        input: &Vec<rctree::Node<HtmlContent>>,
        value_source: &ValueSource,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!(
            "Running APPEND-COMMENT command with value: {:#?}",
            value_source
        );

        for node in input {
            let rendered_value = value_source.render(node).context(SubpipelineFailedSnafu)?;
            let rendered_value = rendered_value.join("");
            let rendered_value = rendered_value.replace("--", "\\x2D\\x2D");

            let working_copy = rctree::Node::clone(node);
            working_copy.prepend(rctree::Node::new(HtmlContent::Comment(rendered_value)));
        }

        Ok(input.clone())
    }

    fn prepend_element(
        input: &Vec<rctree::Node<HtmlContent>>,
        pipeline: &ElementCreatingPipeline,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!("Running APPEND-ELEMENT command");

        for node in input {
            if let Some(new_element) = pipeline
                .run_on(vec![])
                .context(SubpipelineFailedSnafu)?
                .pop()
            {
                let working_copy = rctree::Node::clone(node);
                working_copy.prepend(new_element);
            }
        }

        Ok(input.clone())
    }
}

impl<'a> Add<ElementProcessingCommand<'a>> for ElementProcessingCommand<'a> {
    type Output = Vec<ElementProcessingCommand<'a>>;

    fn add(self, rhs: ElementProcessingCommand<'a>) -> Self::Output {
        vec![self, rhs]
    }
}

impl<'a> Add<Option<Vec<ElementProcessingCommand<'a>>>> for ElementProcessingCommand<'a> {
    type Output = Vec<ElementProcessingCommand<'a>>;

    fn add(self, rhs: Option<Vec<ElementProcessingCommand<'a>>>) -> Self::Output {
        if let Some(mut vec) = rhs {
            vec.insert(0, self);
            return vec;
        }

        vec![self]
    }
}
