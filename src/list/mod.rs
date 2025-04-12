mod builder;

use std::{collections::BTreeMap, path::PathBuf};

use builder::{build_default_list, deserialise_list, serialise_list};

use crate::{error::AnansiError, Task};

type TaskID = usize;

#[derive(Clone)]
pub struct List {
    file_path: PathBuf,
    tasks: BTreeMap<TaskID, Box<Task>>,
    open_tasks: Vec<TaskID>,
    done_tasks: Vec<TaskID>,
}

impl List {
    /// Checks if a task with the given id exists in the list.
    pub fn is_id_used(&self, id: TaskID) -> bool {
        self.tasks.contains_key(&id)
    }

    /// Returns the highest id used in the list or `None` if the list is empty.
    pub fn max_id(&self) -> Option<TaskID> {
        self.tasks.keys().max().copied()
    }

    /// Add a task to the list.
    ///
    /// # Returns
    /// Returns an error if the task id is already used.
    fn push_task(&mut self, task: Task) -> Result<(), AnansiError> {
        if !self.is_id_used(task.id()) {
            if task.is_done() {
                self.done_tasks.push(task.id());
            } else {
                self.open_tasks.push(task.id());
            }
            self.tasks.insert(task.id(), Box::new(task));
            Ok(())
        } else {
            return Err(AnansiError { title: "Invalid ID".to_string(), message: format!("Task with ID {} already exists", task.id()) });
        }
    }

    fn new_empty_with_path<P: Into<PathBuf>>(path: P) -> List {
        List {
            file_path: path.into(),
            tasks: BTreeMap::new(),
            open_tasks: Vec::new(),
            done_tasks: Vec::new(),
        }
    }

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
        let id = if let Some(id) = self.max_id() { id + 1 } else { 0 };
        let task: Task = Task::new(task.as_ref(), id);
        if task.is_done() {
            self.done_tasks.push(id);
        } else {
            self.open_tasks.push(id);
        }
        self.tasks.insert(id, Box::new(task));
    }

    // TODO: Optimise this - maybe don't load all tasks into memory, don't have a better idea though
    /// Remove a task from the list, done or open.
    ///
    /// Provide a `TaskID` to remove.
    ///
    /// # Example
    /// ```
    /// use anansi::{List, Task};
    /// let mut list = List::new("path/to/list.txt");
    /// list.add("Task 1");
    /// list.add("Task 2");
    /// assert_eq!(list.open().len(), 2);
    /// let task = list.open()[0].clone();
    /// list.remove(task.id());
    /// assert_eq!(list.open().len(), 1);
    /// ```
    pub fn remove<ID: Into<TaskID>>(&mut self, task_id: ID) {
        let id = task_id.into();
        self.done_tasks.retain(|t| *t != id);
        self.open_tasks.retain(|t| *t != id);
        self.tasks.remove(&id);
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
    /// list.update(Task::new("Task 3", task.id()), task.id());
    /// assert_eq!(list.get(task.id()).unwrap().original(), "Task 3");
    /// ```
    pub fn update(&mut self, new_task: Task, task_id: TaskID) -> Result<(), AnansiError> {
        if !self.is_id_used(task_id) || new_task.id() != task_id {
            return Err(AnansiError { title: "Invalid ID".to_string(), message: format!("Task with ID {} does not exist", task_id) });
        }
        self.open_tasks.retain(|t| *t != task_id);
        self.done_tasks.retain(|t| *t != task_id);
        self.tasks.insert(task_id, Box::new(new_task));
        Ok(())
    }

    /// Get a task by id.
    ///
    /// # Example
    /// ```
    /// use anansi::List;
    /// let mut list = List::new("path/to/list.txt");
    /// list.add("Task 1");
    /// list.add("Task 2");
    /// let task = list.get(0).unwrap();
    /// assert_eq!(task.original(), "Task 1");
    /// ```
    pub fn get(&self, id: TaskID) -> Option<Task> {
        let boxed_task = self.tasks.get(&id).cloned();
        boxed_task.map(|boxed_task| *boxed_task)
    }

    pub fn tasks(&self) -> Vec<Task> {
        let mut out = Vec::new();
        for task in self.tasks.values() {
            out.push(*task.clone());
        }
        out
    }

    /// Get all done tasks.
    ///
    /// # Example
    /// ```
    /// use anansi::List;
    /// let mut list = List::new("path/to/list.txt");
    /// list.add("x Task 1");
    /// list.add("Task 2");
    /// let done_tasks = list.done();
    /// assert_eq!(done_tasks.len(), 1);
    /// ```
    pub fn done(&self) -> Vec<Task> {
        self.done_tasks.iter().map(|id| self.get(*id).unwrap()).collect()
    }

    /// Get all open tasks.
    ///
    /// # Example
    /// ```
    /// use anansi::List;
    /// let mut list = List::new("path/to/list.txt");
    /// list.add("x Task 1");
    /// list.add("Task 2");
    /// let open_tasks = list.open();
    /// assert_eq!(open_tasks.len(), 1);
    /// ```
    pub fn open(&self) -> Vec<Task> {
        self.open_tasks.iter().map(|id| self.get(*id).unwrap()).collect()
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
    /// use anansi::List;
    ///
    /// let mut list = List::new("path/to/list.txt");
    /// list.add("(A) Task 1");
    /// list.add("(A) Task 2 @air");
    /// list.add("(B) Task 3 @AIR");
    /// list.add("(Z) Task 4 @AirCraft");
    /// let filtered = list.by_prio("a");
    /// assert_eq!(filtered.tasks().len(), 2);
    /// ```
    pub fn by_prio<S: Into<String>>(&self, prio: S) -> List {
        let prio = prio.into().to_uppercase();
        let mut filtered = Vec::new();
        for task in self.tasks.values() {
            if task.prio() == prio {
                filtered.push(*task.clone());
            }
        }
        let mut filtered_list = List::new_empty_with_path(self.file_path.clone());
        for task in filtered {
            let pos_err = filtered_list.push_task(task);
            debug_assert!(pos_err.is_ok());
        }
        filtered_list
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
    /// use anansi::List;
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
    pub fn by_context<S: Into<String>>(&self, context: S) -> List {
        let context = context.into();
        let mut filtered = Vec::new();
        for task in self.tasks.values() {
            for tag in task.contexts() {
                if tag.to_lowercase().contains(&context.to_lowercase()) {
                    filtered.push(*task.clone());
                    break
                }
            }
        }
        let mut filtered_list = List::new_empty_with_path(self.file_path.clone());
        for task in filtered {
            let pos_err = filtered_list.push_task(task);
            debug_assert!(pos_err.is_ok());
        }
        filtered_list
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
    /// use anansi::List;
    ///
    /// let mut list = List::new("path/to/list.txt");
    /// list.add("Task 1");
    /// list.add("Task 2 +home");
    /// list.add("Task 3 +HOME");
    /// list.add("x Task 4 +homeImprovements");
    ///
    /// assert_eq!(list.by_project("home").tasks().len(), 3);
    /// assert_eq!(list.by_project("improvements").tasks().len(), 1);
    pub fn by_project<S: Into<String>>(&self, project: S) -> List {
        let project = project.into();
        let mut filtered = Vec::new();
        for task in self.tasks.values() {
            for tag in task.projects() {
                if tag.to_lowercase().contains(&project.to_lowercase()) {
                    filtered.push(*task.clone());
                    break
                }
            }
        }
        let mut filtered_list = List::new_empty_with_path(self.file_path.clone());
        for task in filtered {
            let pos_err = filtered_list.push_task(task);
            debug_assert!(pos_err.is_ok());
        }
        filtered_list
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
    /// use anansi::List;
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
    pub fn by_special<S: Into<String>>(&self, special: S) -> List {
        let special = special.into();
        let mut filtered = Vec::new();
        for task in self.tasks.values() {
            for key in task.specials().keys() {
                if key.to_lowercase().contains(&special.to_lowercase()) {
                    filtered.push(*task.clone());
                    break
                }
            }
        }
        let mut filtered_list = List::new_empty_with_path(self.file_path.clone());
        for task in filtered {
            let pos_err = filtered_list.push_task(task);
            debug_assert!(pos_err.is_ok());
        }
        filtered_list
    }
}
