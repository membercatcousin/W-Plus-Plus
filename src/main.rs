use std::env;

mod create_project;
mod build_project;

// this is the main function that parses the cli arguments you gave it and then calls the appropriate function

fn main() {
    // argument are stored here to be used later
    let mut args = env::args().skip(1);

    let arg1 = args.next().unwrap_or_default();
    let arg2 = args.next().unwrap_or_default();
    let arg3 = args.next().unwrap_or_default();
    behaviour_decider(arg1, arg2, arg3);
}

fn behaviour_decider(arg1: String, arg2: String, arg3: String) {
    // NOW HERE IS WHERE WE USE THE ARGUMENTS
    if arg1 == "new" {
        if arg2.is_empty() {
            println!("Please provide a name for your project.");
            return;
        } else {
            if arg3 != "--bin" && arg3 != "--lib" && arg3 != "-gooning" {
                println!("Please provide a valid type for your project. ( --bin or --lib )");
                return;
            } else if arg3 == "--gooning"{
                println!("YOU GOONER! Still, invalid project type. Please provide a valid type for your project. ( --bin or --lib )");
            } else if arg3 == "--bin" {
                println!("Creating a new project named {} of type {}", arg2, arg3);
                create_project::create(&arg2, &arg3);
            }
        }
    } else if arg1 == "build" {
        if arg2.is_empty() {
            println!("Please provide a name for the compiled binary.");
        } else {
            build_project::build(&arg2);
        }
    } else if arg1 == "--h" || arg1 == "--help" {
        println!("Organized Resource Collection Assembler v1.1.0");
        println!("made by membercatcousin");
        println!("license: GPL v3");
        println!("Usage:");
        println!("  orca new <project_name> <project_type> - Create a new project (project_type can be --bin or --lib)");
        println!("  orca build <binary_name> - Build the project into a native binary with the given name");
        println!("  orca --help - Show this help message");
        println!("  orca --version - Show the version of the ORCA");

    } else if arg1 == "--version" {
        println!("Organized Resource Collection Assembler v1.1.0");
    } else if arg1.is_empty {
        println!("")
    } else {
        println!("Invalid command: {}. Use 'orca --help' for usage information.", arg1);
    }
}

