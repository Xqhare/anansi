use std::path::PathBuf;

use crate::{util::{is_ascii_whitespace, is_newline}, Task};

use super::List;

pub fn build_default_list<P: Into<PathBuf>>(path: P) -> List {
    List {
        file_path: path.into(),
        open_tasks: Vec::new(),
        done_tasks: Vec::new(),
    }
}

pub fn deserialise_list<P: Into<PathBuf>>(path: P, contents: &Vec<u8>) -> List {
    let mut open_tasks: Vec<Task> = Vec::new();
    let mut done_tasks: Vec<Task> = Vec::new();
    let mut index: usize = 0;

    // leading whitespace removal
    while is_ascii_whitespace(contents[index]) {
        index += 1;
    }

    let mut line_start_index = index;
    // Loop through file
    while index < contents.len() {
        let is_newline = is_newline(index, &contents);
        if is_newline.0 {
            // End of line
            // Lossy should be fine here, can be done properly if needs must
            let task = Task::new(String::from_utf8_lossy(&contents[line_start_index..index]));
            if task.is_done() {
                done_tasks.push(task);
            } else {
                open_tasks.push(task);
            }
            index = is_newline.1;
            line_start_index = index;
        } else {
            // Inside line
            index += 1;
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
