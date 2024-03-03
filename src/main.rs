mod app;
mod cli;
mod converter;
mod error;
mod reader;
mod writer;

use std::process::ExitCode;

use clap::Parser;
use console::style;

use cli::Cli;

#[cfg(not(tarpaulin_include))]
fn main() -> ExitCode {
    let args = Cli::parse();

    if let Err(err) = app::run(args) {
        eprintln!("{}", style(format!("✗ Error: {}", err)).red().bold());
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
