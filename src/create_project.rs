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

    let yaml_file_path = format!("{}/project.yml", name);
    let yaml_content = r##"version: 0.1.0
authors: ["Your Name"] 
"##;

    if let Err(e) = fs::write(&yaml_file_path, yaml_content) {
        println!("Failed to create file {}: {}", yaml_file_path, e);
        return;
    }

    println!("Project {} created successfully!", name);
}
