use std::{env, vec};
use std::fs::write;
use std::process::{exit, ExitCode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Todo {
    full_file_path: String,
    relative_file_path: String,
    line_number: usize,
    todo_text: String,
}



fn main() -> ExitCode {
    let github_output_path = env::var("GITHUB_OUTPUT").unwrap();

    let args: Vec<String> = env::args().collect();
    let linked_issues = &args[1];
    let root_path = &args[2];
    let file_pattern = &args[3];

    let todos = check_for_todos(linked_issues, root_path, file_pattern);

    if !todos.is_empty() {
        let todos_json = serde_json::to_string(&todos).expect("Failed to serialize TODOs to JSON");
        write(github_output_path, format!("todos={}", todos_json)).unwrap();
        return ExitCode::FAILURE
    }
    ExitCode::SUCCESS
}

fn check_for_todos(linked_issues: &str, root_path: &str, file_pattern: &str) -> Vec<Todo> {
    // Implement your TODO checking logic here
    // You may use the linked_issues, root_path, and file_pattern parameters
    // Return a Vec<Todo> containing the TODOs found
    // Example: Dummy implementation returning an empty vector
    vec![
        Todo{
            full_file_path: "full_file_path".to_string(),
            relative_file_path: "relative_file_path".to_string(),
            line_number: 1,
            todo_text: "todo_text".to_string()
        }
    ]
}
