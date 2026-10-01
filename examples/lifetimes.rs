#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Priority {
    Medium,
    High,   
}

struct Task {
    description: String,
    priority: Priority,
}

fn pick_higher<'a>(a: &'a Task, b: &'a Task) -> &'a Task {
    if a.priority >= b.priority {
        a
    } else {
        b
    }
}

fn main() {
    let t1 = Task {
        description: String::from("write tests"),
        priority: Priority::Medium,
    };

    let t2 = Task {
        description: String::from("fix bug"),
        priority: Priority::High,
    };

    let winner = pick_higher(&t1, &t2);
    println!("{}", winner.description);
}