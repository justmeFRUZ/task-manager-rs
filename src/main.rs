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
    println!(
        "[{}] {} (status: {:?})",
        task.id, task.description, task.status
    );
}

fn list_tasks(tasks: &[Task], only_pending: bool) {
    for task in tasks {
        if only_pending && !matches!(task.status, Status::Pending) {
            continue;
        }
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
        description,
        status: Status::Pending,
    });
    new_id
}

fn mark_done(tasks: &mut [Task], id: u32) -> Result<(), TaskError> {    for task in tasks.iter_mut() {
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
        TaskError::NotFound { id } => format!("task {} not found", id),
        TaskError::Io(io_err) => format!("I/O error: {}", io_err),
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
        Some("list") => match args.get(2).map(String::as_str) {
            None => list_tasks(&tasks, false),
            Some("--pending") => list_tasks(&tasks, true),
            Some(other) => println!("unknown flag for list: {}", other),
        },
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
                    println!("rm_task({}) -> ok", id);
                }
                Err(e) => println!("rm_task({}) -> {}", id, render_error(&e)),
            },

            Some(Err(_)) => println!("rm requires a numeric id"),
            None => println!("rm requires an id"),
        },

        Some(other) => println!("unknown subcommand: {}", other),
    }
}

#[cfg(test)]

fn temp_path(name: &str) -> String {
    std::env::temp_dir()
        .join(name)
        .to_string_lossy()
        .into_owned()
}
#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn add_task_returns_incremented_id_on_empty_list() {
        let mut tasks: Vec<Task> = Vec::new();
        let id = add_task(&mut tasks, String::from("buy milk"));
        assert_eq!(id, 1);
        assert_eq!(tasks.len(), 1);
    }

    #[test]
    fn add_task_returns_max_id_plus_one() {
        let mut tasks = vec![
            Task {
                id: 1,
                description: String::from("a"),
                status: Status::Pending,
            },
            Task {
                id: 5,
                description: String::from("b"),
                status: Status::Pending,
            },
        ];
        let id = add_task(&mut tasks, String::from("c"));
        assert_eq!(id, 6);
    }

    #[test]
    fn add_task_second_call_increments_id() {
        let mut tasks: Vec<Task> = Vec::new();
        add_task(&mut tasks, String::from("first"));
        let id = add_task(&mut tasks, String::from("second"));
        assert_eq!(id, 2);
    }

    #[test]
    fn mark_done_sets_status_to_done() {
        let mut tasks = vec![Task {
            id: 1,
            description: String::from("a"),
            status: Status::Pending,
        }];

        let result = mark_done(&mut tasks, 1);
        assert!(result.is_ok());
        assert!(matches!(tasks[0].status, Status::Done));
    }

    #[test]
    fn mark_done_unknown_id_returns_not_found() {
        let mut tasks: Vec<Task> = Vec::new();
        let result = mark_done(&mut tasks, 999);
        assert!(matches!(result, Err(TaskError::NotFound { id: 999 })));
    }

    #[test]
    fn rm_task_removes_task() {
        let mut tasks = vec![Task {
            id: 1,
            description: String::from("a"),
            status: Status::Pending,
        }];
        let result = rm_task(&mut tasks, 1);
        assert!(result.is_ok());
        assert_eq!(tasks.len(), 0);
    }

    #[test]
    fn rm_task_unknown_id_returns_not_found() {
        let mut tasks: Vec<Task> = Vec::new();
        let result = rm_task(&mut tasks, 999);
        assert!(matches!(result, Err(TaskError::NotFound { id: 999 })));
    }

    #[test]
    fn load_tasks_missing_file_returns_empty_vec() {
        let path = temp_path("tm_rs_missing_never_exists.json");
        let _ = std::fs::remove_file(&path);
        let result = load_tasks(&path);
        assert!(matches!(result, Ok(ref v) if v.is_empty()));
    }

    #[test]
    fn load_tasks_corrupted_file_returns_corrupted_error() {
        let path = temp_path("tm_rs_corrupted.json");
        std::fs::write(&path, "this is not json").expect("write to temp dir failed");

        let result = load_tasks(&path);
        assert!(matches!(result, Err(TaskError::Corrupted(_))));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn save_then_load_round_trips() {
        let path = temp_path("tm_rs_roundtrip.json");
        let _ = std::fs::remove_file(&path);
        let mut tasks: Vec<Task> = Vec::new();
        add_task(&mut tasks, String::from("buy milk"));
        add_task(&mut tasks, String::from("walk dog"));
        save_tasks(&path, &tasks).expect("save to temp dir failed");
        let loaded: Vec<Task> = load_tasks(&path).expect("load from temp dir failed");
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].id, 1);
        assert_eq!(loaded[0].description, "buy milk");
        assert_eq!(loaded[1].id, 2);
        assert!(matches!(loaded[0].status, Status::Pending));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn load_task_json_type_mismatch_returns_corrupted() {
        let path = temp_path("tm_rs_type_mismatch.json");
        std::fs::write(
            &path,
            r#"[{"id":"not a number","description":"a","status":"Pending"}]"#,
        )
        .expect("write to temp dir failed");
        let result = load_tasks(&path);
        assert!(matches!(result, Err(TaskError::Corrupted(_))));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn save_tasks_permission_denied_returns_io_error() {
        let path = temp_path("tm_rs_perm_denied.json");
        let _ = std::fs::remove_file(&path);
        std::fs::write(&path, "[]").expect("initial write to temp dir failed");


        let original_perms = std::fs::metadata(&path)
            .expect("metadata failed")
            .permissions();



            let mut readonly_perms = original_perms.clone();
            readonly_perms.set_readonly(true);
            std::fs::set_permissions(&path, readonly_perms).expect("set_permissions failed");


        let tasks: Vec<Task> = Vec::new();
        let result = save_tasks(&path, &tasks);

        std::fs::set_permissions(&path, original_perms).expect("restore permissions failed");
        assert!(matches!(result, Err(TaskError::Io(_))));

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn load_tasks_whitespace_only_file_returns_empty_vec() {
        let path = temp_path("tm_rs_whitespace_json");
        std::fs::write(&path, " \n\t  \n").expect("write to temp dir failed");
        let result = load_tasks(&path);
        assert!(matches!(result, Ok(ref v) if v.is_empty()));
        let _ = std::fs::remove_file(&path);
    }
}
