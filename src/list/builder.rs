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

pub fn deserialize_list<P: Into<PathBuf>, S: AsRef<str>>(path: P, contents: S) -> List {
    let mut open_tasks: Vec<Task> = Vec::new();
    let mut done_tasks: Vec<Task> = Vec::new();

    for task in contents.as_ref().lines() {
        let task = Task::new(task);
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
