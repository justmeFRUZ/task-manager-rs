struct Task {

    id: u32,
    description: String,
    completed: bool,
}

fn render(task: &Task) {

    println!("[{}] {} (done: {})", task.id, task.description, task.completed);
}

fn main() {
    let task = Task {

        id: 1,
        description: String:: from("Write the Rust version"),
        completed: false,
    };

    render(&task);
    render(&task);

    println!("{}", task.description);
}
