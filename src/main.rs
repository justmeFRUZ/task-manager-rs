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
    match args.get(1) {
        None => println!("no subcommand given"),
        Some(cmd) => println!("Subcommand: {}", cmd),
    }

    let mut tasks = vec![
        Task{
            id: 1,
            description: String::from("Write the Rust Verison"),
            status: Status::Pending,
        },
        Task {
            id: 2,
            description: String::from("Commit the strut"),
            status: Status::Done,
        },
    ];

list_tasks(&tasks);
let new_id = add_task(&mut tasks, String::from("Try add_task"));
println!("added task {}", new_id);

let found = mark_done(&mut tasks, 1);
println!("mark_done(1) -> {}", found);

let removed = rm_task(&mut tasks, 2);
println!("rm_task(2) -> {}", removed);

let missing = rm_task(&mut tasks, 99);
println!("rm_task(99) -> {}", missing);

list_tasks(&tasks);

}
