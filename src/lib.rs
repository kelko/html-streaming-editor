use peg::str::LineCol;
use snafu::{Backtrace, ResultExt, Snafu};
use std::io::{BufRead, Read, Write};

pub use crate::command::Command;
pub(crate) use crate::css::{
    CssAttributeComparison, CssAttributeSelector, CssPseudoClass, CssSelector, CssSelectorList,
    CssSelectorPath, CssSelectorStep,
};
use crate::html::HtmlIndex;
pub use crate::parsing::grammar;
pub use crate::pipeline::Pipeline;

mod command;
mod css;
mod html;
mod parsing;
mod pipeline;

#[derive(Debug, Snafu)]
pub enum StreamingEditorError {
    #[snafu(display("Failed to read input from"))]
    ReadingInputFailed {
        source: std::io::Error,
        backtrace: Backtrace,
    },
    #[snafu(display("Failed to write output into"))]
    WritingOutputFailed {
        source: std::io::Error,
        backtrace: Backtrace,
    },
    #[snafu(display("Failed to parse input HTML"))]
    ParsingInputFailed {
        source: tl::ParseError,
        backtrace: Backtrace,
    },
    #[snafu(display("Failed to parse pipeline"))]
    ParsingPipelineFailed {
        source: peg::error::ParseError<LineCol>,
        backtrace: Backtrace,
    },
    #[snafu(display("Failed to render output"))]
    RenderingOutputFailed {
        #[snafu(backtrace)]
        source: crate::html::IndexError,
    },
    #[snafu(display("Failed to run pipeline"))]
    RunningPipelineFailed {
        #[snafu(backtrace)]
        source: crate::pipeline::PipelineError,
    },
}

pub struct HtmlStreamingEditor {
    input: Box<dyn BufRead>,
    output: Box<dyn Write>,
}

impl HtmlStreamingEditor {
    pub fn new(input: Box<dyn BufRead>, output: Box<dyn Write>) -> Self {
        HtmlStreamingEditor { input, output }
    }

    pub fn run(mut self, commands: String) -> Result<(), StreamingEditorError> {
        let pipeline = parsing::grammar::pipeline(&commands).context(ParsingPipelineFailedSnafu)?;
        let mut string_content = String::new();
        self.input
            .read_to_string(&mut string_content)
            .context(ReadingInputFailedSnafu)?;

        let dom = tl::parse(&string_content, tl::ParserOptions::default())
            .context(ParsingInputFailedSnafu)?;
        let index = HtmlIndex::load(dom);
        let result = pipeline
            .run_on(index.root_elements(), &index)
            .context(RunningPipelineFailedSnafu)?;
        for node in result.iter() {
            let html = index.render(node).context(RenderingOutputFailedSnafu)?;
            self.output
                .write((*html).as_bytes())
                .context(WritingOutputFailedSnafu)?;
        }

        self.output.flush().context(WritingOutputFailedSnafu)?;

        Ok(())
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
