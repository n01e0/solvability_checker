#[macro_use]
extern crate clap;
use clap::App;
use std::process::Command;
use walkdir::WalkDir;
use rayon::prelude::*;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let solvers = WalkDir::new(matches.value_of("solver").unwrap()).into_iter().filter_map(|f| f.ok()).filter(|f| f.file_type().is_file()).collect::<Vec<_>>();
    let interval = std::time::Duration::from_millis(matches.value_of("interval").unwrap_or("3000").parse().unwrap_or(3000));

    loop {
        solvers.clone().into_par_iter().for_each(|s| {
            if !Command::new(s.path()).env("PWNLIB_NOTERM", "true").status().unwrap_or_else(|e|{ eprintln!("{}: {}", s.path().display(), e); panic!();}).success() {
                ureq::post(matches.value_of("webhook").unwrap())
                    .set("Content-Type", "application/json")
                    .send_json(ureq::json!({
                        "content": format!("{} failure!", s.path().file_name().unwrap().to_str().unwrap())
                    })).unwrap();
            }
        });
        std::thread::sleep(interval);
    }
}
