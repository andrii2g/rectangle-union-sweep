use clap::{Parser, Subcommand};
use rectangle_union_sweep::{Dataset, Pattern, generate, reference_area, union_area};
use std::{fs, path::PathBuf, process::ExitCode};

#[derive(Parser)]
#[command(version, about = "Exact rectangle union area using a sweep line")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    /// Render up to 24 sampled event groups (at most 200 rectangles).
    Render {
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
        #[arg(long, default_value_t = 8)]
        frames: usize,
    },
    Area {
        input: PathBuf,
    },
    Verify {
        input: PathBuf,
    },
    Generate {
        #[arg(long, value_enum, default_value = "overlapping")]
        pattern: Pattern,
        #[arg(long, default_value_t = 20)]
        count: usize,
        #[arg(long, default_value_t = 42)]
        seed: u64,
        #[arg(long)]
        output: Option<PathBuf>,
    },
}
fn read(input: PathBuf) -> Result<Dataset, Box<dyn std::error::Error>> {
    Ok(serde_json::from_str(&fs::read_to_string(input)?)?)
}
fn run() -> Result<(), Box<dyn std::error::Error>> {
    match Cli::parse().command {
        Command::Render {
            input,
            output,
            frames,
        } => {
            let dataset = read(input)?;
            fs::write(
                &output,
                rectangle_union_sweep::render_svg(&dataset.rectangles, frames)?,
            )?;
            println!("Wrote {}", output.display());
        }
        Command::Area { input } => {
            println!(
                "{}",
                serde_json::to_string_pretty(&union_area(&read(input)?.rectangles)?)?
            );
        }
        Command::Verify { input } => {
            let dataset = read(input)?;
            let reference = reference_area(&dataset.rectangles)?;
            let metrics = union_area(&dataset.rectangles)?;
            if metrics.union_area != reference.to_string() {
                return Err(format!(
                    "verification failed: sweep={}, reference={reference}",
                    metrics.union_area
                )
                .into());
            }
            println!(
                "{}",
                serde_json::json!({"verified": true, "union_area": reference.to_string(),
                "rectangles": dataset.rectangles.len()})
            );
        }
        Command::Generate {
            pattern,
            count,
            seed,
            output,
        } => {
            let json = serde_json::to_string_pretty(&generate(pattern, count, seed)?)?;
            if let Some(path) = output {
                fs::write(path, format!("{json}\n"))?;
            } else {
                println!("{json}");
            }
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
