mod parser;
mod task;

fn main() {
    let tasks = parser::parse_bakefile("Bakefile").expect("Failed to parse Bakefile");

    for (name, task) in &tasks {
        println!("Task: {}", name);
        for cmd in &task.commands {
            println!("  - {}", cmd);
        }
    }
}
