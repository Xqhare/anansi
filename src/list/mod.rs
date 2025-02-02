mod builder;

use std::path::PathBuf;

use builder::{build_default_list, deserialize_list};

use crate::Task;

pub struct List {
    file_path: PathBuf,
    open_tasks: Vec<Task>,
    done_tasks: Vec<Task>,
}

impl List {
    /// Create a new list.
    /// If the supplied path exists, load the file and deserialize.
    /// If the supplied path does not exist, create a new, empty list.
    ///
    /// # Example
    /// ```
    /// use anansi::List;
    /// let list = List::new("path/to/list.json");
    /// ```
    pub fn new<P: Into<PathBuf>>(path: P) -> List {
        let file_path = path.into();
        if let Ok(file) = std::fs::read_to_string(&file_path) {
            // load from file
            deserialize_list(file_path, file)
        } else {
            // new list for new file
            build_default_list(file_path)
        }
    }

    pub fn add<S: AsRef<str>>(&mut self, task: S) {
        let task = Task::new(task);
        if task.is_done() {
            self.done_tasks.push(task);
        } else {
            self.open_tasks.push(task);
        }
    }
}
