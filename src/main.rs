mod parser;
mod task;

use parser::parse_bakefile;
use std::io::Write;
use std::{
    collections::HashMap,
    env,
    process::{exit, Command},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};
use task::Task;

fn print_help() {
    println!(
        r#"bake 🍞

./bake              Show this help
./bake --list (-l)  List all tasks
./bake <task>       Run a task
./bake <task> -v    Run a task (verbose)
"#
    );
}

fn list_tasks(tasks: &HashMap<String, Task>) {
    println!("Available tasks:");
    for name in tasks.keys() {
        println!("• {}", name);
    }
}

fn run_task(tasks: &HashMap<String, Task>, name: &str, verbose: bool) {
    if let Some(task) = tasks.get(name) {
        let running = Arc::new(AtomicBool::new(true));
        let name_for_spinner = name.to_string(); // clone for thread

        let spinner_handle = {
            let running = running.clone();
            thread::spawn(move || {
                let spinner_chars = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
                let mut i = 0;
                while running.load(Ordering::Relaxed) {
                    print!(
                        "\r{} Running \"{}\"..",
                        spinner_chars[i % spinner_chars.len()],
                        name_for_spinner
                    );
                    std::io::stdout().flush().unwrap();
                    thread::sleep(Duration::from_millis(100));
                    i += 1;
                }
            })
        };

        for cmd in &task.commands {
            if verbose {
                println!("› {}", cmd);
            }
            let mut parts = cmd.split_whitespace();
            if let Some(program) = parts.next() {
                let args: Vec<&str> = parts.collect();
                let status = Command::new(program)
                    .args(args)
                    .stdout(if verbose {
                        std::process::Stdio::inherit()
                    } else {
                        std::process::Stdio::null()
                    })
                    .stderr(if verbose {
                        std::process::Stdio::inherit()
                    } else {
                        std::process::Stdio::null()
                    })
                    .status()
                    .expect("Failed to run command");
                if !status.success() {
                    running.store(false, Ordering::Relaxed);
                    spinner_handle.join().unwrap();
                    eprintln!("\n✗ Command failed: {}", cmd);
                    exit(1);
                }
            }
        }

        running.store(false, Ordering::Relaxed);
        spinner_handle.join().unwrap();
        println!("\r✅ Done \"{}\"                      ", name);
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
            if arg == "--list" || arg == "-l" {
                list_tasks(&tasks);
            } else {
                run_task(&tasks, arg, false);
            }
        }
        3 => {
            let task = args[1].as_str();
            let flag = args[2].as_str();
            let verbose = flag == "--verbose" || flag == "-v";
            run_task(&tasks, task, verbose);
        }
        _ => {
            eprintln!("Invalid usage.");
            print_help();
        }
    }
}
