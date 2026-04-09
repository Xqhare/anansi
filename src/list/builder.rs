use std::{collections::BTreeMap, path::PathBuf};

use super::{List, Task, TaskID};

pub fn build_default_list<P: Into<PathBuf>>(path: P) -> List {
    List {
        file_path: path.into(),
        tasks: BTreeMap::new(),
        open_tasks: Vec::new(),
        done_tasks: Vec::new(),
        max_id: None,
    }
}

pub fn deserialise_list<P: Into<PathBuf>, S: AsRef<str>>(path: P, contents: S) -> List {
    let task_amount = contents.as_ref().lines().count();
    let mut tasks: BTreeMap<TaskID, Task> = BTreeMap::new();
    // Probably Overallocates both open and done tasks;
    // This is a bonus for optimisation when working with large lists (marking as done / creating new tasks == no expansion for some time)
    //
    // Example: 1000 tasks; 500 done; 500 open -> Both lists have a capacity of 1000; No expansion
    // of either list is required until they hit 1000 tasks held
    let mut open_tasks: Vec<TaskID> = Vec::with_capacity(task_amount);
    let mut done_tasks: Vec<TaskID> = Vec::with_capacity(task_amount);

    for (id, task) in contents.as_ref().lines().enumerate() {
        let task = Task::new(task, id);
        if task.is_done() {
            done_tasks.push(id);
        } else {
            open_tasks.push(id);
        }
        tasks.insert(id, task);
    }

    let max_id = tasks.keys().max().copied();

    List {
        file_path: path.into(),
        tasks,
        open_tasks,
        done_tasks,
        max_id,
    }
}

pub fn serialise_list(list: &List) -> String {
    let mut output = String::new();
    for (_, task) in list.tasks.iter() {
        output.push_str(&task.to_string());
        output.push('\n');
    }
    output
}
