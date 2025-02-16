use std::path::PathBuf;

use crate::Task;

use super::List;

pub fn build_default_list<P: Into<PathBuf>>(path: P) -> List {
    List {
        file_path: path.into(),
        open_tasks: Vec::new(),
        done_tasks: Vec::new(),
    }
}

pub fn deserialise_list<P: Into<PathBuf>, S: AsRef<str>>(path: P, contents: S) -> List {
    let mut open_tasks: Vec<Task> = Vec::new();
    let mut done_tasks: Vec<Task> = Vec::new();

    for (id, task) in contents.as_ref().lines().enumerate() {
        let task = Task::new(task, id);
        if task.is_done() {
            done_tasks.push(task);
        } else {
            open_tasks.push(task);
        }
    }

    List {
        file_path: path.into(),
        open_tasks,
        done_tasks,
    }
}

pub fn serialise_list(list: &List) -> String {
    let mut output = String::new();
    for task in &list.open_tasks {
        output.push_str(&task.original());
        output.push('\n');
    }
    for task in &list.done_tasks {
        output.push_str(&task.original());
        output.push('\n');
    }
    output
}
