use clap::{Parser, Subcommand};
use rectangle_union_sweep::{Dataset, union_area};
use std::{fs, path::PathBuf, process::ExitCode};

#[derive(Parser)]
#[command(version, about = "Exact rectangle union area using a sweep line")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    Area { input: PathBuf },
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    match Cli::parse().command {
        Command::Area { input } => {
            let dataset: Dataset = serde_json::from_str(&fs::read_to_string(input)?)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&union_area(&dataset.rectangles)?)?
            );
        }
    }
    Ok(())
}
fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
