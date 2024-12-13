use std::io::Write;

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
                        let file_name = transcribed.filename_with_extension();
                        let file_path = dir.join(file_name);
                        let file = std::fs::File::create(file_path)?;
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