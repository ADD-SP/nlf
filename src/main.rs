mod msg;

use clap::Parser;
use msg::Message;
use std::{
    fs::File,
    io::{Read, Seek, Write},
    path::PathBuf,
    process::ExitCode,
};

const HELP_TEMPLATE: &str = concat!(
    r#"
{before-help}
{name} {version}

{about}

{usage-heading} {usage}

{all-args}{after-help}

Examples:
{tab} # Append a newline to all .txt files under the `dir` directory if they don't already end with one
{tab} find dir -type f -name '*.txt' -exec nlf {} \;

Author:
{tab}{author}

Version:
{tab}{version}

Repository:
{tab}"#,
    env!("CARGO_PKG_REPOSITORY")
);

#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
#[command(help_template = HELP_TEMPLATE)]
pub struct Args {
    file: PathBuf,

    #[clap(long, hide(true))]
    otaku: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();
    let otaku = args.otaku;

    match File::options().read(true).write(true).open(&args.file) {
        Ok(mut file) => {
            let metadata = match file.metadata() {
                Ok(metadata) => metadata,
                Err(e) => {
                    eprintln!(
                        "{}: {}: {}",
                        env!("CARGO_PKG_NAME"),
                        args.file.display(),
                        Message::io_error(e, otaku)
                    );
                    return ExitCode::FAILURE;
                }
            };

            if !metadata.is_file() {
                eprintln!(
                    "{}: {}: {}",
                    env!("CARGO_PKG_NAME"),
                    args.file.display(),
                    Message::not_a_regular_file(otaku),
                );
                return ExitCode::FAILURE;
            }

            let mut raw_content = Vec::new();
            if let Err(e) = file.read_to_end(&mut raw_content) {
                eprintln!(
                    "{}: {}: {}",
                    env!("CARGO_PKG_NAME"),
                    args.file.display(),
                    Message::io_error(e, otaku)
                );
                return ExitCode::FAILURE;
            }

            if raw_content.is_empty() {
                return ExitCode::SUCCESS;
            }

            let content = match String::from_utf8(raw_content) {
                Ok(content) => content,
                Err(_) => {
                    eprintln!(
                        "{}: {}: {}",
                        env!("CARGO_PKG_NAME"),
                        args.file.display(),
                        Message::non_utf8_content(otaku),
                    );
                    return ExitCode::FAILURE;
                }
            };

            if content.contains("\r\n") {
                eprintln!(
                    "{}: {}: {}",
                    env!("CARGO_PKG_NAME"),
                    args.file.display(),
                    Message::file_contains_crlf_line_endings(otaku),
                );
                return ExitCode::FAILURE;
            }

            if content.ends_with('\n') {
                return ExitCode::SUCCESS;
            }

            match file.seek(std::io::SeekFrom::End(0)) {
                Ok(_) => match file.write_all(b"\n") {
                    Ok(_) => ExitCode::SUCCESS,
                    Err(e) => {
                        eprintln!(
                            "{}: {}: {}",
                            env!("CARGO_PKG_NAME"),
                            args.file.display(),
                            Message::io_error(e, otaku)
                        );
                        ExitCode::FAILURE
                    }
                },
                Err(e) => {
                    eprintln!(
                        "{}: {}: {}",
                        env!("CARGO_PKG_NAME"),
                        args.file.display(),
                        Message::io_error(e, otaku)
                    );
                    ExitCode::FAILURE
                }
            }
        }
        Err(e) => {
            eprintln!(
                "{}: {}: {}",
                env!("CARGO_PKG_NAME"),
                args.file.display(),
                Message::io_error(e, otaku)
            );
            ExitCode::FAILURE
        }
    }
}
