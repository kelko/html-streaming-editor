use std::io::{BufRead, Read, Write};

mod command;
mod css;
mod html;
mod parsing;
mod pipeline;

pub use crate::command::Command;
pub(crate) use crate::css::{
    CssAttributeComparison, CssAttributeSelector, CssPseudoClass, CssSelector, CssSelectorList,
    CssSelectorPath, CssSelectorStep,
};
use crate::html::HtmlIndex;
pub use crate::parsing::grammar;
pub use crate::pipeline::Pipeline;

pub struct HtmlStreamingEditor {
    input: Box<dyn BufRead>,
    output: Box<dyn Write>,
}

impl HtmlStreamingEditor {
    pub fn new(input: Box<dyn BufRead>, output: Box<dyn Write>) -> Self {
        HtmlStreamingEditor { input, output }
    }

    pub fn run(mut self, commands: String) -> Result<(), ()> {
        if let Ok(pipeline) = parsing::grammar::pipeline(&commands) {
            let mut string_content = String::new();
            if let Err(_) = self.input.read_to_string(&mut string_content) {
                todo!()
            }

            if let Ok(dom) = tl::parse(&string_content, tl::ParserOptions::default()) {
                let index = HtmlIndex::load(dom);
                if let Ok(result) = pipeline.run_on(index.root_elements(), &index) {
                    for node in result.iter() {
                        let html = index.render(node);
                        match self.output.write((*html).as_bytes()) {
                            Ok(_) => (),
                            Err(_) => todo!(),
                        }
                    }
                }
            } else {
                todo!()
            }

            if let Err(_) = self.output.flush() {
                eprintln!("Could not flush output. File might not contain all content");
                std::process::exit(exitcode::IOERR);
            }
        } else {
            todo!()
        }

        Ok(())
    }
}
