use std::{fs, path::PathBuf};

use eclipse_heart::data::GameContent;
use eclipse_heart::engine::SimulationRunner;

fn main() {
    let round_cap = parse_round_cap(std::env::args().skip(1)).unwrap_or(30);
    let content = GameContent::load().expect("failed to load game content");
    let report = SimulationRunner::run_starter_series(&content, round_cap);
    let serialized =
        serde_json::to_string_pretty(&report).expect("failed to serialize simulation report");
    let output_path = simulation_output_path();

    fs::write(&output_path, &serialized).expect("failed to write simulation report");

    println!("{serialized}");
    eprintln!("saved simulation report to {}", output_path.display());
}

fn parse_round_cap(args: impl Iterator<Item = String>) -> Option<u32> {
    let values = args.collect::<Vec<_>>();
    let mut index = 0;

    while index < values.len() {
        if values[index] == "--round-cap" {
            let value = values.get(index + 1)?;
            return value.parse::<u32>().ok();
        }
        index += 1;
    }

    None
}

fn simulation_output_path() -> PathBuf {
    std::env::current_dir()
        .expect("current directory is available")
        .join("simulation_report.json")
}
