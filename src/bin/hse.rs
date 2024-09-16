extern crate clap;

use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Error, Read, Write};
use std::path::PathBuf;

use html_streaming_editor::{report, HtmlRenderable, HtmlStreamingEditor};

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
        pipeline_definition = read_pipeline_from_file(&pipeline_definition);
    }

    let mut input_reader = open_input(input_path);
    let editor = HtmlStreamingEditor::new(&mut input_reader);
    match editor.run(&pipeline_definition) {
        Ok(result) => {
            let mut output_writer = open_output(output_path);
            if let Err(e) = render_result(&result, &mut output_writer) {
                eprintln!("[ERROR] {}", e);
            }
        }
        Err(e) => report(&e),
    }
}

fn render_result(
    result: &Vec<Box<dyn HtmlRenderable>>,
    output_writer: &mut Box<dyn Write>,
) -> Result<(), Error> {
    for node in result {
        let html = node.outer_html();
        match output_writer.write((*html).as_bytes()) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        match output_writer.write_all(b"\n") {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
    }

    output_writer.flush()?;

    Ok(())
}

fn open_output(output_path: PathBuf) -> Box<dyn Write> {
    let output_writer: Box<dyn Write> = if output_path.to_str() == Some("-") {
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
    output_writer
}

fn open_input(input_path: PathBuf) -> Box<dyn BufRead> {
    let input_reader: Box<dyn BufRead> = if input_path.to_str() == Some("-") {
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
    input_reader
}

fn read_pipeline_from_file(file_definition: &str) -> String {
    let filename = file_definition.get(1..).unwrap().to_owned();
    let mut pipeline_definition = String::new();
    if let Ok(mut file) = File::open(filename) {
        if let Err(e) = file.read_to_string(&mut pipeline_definition) {
            eprintln!("[ERROR] Could not read pipeline definition: {}", e);
            std::process::exit(exitcode::NOINPUT);
        }
    } else {
        eprintln!("[ERROR] Could not open pipeline definition");
        std::process::exit(exitcode::NOINPUT);
    };

    pipeline_definition
}
