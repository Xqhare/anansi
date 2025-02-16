mod builder;

use std::path::PathBuf;

use builder::{build_default_list, deserialise_list, serialise_list};

use crate::{Task, TaskList};

#[derive(Clone)]
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
    /// let list = List::new("path/to/list.txt");
    /// ```
    pub fn new<P: Into<PathBuf>>(path: P) -> List {
        let file_path = path.into();
        if let Ok(file) = std::fs::read_to_string(&file_path) {
            // load from file
            deserialise_list(file_path, file.trim())
        } else {
            // new list for new file
            build_default_list(file_path)
        }
    }

    /// Add a task to the list.
    ///
    /// # Example
    /// ```
    /// use anansi::List;
    /// let mut list = List::new("path/to/list.txt");
    /// list.add("Task 1");
    /// list.add("Task 2");
    /// assert_eq!(list.open().len(), 2);
    /// ```
    pub fn add<S: AsRef<str>>(&mut self, task: S) {
        let task: Task = Task::new(task.as_ref(), self.open_tasks.len().saturating_add(self.done_tasks.len()));
        if task.is_done() {
            self.done_tasks.push(task);
        } else {
            self.open_tasks.push(task);
        }
    }

    // TODO: Optimise this - maybe don't load all tasks into memory, don't have a better idea though
    /// Remove a task from the list, done or open.
    ///
    /// Provide a `&Task` to remove.
    ///
    /// # Example
    /// ```
    /// use anansi::{List, Task};
    /// let mut list = List::new("path/to/list.txt");
    /// list.add("Task 1");
    /// list.add("Task 2");
    /// assert_eq!(list.open().len(), 2);
    /// let task = list.open()[0].clone();
    /// list.remove(&task);
    /// assert_eq!(list.open().len(), 1);
    /// ```
    pub fn remove(&mut self, task: &Task) {
        if task.is_done() {
            self.done_tasks.retain(|t| t.original() != task.original());
        } else {
            self.open_tasks.retain(|t| t.original() != task.original());
        }
    }

    /// Update a task in the list.
    ///
    /// Provide a new task and the id of the task to update.
    ///
    /// # Example
    /// ```
    /// use anansi::{List, Task};
    /// let mut list = List::new("path/to/list.txt");
    /// list.add("Task 1");
    /// list.add("Task 2");
    /// let task = list.open()[0].clone();
    /// list.update("Task 3", task.id());
    /// assert_eq!(list.get(task.id()).unwrap().original(), "Task 3");
    /// ```
    pub fn update<S: AsRef<str>>(&mut self, new_task: S, task_id: usize) {
        let new_task = Task::new(new_task.as_ref(), task_id);
        self.done_tasks.retain(|t| t.id() != task_id);
        self.open_tasks.retain(|t| t.id() != task_id);
        if new_task.is_done() {
            self.done_tasks.push(new_task);
        } else {
            self.open_tasks.push(new_task);
        }
    }

    /// Get a task by id.
    ///
    /// # Example
    /// ```
    /// use anansi::{List, TaskList};
    /// let mut list = List::new("path/to/list.txt");
    /// list.add("Task 1");
    /// list.add("Task 2");
    /// let task = list.get(0).unwrap();
    /// assert_eq!(task.original(), "Task 1");
    /// ```
    pub fn get(&self, id: usize) -> Option<Task> {
        self.open_tasks.iter().find(|t| t.id() == id).cloned().or_else(|| self.done_tasks.iter().find(|t| t.id() == id).cloned())
    }

    /// Get all done tasks.
    ///
    /// # Example
    /// ```
    /// use anansi::{List, TaskList};
    /// let mut list = List::new("path/to/list.txt");
    /// list.add("x Task 1");
    /// list.add("Task 2");
    /// let done_tasks = list.done();
    /// assert_eq!(done_tasks.len(), 1);
    /// ```
    pub fn done(&self) -> &Vec<Task> {
        &self.done_tasks
    }

    /// Get all open tasks.
    ///
    /// # Example
    /// ```
    /// use anansi::{List, TaskList};
    /// let mut list = List::new("path/to/list.txt");
    /// list.add("x Task 1");
    /// list.add("Task 2");
    /// let open_tasks = list.open();
    /// assert_eq!(open_tasks.len(), 1);
    /// ```
    pub fn open(&self) -> &Vec<Task> {
        &self.open_tasks
    }

    /// Save the list to the file.
    ///
    /// # Example
    /// ```
    /// use anansi::List;
    /// let mut list = List::new("list.txt");
    /// list.add("Task 1");
    /// list.add("Task 2");
    ///
    /// let result = list.save();
    /// assert!(result.is_ok());
    ///
    /// # let _ = std::fs::remove_file("list.txt");
    /// ```
    pub fn save(&self) -> Result<(), std::io::Error> {
        let serialised = serialise_list(self);
        std::fs::write(&self.file_path, serialised)
    }

    /// Filter tasks by priority.
    ///
    /// Will filter both open and done tasks.
    /// This will filter by checking if the predicate is contained within any priority tag.
    ///
    /// The filter is case insensitive.
    ///
    /// # Example
    /// ```
    /// use anansi::{List, TaskList};
    ///
    /// let mut list = List::new("path/to/list.txt");
    /// list.add("(A) Task 1");
    /// list.add("(A) Task 2 @air");
    /// list.add("(B) Task 3 @AIR");
    /// list.add("(Z) Task 4 @AirCraft");
    /// let filtered = list.by_prio("a");
    /// assert_eq!(filtered.tasks().len(), 2);
    /// ```
    pub fn by_prio<S: Into<String>>(&self, prio: S) -> TaskList {
        let prio = prio.into().to_uppercase();
        let mut open_filtered: Vec<Task> = self.open_tasks
            .iter()
            .filter(|task| task.prio() == prio)
            .cloned()
            .collect();
        let mut done_filtered: Vec<Task> = self.done_tasks
            .iter()
            .filter(|task| task.prio() == prio)
            .cloned()
            .collect();
        open_filtered.append(&mut done_filtered);
        TaskList::new(open_filtered)
    }

    /// Filter tasks by context.
    ///
    /// Will filter both open and done tasks.
    /// This will filter by checking if the predicate is contained within any context tag.
    /// This means that calling `by_context("air")` would return tasks with context tags `air` or `aircraft`.
    ///
    /// The filter is case insensitive.
    ///
    /// Can be chained with any other `by_*` method.
    ///
    /// # Example
    /// ```
    /// use anansi::{List, TaskList};
    ///
    /// let mut list = List::new("path/to/list.txt");
    /// list.add("Task 1");
    /// list.add("Task 2 @air");
    /// list.add("Task 3 @AIR");
    /// list.add("x Task 4 @AirCraft");
    ///
    /// assert_eq!(list.by_context("air").tasks().len(), 3);
    /// assert_eq!(list.by_context("craft").tasks().len(), 1);
    /// ```
    pub fn by_context<S: Into<String>>(&self, context: S) -> TaskList {
        let context = context.into();
        let mut open_filtered: Vec<Task> = self.open_tasks
            .iter()
            .filter(|task| {
                for tag in task.contexts() {
                    if tag.to_lowercase().contains(&context.to_lowercase()) {
                        return true;
                    }
                }
                return false;
            })
            .cloned()
            .collect();
        let mut done_filtered: Vec<Task> = self.done_tasks
            .iter()
            .filter(|task| {
                for tag in task.contexts() {
                    if tag.to_lowercase().contains(&context.to_lowercase()) {
                        return true;
                    }
                }
                return false;
            })
            .cloned()
            .collect();
        open_filtered.append(&mut done_filtered);
        TaskList::new(open_filtered)
    }

    /// Filter tasks by project.
    ///
    /// Will filter both open and done tasks.
    /// This will filter by checking if the predicate is contained within any project tag.
    /// This means that calling `by_project("air")` would return tasks with project tags `air` or `aircraft`.
    ///
    /// The filter is case insensitive.
    ///
    /// Can be chained with any other `by_*` method.
    ///
    /// # Example
    /// ```
    /// use anansi::{List, TaskList};
    ///
    /// let mut list = List::new("path/to/list.txt");
    /// list.add("Task 1");
    /// list.add("Task 2 +home");
    /// list.add("Task 3 +HOME");
    /// list.add("x Task 4 +homeImprovements");
    ///
    /// assert_eq!(list.by_project("home").tasks().len(), 3);
    /// assert_eq!(list.by_project("improvements").tasks().len(), 1);
    pub fn by_project<S: Into<String>>(&self, project: S) -> TaskList {
        let project = project.into();
        let mut open_filtered: Vec<Task> = self.open_tasks
            .iter()
            .filter(|task| {
                for tag in task.projects() {
                    if tag.to_lowercase().contains(&project.to_lowercase()) {
                        return true;
                    }
                }
                return false;
            })
            .cloned()
            .collect();
        let mut done_filtered: Vec<Task> = self.done_tasks
            .iter()
            .filter(|task| {
                for tag in task.projects() {
                    if tag.to_lowercase().contains(&project.to_lowercase()) {
                        return true;
                    }
                }
                return false;
            })
            .cloned()
            .collect();
        open_filtered.append(&mut done_filtered);
        TaskList::new(open_filtered)
    }

    /// Filter tasks by special tag.
    ///
    /// Will filter both open and done tasks.
    /// This will filter by checking if the predicate is contained within any special tag key.
    ///
    /// The filter is case insensitive.
    ///
    /// Can be chained with any other `by_*` method.
    ///
    /// # Example
    /// ```
    /// use anansi::{List, TaskList};
    ///
    /// let mut list = List::new("path/to/list.txt");
    /// list.add("Task 1");
    /// list.add("Task 2 due:tomorrow");
    /// list.add("Task 3 DUE:31.12");
    /// list.add("x Task 4 assignment_due:2020-01-01");
    /// 
    /// for t in list.by_special("due").tasks() {
    ///     println!("{:?}", t);
    /// }
    ///
    /// assert_eq!(list.by_special("due").tasks().len(), 3);
    /// assert_eq!(list.by_special("assignment").tasks().len(), 1);
    /// ```
    pub fn by_special<S: Into<String>>(&self, special: S) -> TaskList {
        let special = special.into();
        let mut open_filtered: Vec<Task> = self.open_tasks
            .iter()
            .filter(|task| {
                for key in task.specials().keys() {
                    if key.to_lowercase().contains(&special.to_lowercase()) {
                        return true;
                    }
                }
                return false;
            })
            .cloned()
            .collect();
        let mut done_filtered: Vec<Task> = self.done_tasks
            .iter()
            .filter(|task| {
                for key in task.specials().keys() {
                    if key.to_lowercase().contains(&special.to_lowercase()) {
                        return true;
                    }
                }
                return false;
            })
            .cloned()
            .collect();
        open_filtered.append(&mut done_filtered);
        TaskList::new(open_filtered)
    }
}
