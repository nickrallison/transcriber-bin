use std::io::Write;
use regex::Regex;
use crate::args::ArgsOutput;
use crate::error::Error;

mod args;
mod error;

fn main() -> Result<(), Error> {
    let args = args::parse_args()?;
    let transcribed_files = transcriber::transcribe(&args.input)?;
    match args.output {
        ArgsOutput::PrintJson => {
            for transcribed_res in transcribed_files {
                match transcribed_res {
                    Ok(transcribed) => println!("{}", serde_json::to_string(&transcribed)?),
                    Err(err) => eprintln!("Couldn't transcribe: {}", err),
                }
            }
        }
        ArgsOutput::Write(dir) => {
            for transcribed_res in transcribed_files {
                match transcribed_res {
                    Ok(transcribed) => {
                        let filename = transcribed.filename_with_extension();
                        let filename = filename.to_string_lossy();
                        let filename = filename.as_ref().to_string();
                        let filename: String = transcriber::util::clean_filename(filename);

                        let filepath = dir.join(filename);
                        println!("Writing file: {}", filepath.display());
                        let file = std::fs::File::create(filepath)?;
                        let mut writer = std::io::BufWriter::new(file);
                        writer.write_all(transcribed.contents().as_ref())?;
                    },
                    Err(err) => eprintln!("Couldn't transcribe: {}", err),
                }
            }
        }
    }

    Ok(())
}