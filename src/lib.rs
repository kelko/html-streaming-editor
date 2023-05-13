use log::debug;
use peg::str::LineCol;
use snafu::{Backtrace, ResultExt, Snafu};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

pub(crate) use crate::css::{
    CssAttributeComparison, CssAttributeSelector, CssPseudoClass, CssSelector, CssSelectorList,
    CssSelectorPath, CssSelectorStep,
};
use crate::html::HtmlContent;
use crate::string_creating::StringValueCreatingPipeline;

pub use crate::html::HtmlRenderable;

mod css;
mod element_creating;
mod element_processing;
mod html;
mod parsing;
mod string_creating;

#[derive(Debug, Snafu)]
pub enum StreamingEditorError {
    #[snafu(display("Failed to read input from"))]
    ReadingInputFailed {
        source: std::io::Error,
        backtrace: Backtrace,
    },
    #[snafu(display("Failed to write output into"))]
    WritingOutputFailed { source: std::io::Error },
    #[snafu(display("Failed to parse input HTML"))]
    ParsingInputFailed {
        source: tl::ParseError,
        backtrace: Backtrace,
    },
    #[snafu(display("Failed to convert parsed HTML into memory model"))]
    LoadingParsedHtmlFailed {
        #[snafu(backtrace)]
        source: html::HtmlDomError,
    },
    #[snafu(display("Failed to parse pipeline"))]
    ParsingPipelineFailed {
        source: peg::error::ParseError<LineCol>,
        backtrace: Backtrace,
    },
    #[snafu(display("Failed to run pipeline"))]
    RunningPipelineFailed {
        #[snafu(backtrace)]
        source: PipelineError,
    },
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum PipelineError {
    #[snafu(display("Command at index {index} failed"))]
    CommandFailed {
        index: usize,
        #[snafu(backtrace)]
        source: CommandError,
    },
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum CommandError {
    #[snafu(display("Sub-Pipeline failed"))]
    SubpipelineFailed {
        #[snafu(backtrace)]
        #[snafu(source(from(PipelineError, Box::new)))]
        source: Box<PipelineError>,
    },
    #[snafu(display("Failed to read input from"))]
    ReadingCommandInputFailed {
        source: std::io::Error,
        backtrace: Backtrace,
    },
    #[snafu(display("Failed to parse input HTML"))]
    ParsingCommandInputFailed {
        source: tl::ParseError,
        backtrace: Backtrace,
    },
    #[snafu(display("Failed to convert parsed HTML into memory model"))]
    LoadingParsedCommandHtmlFailed {
        #[snafu(backtrace)]
        source: crate::html::HtmlDomError,
    },
    #[snafu(display("Failed to parse regular expression"))]
    ParsingRegexFailed {
        source: regex::Error,
        backtrace: Backtrace,
    },
}

pub struct HtmlStreamingEditor<'a> {
    input: &'a mut dyn BufRead,
}

impl<'a> HtmlStreamingEditor<'a> {
    pub fn new(input: &'a mut dyn BufRead) -> Self {
        HtmlStreamingEditor { input }
    }

    pub fn run(
        self,
        pipeline_definition: &str,
    ) -> Result<Vec<Box<dyn HtmlRenderable>>, StreamingEditorError> {
        let pipeline =
            parsing::grammar::pipeline(pipeline_definition).context(ParsingPipelineFailedSnafu)?;
        debug!("Parsed Pipeline: {:#?}", &pipeline);

        let mut string_content = String::new();
        self.input
            .read_to_string(&mut string_content)
            .context(ReadingInputFailedSnafu)?;

        let dom = tl::parse(&string_content, tl::ParserOptions::default())
            .context(ParsingInputFailedSnafu)?;
        let root_element = HtmlContent::import(dom).context(LoadingParsedHtmlFailedSnafu)?;
        let result = pipeline
            .run_on(vec![root_element])
            .context(RunningPipelineFailedSnafu)?;

        debug!("Final Result: {:#?}", &result);
        Ok(result
            .iter()
            .map(|n| Box::new(n.clone()) as Box<dyn HtmlRenderable>)
            .collect::<Vec<_>>())
    }
}

pub fn report<E: 'static>(err: &E)
where
    E: std::error::Error,
    E: snafu::ErrorCompat,
    E: Send + Sync,
{
    eprintln!("[ERROR] {}", err);
    if let Some(source) = err.source() {
        eprintln!();
        eprintln!("Caused by:");
        for (i, e) in std::iter::successors(Some(source), |e| e.source()).enumerate() {
            eprintln!("   {}: {}", i, e);
        }
    }

    if let Some(backtrace) = snafu::ErrorCompat::backtrace(err) {
        eprintln!("Backtrace:");
        eprintln!("{}", backtrace);
    }
}

/// Is the value directly defined or is it a sub-pipeline?
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum ValueSource<'a> {
    StringValue(&'a str),
    SubPipeline(StringValueCreatingPipeline<'a>),
}

impl<'a> ValueSource<'a> {
    pub fn render(
        &self,
        element: &rctree::Node<HtmlContent>,
    ) -> Result<Vec<String>, PipelineError> {
        match self {
            ValueSource::StringValue(value) => Ok(vec![String::from(*value)]),
            ValueSource::SubPipeline(pipeline) => pipeline.run_on(element),
        }
    }
}

pub(crate) fn load_html_file(file_path: &str) -> Result<rctree::Node<HtmlContent>, CommandError> {
    let file = File::open(file_path).context(ReadingCommandInputFailedSnafu)?;
    let mut buffered_reader = BufReader::new(file);

    let mut string_content = String::new();
    buffered_reader
        .read_to_string(&mut string_content)
        .context(ReadingCommandInputFailedSnafu)?;

    let dom = tl::parse(&string_content, tl::ParserOptions::default())
        .context(ParsingCommandInputFailedSnafu)?;

    HtmlContent::import(dom).context(LoadingParsedCommandHtmlFailedSnafu)
}

#[cfg(test)]
pub(crate) fn load_inline_html(html: &str) -> rctree::Node<HtmlContent> {
    let dom = tl::parse(html, tl::ParserOptions::default()).unwrap();

    HtmlContent::import(dom).unwrap()
}
