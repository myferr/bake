mod parser;
mod task;

use parser::parse_bakefile;
use std::collections::HashMap;
use std::env;
use std::process::{exit, Command};
use std::thread::sleep;
use std::time::Duration;
use task::Task;

fn print_help() {
    println!(
        r#"bake 🍞

./bake             Show this help
./bake --list       List all tasks (or -l)
./bake <task>      Run a task
"#
    );
}

fn list_tasks(tasks: &HashMap<String, Task>) {
    println!("Available tasks:");
    for name in tasks.keys() {
        println!("• {}", name);
    }
}

fn run_task(tasks: &HashMap<String, Task>, name: &str) {
    if let Some(task) = tasks.get(name) {
        println!("\n⏳ Running \"{}\"..", task.name);
        for cmd in &task.commands {
            println!("  › {}", cmd);
            let mut parts = cmd.split_whitespace();
            if let Some(program) = parts.next() {
                let args: Vec<&str> = parts.collect();
                let status = Command::new(program)
                    .args(args)
                    .status()
                    .expect("Failed to run command");
                if !status.success() {
                    eprintln!("  ✗ Command failed: {}", cmd);
                    exit(1);
                }
            }
        }
        println!("✅ Done.\n");
    } else {
        eprintln!("Task \"{}\" not found.", name);
        exit(1);
    }
}

fn main() {
    let bakefile = "Bakefile";
    let tasks = parse_bakefile(bakefile).expect("Failed to parse Bakefile");
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => print_help(),
        2 => {
            let arg = args[1].as_str();
            if arg == "--list" {
                list_tasks(&tasks);
            } else {
                if arg == "-l" {
                    list_tasks(&tasks);
                } else {
                    run_task(&tasks, arg);
                }
            }
        }
        _ => {
            eprintln!("Invalid usage.");
            print_help();
        }
    }
}
