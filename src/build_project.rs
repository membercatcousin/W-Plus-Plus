
// W++ Compiler (REWRITEN IN RUST)
// made by membercatcousin
// license: GPL v3


use std::fs;
use std::process::Command;
use serde::Deserialize;

#[derive(Deserialize)]
struct Mapping {
    w: String,
    rs: String,
}

#[derive(Deserialize)]
struct ParseInfo {
    mappings: Vec<Mapping>,
}

pub fn build(name: &str) {
    println!("Building project {}...", name);

    // Load parse_info.json
    let parse_info_content = fs::read_to_string("parse_info.json").expect("Failed to read parse_info.json");
    let parse_info: ParseInfo = serde_json::from_str(&parse_info_content).expect("Failed to parse parse_info.json");

    // Read the wpp file
    let input_file = format!("src/main.wpp");
    let mut content = fs::read_to_string(&input_file).expect("Failed to read input file");

    // Apply replacements
    for mapping in &parse_info.mappings {
        content = content.replace(&mapping.w, &mapping.rs);
    }

    // Write to temp_output.rs
    fs::write("temp_output.rs", &content).expect("Failed to write temp_output.rs");

    // Compile
    println!("[INFO] Building Native Binary...");
    let output = Command::new("rustc")
        .arg("temp_output.rs")
        .arg("-o")
        .arg(name)
        .status()
        .expect("Failed to run rustc, make sure you have Rust installed and added to your PATH");

    if output.success() {
        println!("Success! Run your program with: ./{}", name);
    } else {
        println!("Error: Failed to compile. Check your W++ syntax!");
    }
}