use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum Status {
    Pending,
    Done,
}

#[derive(Debug)]
enum TaskError {
    NotFound { id: u32 },
    Io(std::io::Error),
    Corrupted(String),
}

#[derive(Serialize, Deserialize)]
struct Task {

    id: u32,
    description: String,
    status: Status,
}

fn render(task: &Task) {

    println!("[{}] {} (status: {:?})", task.id, task.description, task.status);
}

fn list_tasks(tasks: &[Task]) {
    for task in tasks {
        render(task);
    }
}

fn add_task(tasks: &mut Vec<Task>, description: String) -> u32 {

    let mut max_id = 0;
    for task in tasks.iter() {
        if task.id > max_id {
            max_id = task.id;
        }
    }

    let new_id = max_id + 1;
    tasks.push(Task {
        id: new_id,
        description: description,
        status: Status::Pending,
    });
    new_id

} 


fn mark_done(tasks: &mut Vec<Task>, id: u32) -> Result<(), TaskError> {
    for task in tasks.iter_mut() {
        if task.id == id {
            task.status = Status::Done;
            return Ok(());
        }
    }

    Err(TaskError::NotFound { id })
}


fn rm_task(tasks: &mut Vec<Task>, id: u32) -> Result<(), TaskError> {
    match tasks.iter().position(|t| t.id == id) {
        Some(index) => {
            tasks.remove(index);
            Ok(())
        }
        None => Err(TaskError::NotFound { id }),
    }
}


fn load_tasks(path: &str) -> Result<Vec<Task>, TaskError> {
    let contents = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(TaskError::Io(e)),
    };
    if contents.trim().is_empty() {
        return Ok(Vec::new());
    }

    match serde_json::from_str::<Vec<Task>>(&contents) {
        Ok(tasks) => Ok(tasks),
        Err(e) => Err(TaskError::Corrupted(e.to_string())),
    }
}

fn save_tasks(path: &str, tasks: &[Task]) -> Result<(), TaskError> {
    let json = match serde_json::to_string(tasks) {
        Ok(s) => s,
        Err(e) => return Err(TaskError::Corrupted(e.to_string())),
    };

    match std::fs::write(path, json) {
        Ok(()) => Ok(()),
        Err(e) => Err(TaskError::Io(e)),
    }
}


fn render_error(e: &TaskError) -> String {
    match e {
        TaskError::NotFound {id} => format!("task {} not found", id),
        TaskError::Io(io_err) => format! ("I/O error: {}", io_err),
        TaskError::Corrupted(msg) => format!("data corrupted: {}", msg),
    }
}

fn main() {
  

    let args: Vec<String> = std::env::args().collect();
    let path = "tasks.json";

    let mut tasks: Vec<Task> = match load_tasks(path) {

        Ok(t) => t,
        Err(e) => {

            eprintln!("failed to load tasks: {}", render_error(&e));
            std::process::exit(1);
        }
    };

    match args.get(1).map(String::as_str) {
        None => println!("no subcommand given"),
        Some("add") => match args.get(2) {
            Some(description) => {
                let id = add_task(&mut tasks, description.clone());
                if let Err(e) = save_tasks(path, &tasks) {
                    eprintln!("failed to save tasks: {}", render_error(&e));
                    std::process::exit(1);

                }
                println!("added task {}", id);
            }
            None => println!("add requires a description"),
        },
        Some("list") => list_tasks(&tasks),
        Some("done") => match args.get(2).map(|s| s.parse::<u32>()) {
            Some(Ok(id)) => match mark_done(&mut tasks, id) {
                Ok(()) => {
                    if let Err(e) = save_tasks(path, &tasks) {
                        eprintln!("failed to save tasks: {}", render_error(&e));
                        std::process::exit(1);
                    }
                    println!("mark_done({}) -> ok", id);
                }
                Err(e) => println!("mark_done({}) -> {}", id, render_error(&e)),
            },

            Some(Err(_)) => println!("done requires a numeric id"),
            None => println!("done requires an id"),
        },

        Some("rm") => match args.get(2).map(|s| s.parse::<u32>()) {
            Some(Ok(id)) => match rm_task(&mut tasks, id) {
                Ok(()) => {
                    if let Err(e) = save_tasks(path, &tasks) {
                        eprintln!("failed to save tasks: {}", render_error(&e));
                        std::process::exit(1);
                    }
                    println!("rm_task({}) -> ok", id );

                }
                Err(e) => println!("rm_task({}) -> {}", id, render_error(&e)),
            },

            Some(Err(_)) => println!("rm requires a numeric id"),
            None => println!("re requires an id")
        }

        Some(other) => println!("unknown subcommand: {}", other),
    }
}
