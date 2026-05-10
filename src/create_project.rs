use std::fs;

pub fn create(name: &str, _project_type: &str) {
    // Create the directory structure
    let dir_path = format!("{}/src", name);
    if let Err(e) = fs::create_dir_all(&dir_path) {
        println!("Failed to create directory {}: {}", dir_path, e);
        return;
    }

    // Create the main.wpp file with the specified content
    let file_path = format!("{}/src/main.wpp", name);
    let content = r#"func main() {
    print("hello world");
}"#;

    if let Err(e) = fs::write(&file_path, content) {
        println!("Failed to create file {}: {}", file_path, e);
        return;
    }

    // Create the parse_info.json file with the specified content
    let json_file_path = format!("{}/parse_info.json", name);
    let json_content = r##"{
    "mappings": [
        { "w": "func ",      "rs": "fn " },
        { "w": "print(",     "rs": "println!("},
        { "w": "pass(",      "rs": "std::process::exit(" },
        { "w": "//",          "rs": "//" },
        { "w": "let ",       "rs": "let mut "},
        { "w": "if ",        "rs": "if " },
        { "w": "elif ",      "rs": "else if " },
        { "w": "else",       "rs": "else" },
        { "w": "while ",     "rs": "while " },
        { "w": "for ",       "rs": "for " },
        { "w": "loop ",      "rs": "loop " },
        { "w": "stop",       "rs": "break" },
        { "w": "true",       "rs": "true" },
        { "w": "false",      "rs": "false" }
    ]
}"##;

    let json_file_path = format!("{}/project.yml", name);
    let json_content = r#"name:{}
version: 0.1.0
authors: ["Your Name"] 
"#, name;

    if let Err(e) = fs::write(&json_file_path, json_content) {
        println!("Failed to create file {}: {}", json_file_path, e);
        return;
    }

    println!("Project {} created successfully!", name);
}
