mod parser;
mod task;

use parser::{parse_bakefile, parse_makefile};
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

USAGE:
  bake --help (-h)                  Show this help
  bake --list (-l)                  List all tasks
  bake <task>                       Run a task
  bake <task> -v                    Run a task (verbose)
  bake <task> -m                    Use Makefile instead of Bakefile
  bake <task> -m -v                 Use Makefile + verbose

If you installed this using cargo, use "bake-tool" instead of "bake".
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
        let name_for_spinner = name.to_string();

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
        println!("\r✓ Done \"{}\"                      ", name);
    } else {
        eprintln!("Task \"{}\" not found.", name);
        exit(1);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut use_makefile = false;
    let mut verbose = false;
    let mut task_name: Option<String> = None;
    let mut show_help = false;

    for arg in &args[1..] {
        match arg.as_str() {
            "--list" | "-l" => {
                let (tasks, _) = parse_bakefile("Bakefile").unwrap_or_else(|_| (HashMap::new(), None));
                list_tasks(&tasks);
                return;
            }
            "--verbose" | "-v" => verbose = true,
            "--makefile" | "-m" => use_makefile = true,
            "--help" | "-h" => {
                print_help();
                show_help = true;
            }
            _ if !arg.starts_with('-') => task_name = Some(arg.clone()),
            _ => {}
        }
    }

    if show_help {
        return;
    }

    let file = if use_makefile { "Makefile" } else { "Bakefile" };
    
    let (tasks, default_task) = if use_makefile {
        parse_makefile(file)
    } else {
        parse_bakefile(file)
    }
    .unwrap_or_else(|_| {
        eprintln!("Could not read {}.", file);
        exit(1);
    });

    if let Some(name) = task_name {
        run_task(&tasks, &name, verbose);
    } else if let Some(default) = default_task {
        run_task(&tasks, &default, verbose);
    } else {
        eprintln!("No task specified and no default task found.");
        exit(1);
    }
}