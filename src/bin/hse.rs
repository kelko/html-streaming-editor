extern crate clap;

use clap::clap_app;
use html_streaming_editor::HtmlStreamingEditor;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

fn main() {
    let options = clap_app!(hse =>
        (version: "0.1")
        (author: ":kelko:")
        (about: "Html Streaming Editor")
        (@arg input: -i --input +takes_value "File name of the Input. `-` for stdin (default)")
        (@arg output: -o --output +takes_value "File name of the Output. `-` for stdout (default)")
        (@arg COMMANDS: +required "Single string with the command pipeline to perform")
    )
    .get_matches();

    let input_path = options.value_of("input").unwrap_or("-").to_string();
    let output_path = options.value_of("output").unwrap_or("-").to_string();
    let commands = options
        .value_of("COMMANDS")
        .expect("COMMANDS must be set")
        .to_string();

    let input_reader: Box<dyn BufRead> = if input_path == "-" {
        Box::new(std::io::stdin().lock())
    } else {
        let input_file = if let Ok(file) = File::open(input_path) {
            file
        } else {
            eprintln!("Could not open input file");
            std::process::exit(exitcode::NOINPUT);
        };

        Box::new(BufReader::new(input_file))
    };

    let output_writer: Box<dyn Write> = if output_path == "-" {
        Box::new(std::io::stdout().lock())
    } else {
        let output_file = if let Ok(file) = File::create(output_path) {
            file
        } else {
            eprintln!("Could not open output file");
            std::process::exit(exitcode::CANTCREAT);
        };

        Box::new(BufWriter::new(output_file))
    };

    let editor = HtmlStreamingEditor::new(input_reader, output_writer);
    match editor.run(commands) {
        Ok(()) => (),
        Err(e) => todo!(),
    }
}
