extern crate clap;

use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

use html_streaming_editor::{report, HtmlStreamingEditor};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// File name of the Input. `-` for stdin (default)
    #[arg(short, long, value_name = "INPUT")]
    input: Option<PathBuf>,

    /// File name of the Output. `-` for stdout (default)
    #[arg(short, long, value_name = "OUTPUT")]
    output: Option<PathBuf>,

    /// Single string with the command pipeline to perform.
    /// If it starts with an @ the rest is treated as file name
    /// to read the pipeline definition from
    pipeline: String,
}

fn main() {
    pretty_env_logger::init();

    let cli = Cli::parse();

    let input_path = cli.input.unwrap_or_else(|| PathBuf::from("-"));
    let output_path = cli.output.unwrap_or_else(|| PathBuf::from("-"));
    let mut pipeline_definition = cli.pipeline;

    if pipeline_definition.starts_with('@') {
        let filename = pipeline_definition.get(1..).unwrap().to_owned();
        pipeline_definition = String::new();
        if let Ok(mut file) = File::open(filename) {
            if let Err(e) = file.read_to_string(&mut pipeline_definition) {
                eprintln!("[ERROR] Could not read pipeline definition: {}", e);
                std::process::exit(exitcode::NOINPUT);
            }
        } else {
            eprintln!("[ERROR] Could not open pipeline definition");
            std::process::exit(exitcode::NOINPUT);
        };
    }

    let mut input_reader: Box<dyn BufRead> = if input_path.to_str() == Some("-") {
        Box::new(std::io::stdin().lock())
    } else {
        let input_file = if let Ok(file) = File::open(input_path) {
            file
        } else {
            eprintln!("[ERROR] Could not open input file");
            std::process::exit(exitcode::NOINPUT);
        };

        Box::new(BufReader::new(input_file))
    };

    let mut output_writer: Box<dyn Write> = if output_path.to_str() == Some("-") {
        Box::new(std::io::stdout().lock())
    } else {
        let output_file = if let Ok(file) = File::create(output_path) {
            file
        } else {
            eprintln!("[ERROR] Could not open output file");
            std::process::exit(exitcode::CANTCREAT);
        };

        Box::new(BufWriter::new(output_file))
    };

    let editor = HtmlStreamingEditor::new(&mut input_reader, &mut output_writer);
    match editor.run(&pipeline_definition) {
        Ok(_) => (),
        Err(e) => report(&e),
    }
}
