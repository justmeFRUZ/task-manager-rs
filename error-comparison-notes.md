# Python vs Rust error messages — Week 3 Weekday 2

Source of truth for this comparison: same bad input, run from a temp dir
outside either repo, so neither repo's real tasks.json was touched.

## Missing file

Python (`python task_manager.py list` from a dir with no tasks.json):
    No tasks found.
    exit 0

Rust (`task-manager-rs.exe list` from a dir with no tasks.json):
    (no output)
    exit 0

Difference: same exit code, same semantics (empty list is not an error).
Python prints a message; Rust is silent. Rust's silence is a design choice
in load_tasks — NotFound is mapped to Ok(Vec::new()), not to an error.

## Corrupted file (tasks.json contains: this is not json)

Python:
    Traceback (most recent call last):
      ... ~20 lines of internal frames ...
    CorruptedTasksFileError: Tasks file is corrupted and cannot be loaded: tasks.json
    exit 1

Rust:
    failed to load tasks: data corrupted: expected ident at line 1 column 2
    exit 1

Difference: both exit 1. Python's custom error fires correctly but is not
caught in main(), so the user sees a raw traceback. Rust emits one line on
stderr. Rust's serde message includes exact line/column of the parse
failure; Python's message includes only the path, not the parse position
(the underlying JSONDecodeError has the position but it is buried in the
"direct cause" chain).

## Flags for the weekly check-in (do not fix this session)

- Python: catch CorruptedTasksFileError in main() and print a clean
  message. Tracebacks are for developers, not CLI users.
- Rust: silent exit 0 on missing file is defensible for `list`, but
  worth a deliberate decision for `add`.
- Rust: serde's error text ("expected ident at line 1 column 2") leaks
  a parser detail. Fine for developers; worth considering whether the
  README should present it as-is.