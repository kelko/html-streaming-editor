use html_escape::{encode_double_quoted_attribute, encode_text};
use log::trace;
use snafu::{Backtrace, ResultExt, Snafu};
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufReader, Read};
use std::ops::Add;

use crate::html::{HtmlContent, HtmlTag};
use crate::pipeline::PipelineError;
use crate::{CssSelectorList, Pipeline};

#[derive(Debug, Snafu)]
pub enum CommandError {
    #[snafu(display("Failed to remove HTML node"))]
    RemovingNodeFailed {
        #[snafu(backtrace)]
        source: crate::html::IndexError,
    },
    #[snafu(display("Sub-Pipeline failed"))]
    SubpipelineFailed {
        #[snafu(backtrace)]
        #[snafu(source(from(PipelineError, Box::new)))]
        source: Box<PipelineError>,
    },
    #[snafu(display("Failed to read input from"))]
    ReadingInputFailed {
        source: std::io::Error,
        backtrace: Backtrace,
    },
    #[snafu(display("Failed to parse input HTML"))]
    ParsingInputFailed {
        source: tl::ParseError,
        backtrace: Backtrace,
    },
    #[snafu(display("Failed to convert parsed HTML into memory model"))]
    LoadingParsedHtmlFailed {
        #[snafu(backtrace)]
        source: crate::html::StreamingEditorError,
    },
}

/// Is the value directly defined or is it a sub-pipeline?
#[derive(Debug, PartialEq, Clone)]
pub enum ValueSource {
    StringValue(String),
}

impl ValueSource {
    pub(crate) fn render(&self) -> String {
        match self {
            ValueSource::StringValue(value) => value.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Command<'a> {
    /// Find all nodes, beginning at the input, that match the given CSS selector and detach them
    /// and return only those
    ExtractElement(CssSelectorList<'a>),
    /// Find all nodes, beginning at the input, that match the given CSS selector
    /// and remove them from their parent nodes.
    /// Returns the input as result.
    RemoveElement(CssSelectorList<'a>),
    /// runs a sub-pipeline on each element matching the given CSS selector
    /// Returns the input as result.
    ForEach(CssSelectorList<'a>, Pipeline<'a>),
    /// runs a sub-pipeline and replaces each element matching the given CSS selector with the result of the pipeline
    /// Returns the input as result.
    Replace(CssSelectorList<'a>, Pipeline<'a>),
    /// Remove the given attribute from all currently selected nodes
    /// Returns the input as result.
    ClearAttribute(String),
    /// Remove all children of the currently selected nodes
    /// Returns the input as result
    ClearContent,
    /// Add or Reset a given attribute with a new value
    /// Returns the input as result.
    SetAttribute(String, ValueSource),
    /// Remove all children of the currently selected nodes and add a new text as child instead
    /// Returns the input as result.
    SetTextContent(ValueSource),
    /// adds a new text as child
    /// Returns the input as result.
    AddTextContent(ValueSource),
    /// adds a new comment as child
    /// Returns the input as result.
    AddComment(ValueSource),
    /// runs a sub-pipeline and adds the result as child
    /// Returns the input as result.
    AddElement(Pipeline<'a>),
    /// creates an HTML element of given type
    /// Returns the created element as result.
    CreateElement(String),
    /// reads a different file into memory
    /// Returns the content of that file as result.
    FromFile(String),
    /// Starting at the element being replaced run a sub-query
    /// Returns all sub-elements that match the given CSS selector.
    FromReplaced(CssSelectorList<'a>),
}

impl<'a> Command<'a> {
    /// perform the action defined by the command on the set of nodes
    /// and return the calculated results.
    /// For some command the output can be equal to the input,
    /// others change the result-set
    pub(crate) fn execute(
        &self,
        input: &Vec<rctree::Node<HtmlContent>>,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        match self {
            Command::ExtractElement(selector) => Self::extract_element(input, selector),
            Command::RemoveElement(selector) => Self::remove_element(input, selector),
            Command::ClearAttribute(attribute) => Self::clear_attr(input, attribute),
            Command::ClearContent => Self::clear_content(input),
            Command::SetAttribute(attribute, value_source) => {
                Self::set_attr(input, attribute, value_source)
            }
            Command::SetTextContent(value_source) => Self::set_text_content(input, value_source),
            Command::AddTextContent(value_source) => Self::add_text_content(input, value_source),
            Command::AddComment(value_source) => Self::add_comment(input, value_source),
            Command::ForEach(selector, pipeline) => Self::for_each(input, selector, pipeline),
            Command::AddElement(pipeline) => Self::add_element(input, pipeline),
            Command::CreateElement(element_name) => Self::create_element(element_name),
            Command::Replace(selector, pipeline) => Self::replace(input, selector, pipeline),
            Command::FromFile(file_path) => Self::from_file(file_path),
            Command::FromReplaced(selector) => Self::from_replaced(input, selector),
        }
    }

    fn for_each(
        input: &Vec<rctree::Node<HtmlContent>>,
        selector: &CssSelectorList<'a>,
        pipeline: &Pipeline,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        let queried_elements = selector.query(input);
        let _ = pipeline.run_on(queried_elements);

        Ok(input.clone())
    }

    fn extract_element(
        input: &Vec<rctree::Node<HtmlContent>>,
        selector: &CssSelectorList<'a>,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!(
            "Running EXTRACT-ELEMENT command using selector: {:#?}",
            selector
        );

        let mut matching_elements = selector.query(input);

        for element in &mut matching_elements {
            element.detach()
        }

        Ok(matching_elements)
    }

    fn remove_element(
        input: &Vec<rctree::Node<HtmlContent>>,
        selector: &CssSelectorList<'a>,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!("Running WITHOUT command using selector: {:#?}", selector);

        let findings = selector.query(input);

        for mut node in findings {
            node.detach();
        }

        Ok(input.clone())
    }

    fn replace(
        input: &Vec<rctree::Node<HtmlContent>>,
        selector: &CssSelectorList<'a>,
        pipeline: &Pipeline,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!("Running REPLACE command using selector: {:#?}", selector);

        let queried_elements = selector.query(input);

        for mut element_for_replacement in queried_elements {
            let mut created_elements = pipeline
                .run_on(vec![rctree::Node::clone(&element_for_replacement)])
                .context(SubpipelineFailedSnafu)?;
            for new_element in &mut created_elements {
                let copy = new_element.make_deep_copy();
                element_for_replacement.insert_before(copy);
            }
            element_for_replacement.detach();
        }

        Ok(input.clone())
    }

    fn clear_attr(
        input: &Vec<rctree::Node<HtmlContent>>,
        attribute: &String,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!("Running CLEAR-ATTR command for attr: {:#?}", attribute);

        for node in input {
            let mut working_copy = rctree::Node::clone(node);
            let mut data = working_copy.borrow_mut();
            data.clear_attribute(attribute);
        }

        Ok(input.clone())
    }

    fn clear_content(
        input: &Vec<rctree::Node<HtmlContent>>,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!("Running CLEAR-CONTENT command");

        for node in input {
            for mut child in node.children() {
                child.detach()
            }
        }

        Ok(input.clone())
    }

    fn set_attr(
        input: &Vec<rctree::Node<HtmlContent>>,
        attribute: &String,
        value_source: &ValueSource,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!(
            "Running SET-ATTR command for attr: {:#?} with value: {:#?}",
            attribute,
            value_source
        );

        for node in input {
            let mut working_copy = rctree::Node::clone(node);
            let mut data = working_copy.borrow_mut();
            let rendered_value = value_source.render();
            let rendered_value = String::from(encode_double_quoted_attribute(&rendered_value));
            let rendered_value = rendered_value.replace("\n", "\\n");
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
            // first clear everything that was there before
            for mut child in node.children() {
                child.detach()
            }

            let mut working_copy = rctree::Node::clone(node);
            let rendered_value = value_source.render();
            let rendered_value = String::from(encode_text(&rendered_value));
            working_copy.append(rctree::Node::new(HtmlContent::Text(rendered_value)));
        }

        Ok(input.clone())
    }

    fn add_text_content(
        input: &Vec<rctree::Node<HtmlContent>>,
        value_source: &ValueSource,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!(
            "Running ADD-TEXT-CONTENT command with value: {:#?}",
            value_source
        );

        for node in input {
            let mut working_copy = rctree::Node::clone(node);
            let rendered_value = value_source.render();
            let rendered_value = String::from(encode_text(&rendered_value));
            working_copy.append(rctree::Node::new(HtmlContent::Text(rendered_value)));
        }

        Ok(input.clone())
    }

    fn add_comment(
        input: &Vec<rctree::Node<HtmlContent>>,
        value_source: &ValueSource,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!(
            "Running ADD-COMMENT command with value: {:#?}",
            value_source
        );

        for node in input {
            let mut working_copy = rctree::Node::clone(node);
            let rendered_value = value_source.render();
            let rendered_value = rendered_value.replace("--", "\\x2D\\x2D");
            working_copy.append(rctree::Node::new(HtmlContent::Comment(rendered_value)));
        }

        Ok(input.clone())
    }

    fn add_element(
        input: &Vec<rctree::Node<HtmlContent>>,
        pipeline: &Pipeline,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!("Running ADD-ELEMENT command");

        for node in input {
            if let Some(new_element) = pipeline
                .run_on(vec![])
                .context(SubpipelineFailedSnafu)?
                .pop()
            {
                let mut working_copy = rctree::Node::clone(node);
                working_copy.append(new_element);
            }
        }

        Ok(input.clone())
    }

    fn create_element(name: &String) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!("Running CREATE-ELEMENT command using name: {:#?}", name);

        Ok(vec![rctree::Node::new(HtmlContent::Tag(HtmlTag::of_name(
            name.clone(),
        )))])
    }

    fn from_file(file_path: &String) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!("Running FROM-FILE command using file: {:#?}", file_path);

        let file = File::open(file_path).context(ReadingInputFailedSnafu)?;
        let mut buffered_reader = BufReader::new(file);

        let mut string_content = String::new();
        buffered_reader
            .read_to_string(&mut string_content)
            .context(ReadingInputFailedSnafu)?;

        let dom = tl::parse(&string_content, tl::ParserOptions::default())
            .context(ParsingInputFailedSnafu)?;
        let mut root_element = HtmlContent::import(dom).context(LoadingParsedHtmlFailedSnafu)?;

        Ok(vec![root_element.make_deep_copy()])
    }

    fn from_replaced(
        input: &Vec<rctree::Node<HtmlContent>>,
        selector: &CssSelectorList<'a>,
    ) -> Result<Vec<rctree::Node<HtmlContent>>, CommandError> {
        trace!("Running FROM-REPLACED command");

        Self::extract_element(input, selector)
    }
}

impl<'a> Add<Command<'a>> for Command<'a> {
    type Output = Vec<Command<'a>>;

    fn add(self, rhs: Command<'a>) -> Self::Output {
        vec![self, rhs]
    }
}

impl<'a> Add<Option<Vec<Command<'a>>>> for Command<'a> {
    type Output = Vec<Command<'a>>;

    fn add(self, rhs: Option<Vec<Command<'a>>>) -> Self::Output {
        if let Some(mut vec) = rhs {
            vec.insert(0, self);
            return vec;
        }

        vec![self]
    }
}
