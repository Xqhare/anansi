use std::{collections::BTreeMap, path::PathBuf};

use super::{TaskID, List, Task};

pub fn build_default_list<P: Into<PathBuf>>(path: P) -> List {
    List {
        file_path: path.into(),
        tasks: BTreeMap::new(),
        open_tasks: Vec::new(),
        done_tasks: Vec::new(),
    }
}

pub fn deserialise_list<P: Into<PathBuf>, S: AsRef<str>>(path: P, contents: S) -> List {
    let mut tasks: BTreeMap<TaskID, Box<Task>> = BTreeMap::new();
    let mut open_tasks: Vec<TaskID> = Vec::new();
    let mut done_tasks: Vec<TaskID> = Vec::new();

    for (id, task) in contents.as_ref().lines().enumerate() {
        let task = Task::new(task, id);
        if task.is_done() {
            done_tasks.push(id);
        } else {
            open_tasks.push(id);
        }
        tasks.insert(id, Box::new(task));
    }

    List {
        file_path: path.into(),
        tasks,
        open_tasks,
        done_tasks,
    }
}

pub fn serialise_list(list: &List) -> String {
    let mut output = String::new();
    for (_, task) in list.tasks.iter() {
        output.push_str(&task.original());
        output.push('\n');
    }
    output
}
