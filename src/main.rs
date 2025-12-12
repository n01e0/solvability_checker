use clap::Parser;
use std::time::Duration;
use rayon::prelude::*;
use std::process::Command;
use walkdir::WalkDir;
use serde_json::json;
use log::*;

// CLI arguments
#[derive(Parser, Debug)]
#[command(
    name = "solvability_checker",
    version,
    author,
    about = "CTF solvability checker"
)]
struct Args {
    /// webhook url
    #[arg(short = 'u', long = "url")]
    webhook: String,

    /// solver files directory
    #[arg(short = 's', long, default_value = "solver")]
    solver: String,

    /// interval between runs (milliseconds)
    #[arg(short = 'i', long, default_value_t = 300_000)]
    interval: u64,

    /// number of retries on failure
    #[arg(short = 'r', long, default_value_t = 5)]
    retries: u32,
}

fn main() {
    // parse CLI args
    let args = Args::parse();
    env_logger::init();
    let solvers = WalkDir::new(&args.solver)
        .into_iter()
        .filter_map(|f| f.ok())
        .filter(|f| f.file_type().is_file())
        .collect::<Vec<_>>();
    println!("Collected solvers.");
    for solver in solvers.clone() {
        println!("{}", solver.path().display());
    }
    let interval = Duration::from_millis(args.interval);

    let retries = args.retries;
    loop {
        solvers.clone().into_par_iter().for_each(|s| {
            let mut succeeded = false;
            for attempt in 1..=retries {
                let status = Command::new(s.path())
                    .env("PWNLIB_NOTERM", "true")
                    .status()
                    .unwrap_or_else(|e| {
                        eprintln!("{}: {}", s.path().display(), e);
                        panic!();
                    });
                if status.success() {
                    info!("{} Success (attempt {}/{})",
                        s.path().display(), attempt, retries);
                    succeeded = true;
                    break;
                } else {
                    warn!("{} failed (attempt {}/{})",
                        s.path().display(), attempt, retries);
                }
            }
            if !succeeded {
                warn!("{} Failure after {} attempts", s.path().display(), retries);
                // send webhook notification
                let webhook_url = &args.webhook;
                ureq::post(webhook_url)
                    .header("Content-Type", "application/json")
                    .send_json(json!({
                        "content": format!(
                            "{} failure after {} retries!",
                            s.path().file_name().unwrap().to_str().unwrap(),
                            retries
                        )
                    }))
                    .unwrap();
            }
        });
        std::thread::sleep(interval);
    }
}
