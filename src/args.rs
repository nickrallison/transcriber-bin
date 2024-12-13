use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use clap::{Parser, ValueEnum};
use crate::error::Error;

/// Takes some input, either a file path or a link and returns a transcribed file
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct RawArgs {
    /// Input to transcribe
    #[arg(short, long)]
    pub input: String,

    /// The way the results should be returned to the caller, either print to stdout as json or write to file(s) in a directory
    #[arg(short, long, default_value_t = RawArgsOutput::PrintJson)]
    pub output: RawArgsOutput,

    /// If write is chosen, this is the path to write to, defaults to cwd
    #[arg(short, long, default_value_t = std::env::current_dir().unwrap().to_str().unwrap().to_string())]
    pub write_path: String
}

#[derive(ValueEnum, Debug, Clone)]
enum RawArgsOutput {
    PrintJson,
    Write,
}

impl Display for RawArgsOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RawArgsOutput::PrintJson => write!(f, "print-json"),
            RawArgsOutput::Write => write!(f, "write")
        }
    }
}


pub struct Args {
    pub input: String,
    pub output: ArgsOutput,
}

pub enum ArgsOutput {
    PrintJson,
    Write(PathBuf),
}

pub fn parse_args() -> Result<Args, Error> {
    let raw_args = RawArgs::parse();
    let input = raw_args.input.clone();
    let output = match raw_args.output {
        RawArgsOutput::PrintJson => ArgsOutput::PrintJson,
        RawArgsOutput::Write => {
            let path = PathBuf::from(raw_args.write_path);
            if !path.exists() {
                return Err(Error::OutputDirectoryDoesNotExist(path));
            }
            ArgsOutput::Write(path)
        },
    };
    Ok(Args { input, output })
}