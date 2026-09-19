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

fn main() {
    
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

list_tasks(&tasks);

}
