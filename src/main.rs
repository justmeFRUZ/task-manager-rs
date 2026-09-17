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

fn main() {
    let t1 = Task {

        id: 1,
        description: String::from("Write the Rust version"),
        status: Status::Pending,
    };

    let t2 = Task {

        id:2,
        description: String::from("Commit the struct"),
        status: Status::Done,
    };

    render(&t1);
    render(&t2);
    
    println!("{:?}", t1.status);
    println!("{:?}", t2.status);


}
