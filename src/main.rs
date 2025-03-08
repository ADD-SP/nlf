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
{name}

{about}

{usage-heading} {usage}

{all-args}{after-help}

Examples:
{tab}# Append a newline to all .txt files under the `dir` directory if they don't already end with one
{tab}find dir -type f -name '*.txt' -exec nlf {} \;

Exit Codes:
{tab}0: File has been fixed or already ends with a newline character
{tab}1: Error occurred
{tab}3: File doesn't end with a newline character (when using --check)

Author:
{tab}{author}

Version:
{tab}{version}

Repository:
{tab}"#,
    env!("CARGO_PKG_REPOSITORY")
);

const EXITCODE_PLEASE_FIX: u8 = 3;

#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
#[command(help_template = HELP_TEMPLATE)]
pub struct Args {
    /// The file (not directory) to process.
    file: PathBuf,

    #[clap(short, long, default_value = "false")]
    /// Only check if the file ends with a newline character
    /// without modifying it.
    ///
    /// Exit code 0: file ends with a newline character.
    ///
    /// Exit code 1: error occurred.
    ///
    /// Exit code 3: file doesn't end with a newline character.
    check: bool,

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

            if args.check {
                eprintln!(
                    "{}: {}: {}",
                    env!("CARGO_PKG_NAME"),
                    args.file.display(),
                    Message::please_fix(otaku),
                );
                return ExitCode::from(EXITCODE_PLEASE_FIX);
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
