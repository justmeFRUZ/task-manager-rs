#[derive(Debug)]
enum Status {
    Pending,
    Done,
}

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


fn mark_done(tasks: &mut Vec<Task>, id: u32) -> bool {
    for task in tasks.iter_mut() {
        if task.id == id {
            task.status = Status::Done;
            return true;
        }
    }

    false
}


fn rm_task(tasks: &mut Vec<Task>, id: u32) -> bool {
    match tasks.iter().position(|t| t.id == id) {
        Some(index) => {
            tasks.remove(index);
            true
        }
        None => false,
    }
}

fn main() {
  
    let args: Vec<String> = std::env::args().collect();
    let mut tasks: Vec<Task> = Vec::new();

    match args.get(1).map(String::as_str) {
        None => println!("no subcommand given"),
        Some("add") => match args.get(2) {
            Some(description) => {
                let id = add_task(&mut tasks, description.clone());
                println!("added task {}", id);
            }
            None => println!("add requires a description"),
        },
        Some("list") => list_tasks(&tasks),
        Some("done") => match args.get(2).map(|s| s.parse::<u32>()) {
            Some(Ok(id)) => {
                let found = mark_done(&mut tasks, id);
                println!("mark_done({}) -> {}", id, found);
            }

            Some(Err(_)) => println!("done requires a numeric id"),
            None => println!("done requires an id"),
        },

        Some("rm") => match args.get(2).map(|s| s.parse::<u32>()) {
            Some(Ok(id)) => {
                let removed = rm_task(&mut tasks, id);
                println!("rm_task({}) -> {}", id, removed);
            }
            Some(Err(_)) => println!("rm requires a numeric id"),
            None => println!("rm requires an id")
        }

        Some(other) => println!("unknown subcommand: {}", other),
    }
}
