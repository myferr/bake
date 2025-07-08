use crate::task::Task;
use std::collections::HashMap;
use std::fs;

pub fn parse_bakefile(path: &str) -> std::io::Result<HashMap<String, Task>> {
    let data = fs::read_to_string(path)?;
    let mut tasks = HashMap::new();
    let mut current_task: Option<Task> = None;

    for line in data.lines() {
        let line = line.trim_end();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if !line.starts_with('\t') && !line.starts_with(' ') {
            let mut parts = line.splitn(2, ':');
            let name = parts.next().unwrap().trim();
            let deps = parts
                .next()
                .map(|s| s.split_whitespace().map(|d| d.to_string()).collect())
                .unwrap_or_else(Vec::new);

            if let Some(task) = current_task.take() {
                tasks.insert(task.name.clone(), task);
            }

            current_task = Some(Task {
                name: name.to_string(),
                deps,
                commands: Vec::new(),
            });
        } else if let Some(task) = &mut current_task {
            task.commands.push(line.trim().to_string());
        }
    }

    if let Some(task) = current_task {
        tasks.insert(task.name.clone(), task);
    }

    Ok(tasks)
}
