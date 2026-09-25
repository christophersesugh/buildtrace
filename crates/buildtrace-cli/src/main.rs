use std::path::PathBuf;
use std::process::ExitCode;

use buildtrace_cargo::{InventoryError, inventory};
use buildtrace_core::{render_human, render_json};
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(
    name = "cargo-buildtrace",
    version,
    about = "List dependency build scripts and procedural macros without executing them"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// List build-time executables without executing them
    Inventory {
        /// Path to the manifest to inspect (defaults to ./Cargo.toml)
        #[arg(long, default_value = "Cargo.toml")]
        manifest_path: PathBuf,
        /// Output format
        #[arg(long, value_enum, default_value_t = Format::Human)]
        format: Format,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Format {
    Human,
    Json,
}

fn run(cli: Cli) -> Result<(), InventoryError> {
    let Command::Inventory {
        manifest_path,
        format,
    } = cli.command;
    let report = inventory(&manifest_path)?;
    match format {
        Format::Human => print!("{}", render_human(&report)),
        Format::Json => println!("{}", render_json(&report)),
    }
    Ok(())
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::from(2)
        }
    }
}
