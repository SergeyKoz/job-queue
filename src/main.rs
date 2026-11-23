mod csv_task_analyzer;

use crate::csv_task_analyzer::RowError;
use csv_task_analyzer::parse_tasks;
use std::collections::HashMap;
use std::fs;

fn main() {
    let path = std::env::args().nth(1).expect("No file were provided");
    println!("Reading file: {}", path);
    println!();

    let content = fs::read_to_string(&path).expect("Failed to read file");
    let tasks_stat = parse_tasks(&content);

    render_tasks_stat(tasks_stat);
}

fn render_tasks_stat((task_stat, parse_errors): (HashMap<String, u32>, Vec<RowError>)) {
    for error in &parse_errors {
        println!("line {}: {}", error.row, error.message);
    }

    println!();

    let valid: u32 = task_stat.values().sum();
    let invalid = parse_errors.len();

    println!(
        "total: {}, valid: {}, invalid: {}",
        valid + invalid as u32,
        valid,
        invalid
    );

    println!("by kind:");

    let mut kinds: Vec<&String> = task_stat.keys().collect();
    kinds.sort();

    for kind in kinds {
        println!("  {}: {}", kind, task_stat[kind]);
    }
}
