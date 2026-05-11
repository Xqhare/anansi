mod builder;
mod test;

use std::{collections::BTreeMap, ops::Deref, path::PathBuf};

use builder::{build_default_list, deserialise_list, serialise_list};

use crate::{Date, Task, error::AnansiResult, util::SortBy};

type TaskID = usize;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct List {
    file_path: PathBuf,
    tasks: BTreeMap<TaskID, Task>,
    open_tasks: Vec<TaskID>,
    done_tasks: Vec<TaskID>,
    max_id: Option<TaskID>,
}

impl From<Vec<Task>> for List {
    fn from(tasks: Vec<Task>) -> Self {
        let mut list = List::new_empty_with_path("");
        for task in tasks {
            list.push_task(task);
        }
        list
    }
}

impl List {
    /// Checks if a task with the given id exists in the list.
    pub fn is_id_used(&self, id: TaskID) -> bool {
        self.tasks.contains_key(&id)
    }

    /// Returns the next available id.
    pub fn max_id(&self) -> TaskID {
        self.max_id.unwrap_or(0) + 1
    }

    /// Returns the path to the list file.
    pub fn get_path(&self) -> &PathBuf {
        &self.file_path
    }

    /// Updates the path to the list file.
    pub fn update_path<P: Into<PathBuf>>(&mut self, path: P) -> List {
        self.file_path = path.into();
        self.clone()
    }

    /// Returns the amount of open tasks in the list.
    pub fn open_task_amount(&self) -> usize {
        self.open_tasks.len()
    }

    /// Returns the amount of done tasks in the list.
    pub fn done_task_amount(&self) -> usize {
        self.done_tasks.len()
    }

    /// Returns the amount of tasks in the list.
    pub fn task_amount(&self) -> usize {
        self.tasks.len()
    }

    /// Add a task to the list.
    ///
    /// Ignores the set `id` and uses the next available `id`.
    ///
    /// Consider using `add` instead. Same behaviour, but the input is `&str` instead of `Task`.
    pub fn push_task(&mut self, task: Task) -> TaskID {
        let id = self.max_id();
        let task = task.with_id(id);
        self.max_id = Some(id);
        if task.is_done() {
            self.done_tasks.push(id);
        } else {
            self.open_tasks.push(id);
        }
        self.tasks.insert(id, task);
        id
    }

    fn new_empty_with_path<P: Into<PathBuf>>(path: P) -> List {
        List {
            file_path: path.into(),
            tasks: BTreeMap::new(),
            open_tasks: Vec::new(),
            done_tasks: Vec::new(),
            max_id: None,
        }
    }

    /// Create a new list.
    /// If the supplied path exists, load the file and deserialize.
    /// If the supplied path does not exist, create a new, empty list.
    ///
    /// Will not error if the file cannot be read or does not exist.
    /// Consider using `load` instead if you want to handle these errors.
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

    /// Load a list from a file.
    ///
    /// Will error if the file cannot be read, or does not exist.
    ///
    /// If you do not want to handle these errors, and always fall back to a new list, use `new` instead.
    ///
    /// # Example
    /// ```
    /// use anansi::List;
    /// let mut list = List::load("non_existing_path/to/list.txt");
    /// assert!(list.is_err());
    /// ```
    pub fn load<P: Into<PathBuf>>(path: P) -> AnansiResult<List> {
        let file_path = path.into();
        match std::fs::read_to_string(&file_path) {
            Ok(file) => Ok(deserialise_list(file_path, file.trim())),
            Err(err) => Err(err.into()),
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
    pub fn add<S: AsRef<str>>(&mut self, task: S) -> TaskID {
        let id = self.max_id();
        let task = Task::new(task, id);
        self.push_task(task);
        id
    }

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
        let done_index = self.done_tasks.iter().position(|t| *t == id);
        let open_index = if done_index.is_some() {
            None
        } else {
            self.open_tasks.iter().position(|t| *t == id)
        };
        if let Some(open_index) = open_index {
            self.open_tasks.swap_remove(open_index);
        }
        if let Some(done_index) = done_index {
            self.done_tasks.swap_remove(done_index);
        }
        self.tasks.remove(&id);
    }

    /// Get a task by id.
    ///
    /// # Example
    /// ```
    /// use anansi::List;
    /// let mut list = List::new("path/to/list.txt");
    /// list.add("Task 1");
    /// list.add("Task 2");
    /// let task = list.get(1).unwrap();
    /// assert_eq!(task.to_string(), "Task 1");
    /// ```
    pub fn get(&self, id: TaskID) -> Option<&Task> {
        self.tasks.get(&id)
    }

    /// Sort tasks.
    ///
    /// Provide a `SortBy` to sort by.
    /// Possible values are:
    /// - `SortBy::Priority`
    /// - `SortBy::InceptionDate`
    /// - `SortBy::CompletionDate`
    ///
    /// # Example
    /// ```
    /// use anansi::{List, SortBy};
    /// let mut list = List::new("path/to/list.txt");
    /// list.add("(B) Task 1");
    /// list.add("(A) Task 2");
    /// let sorted_tasks = list.sort(SortBy::Priority);
    /// assert_eq!(sorted_tasks[0].to_string(), "(A) Task 2");
    /// assert_eq!(sorted_tasks[1].to_string(), "(B) Task 1");
    /// ```
    pub fn sort(self, sort_by: SortBy) -> Vec<Task> {
        match sort_by {
            SortBy::Priority => self.sort_priority(),
            SortBy::InceptionDate => self.sort_inception_date(),
            SortBy::CompletionDate => self.sort_completion_date(),
        }
    }

    fn sort_priority(self) -> Vec<Task> {
        let mut tasks = self.tasks();
        tasks.sort_by(|a, b| a.prio().cmp(&b.prio()));
        let mut no_prio = Vec::new();
        let mut with_prio = Vec::new();
        for task in tasks {
            if task.prio().is_none() {
                no_prio.push(task);
            } else {
                with_prio.push(task);
            }
        }
        with_prio.extend(no_prio);
        with_prio
    }

    fn sort_inception_date(self) -> Vec<Task> {
        let mut tasks = self.tasks();
        tasks.sort_by(|a, b| a.inception_date().cmp(&b.inception_date()));
        let mut no_date = Vec::new();
        let mut with_date = Vec::new();
        for task in tasks {
            if task.inception_date() == Date::default().to_string() {
                no_date.push(task);
            } else {
                with_date.push(task);
            }
        }
        with_date.extend(no_date);
        with_date
    }

    fn sort_completion_date(self) -> Vec<Task> {
        let mut tasks = self.tasks();
        tasks.sort_by(|a, b| a.completion_date().cmp(&b.completion_date()));
        let mut no_date = Vec::new();
        let mut with_date = Vec::new();
        for task in tasks {
            if task.completion_date() == Date::default().to_string() {
                no_date.push(task);
            } else {
                with_date.push(task);
            }
        }
        with_date.extend(no_date);
        with_date
    }

    /// Get all tasks.
    ///
    /// # Example
    /// ```
    /// use anansi::List;
    /// let mut list = List::new("path/to/list.txt");
    /// list.add("x Task 1");
    /// list.add("Task 2");
    /// let tasks = list.tasks();
    /// assert_eq!(tasks.len(), 2);
    /// ```
    pub fn tasks(&self) -> Vec<Task> {
        let mut out = Vec::with_capacity(self.tasks.len());
        for task in self.tasks.values() {
            out.push(task.clone());
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
        self.done_tasks
            .iter()
            .map(|id| self.get(*id).unwrap().clone())
            .collect()
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
        self.open_tasks
            .iter()
            .map(|id| self.get(*id).unwrap().clone())
            .collect()
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
    pub fn save(&self) -> AnansiResult<()> {
        let serialised = serialise_list(self);
        match std::fs::write(&self.file_path, serialised) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
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
        let prio = prio
            .into()
            .to_uppercase()
            .chars()
            .next()
            .expect("prio should not be empty");
        let mut filtered = Vec::new();
        for task in self.tasks.values() {
            if let Some(task_prio) = task.prio()
                && task_prio == prio
            {
                filtered.push(task.clone());
            }
        }
        let mut filtered_list = List::new_empty_with_path(self.file_path.clone());
        for task in filtered {
            filtered_list.push_task(task);
        }
        filtered_list
    }

    /// Filter tasks by priority.
    ///
    /// Will filter both open and done tasks.
    /// This will filter by checking if the predicate is contained within any priority tag.
    ///
    /// The filter is case insensitive.
    ///
    /// # Returns
    /// A vector of tasks.
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
    /// let filtered = list.by_prio_to_vec('a');
    /// assert_eq!(filtered.len(), 2);
    /// ```
    pub fn by_prio_to_vec(&self, prio: char) -> Vec<Task> {
        let prio = prio.to_uppercase().take(1).next().unwrap();
        let mut filtered = Vec::new();
        for task in self.tasks.values() {
            if let Some(task_prio) = task.prio()
                && task_prio == prio
            {
                filtered.push(task.clone());
            }
        }
        filtered
    }

    /// Filter tasks by text.
    ///
    /// Will filter both open and done tasks.
    /// This will filter by checking if the predicate is contained within the task text.
    ///
    /// The filter is case insensitive and also matches partial text.
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
    /// let filtered = list.by_text("task");
    /// assert_eq!(filtered.tasks().len(), 4);
    /// let filtered = list.by_text("air");
    /// assert_eq!(filtered.tasks().len(), 3);
    /// ```
    pub fn by_text<S: Into<String>>(&self, text: S) -> List {
        let text = text.into().to_lowercase();
        let mut filtered = Vec::new();
        for task in self.tasks.values() {
            if task.text().to_lowercase().contains(&text) {
                filtered.push(task.clone());
            }
        }
        let mut filtered_list = List::new_empty_with_path(self.file_path.clone());
        for task in filtered {
            filtered_list.push_task(task);
        }
        filtered_list
    }

    /// Filter tasks by text.
    ///
    /// Will filter both open and done tasks.
    /// This will filter by checking if the predicate is contained within the task text.
    ///
    /// The filter is case insensitive and also matches partial text.
    ///
    /// # Returns
    /// A vector of tasks.
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
    /// let filtered = list.by_text_to_vec("task");
    /// assert_eq!(filtered.len(), 4);
    /// let filtered = list.by_text_to_vec("air");
    /// assert_eq!(filtered.len(), 3);
    /// ```
    pub fn by_text_to_vec(&self, text: &str) -> Vec<Task> {
        let text = text.to_lowercase();
        let mut filtered = Vec::new();
        for task in self.tasks.values() {
            if task.text().to_lowercase().contains(&text) {
                filtered.push(task.clone());
            }
        }
        filtered
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
                    filtered.push(task.clone());
                    break;
                }
            }
        }
        let mut filtered_list = List::new_empty_with_path(self.file_path.clone());
        for task in filtered {
            filtered_list.push_task(task);
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
                    filtered.push(task.clone());
                    break;
                }
            }
        }
        let mut filtered_list = List::new_empty_with_path(self.file_path.clone());
        for task in filtered {
            filtered_list.push_task(task);
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
                    filtered.push(task.clone());
                    break;
                }
            }
        }
        let mut filtered_list = List::new_empty_with_path(self.file_path.clone());
        for task in filtered {
            filtered_list.push_task(task);
        }
        filtered_list
    }
}

/// Sort a vector of tasks in place.
///
/// # Example
/// ```
/// use anansi::{List, vec::sort_vec_task, SortBy};
/// let mut list = List::new("path/to/list.txt");
/// list.add("(A) Task 1");
/// list.add("(B) Task 2");
/// list.add("(C) Task 3");
/// list.add("(D) Task 4");
/// list.add("(E) Task 5");
/// list.add("(F) Task 6");
/// list.add("(G) Task 7");
/// list.add("(H) Task 8");
/// list.add("(I) Task 9");
/// let mut sorted_tasks = list.sort(SortBy::Priority);
/// sort_vec_task(&mut sorted_tasks, SortBy::CompletionDate);
/// assert_eq!(sorted_tasks[0].to_string(), "(A) Task 1");
/// assert_eq!(sorted_tasks[1].to_string(), "(B) Task 2");
/// ```
pub fn sort_vec_task(to_sort: &mut Vec<Task>, sort_by: SortBy) {
    match sort_by {
        SortBy::Priority => {
            to_sort.sort_by(|a, b| a.prio().cmp(&b.prio()));
            let mut no_prio = Vec::new();
            let mut with_prio = Vec::new();
            let len = to_sort.len();
            for task in to_sort.iter() {
                if task.prio().is_none() {
                    no_prio.push(task);
                } else {
                    with_prio.push(task);
                }
            }
            with_prio.extend(no_prio);
            let mut out = Vec::with_capacity(len);
            for task in with_prio {
                out.push(task.clone());
            }
            *to_sort = out
        }
        SortBy::InceptionDate => {
            to_sort.sort_by(|a, b| a.inception_date().cmp(&b.inception_date()));
            let mut no_date = Vec::new();
            let mut with_date = Vec::new();
            let len = to_sort.len();
            for task in to_sort.iter() {
                if task.inception_date() == Date::default().to_string() {
                    no_date.push(task);
                } else {
                    with_date.push(task);
                }
            }
            with_date.extend(no_date);
            let mut out = Vec::with_capacity(len);
            for task in with_date {
                out.push(task.clone());
            }
            *to_sort = out
        }
        SortBy::CompletionDate => {
            to_sort.sort_by(|a, b| a.completion_date().cmp(&b.completion_date()));
            let mut no_date = Vec::new();
            let mut with_date = Vec::new();
            let len = to_sort.len();
            for task in to_sort.iter() {
                if task.completion_date() == Date::default().to_string() {
                    no_date.push(task);
                } else {
                    with_date.push(task);
                }
            }
            with_date.extend(no_date);
            let mut out = Vec::with_capacity(len);
            for task in with_date {
                out.push(task.clone());
            }
            *to_sort = out
        }
    }
}

/// Search a vector of tasks.
///
/// # Returns
/// A vector of tasks matching the search string in their priority.
pub fn search_vec_task_prio(to_search: &Vec<Task>, search: &str) -> Vec<Task> {
    let search = search.to_lowercase();
    let mut filtered = Vec::new();
    for task in to_search {
        if task.text().to_lowercase().contains(&search) {
            filtered.push(task.clone());
        }
    }
    filtered
}

/// Search a vector of tasks.
///
/// # Returns
/// A vector of tasks matching the search string in their text.
pub fn search_vec_task_text(to_search: &Vec<Task>, search: &str) -> Vec<Task> {
    let search = search.to_lowercase();
    let mut filtered = Vec::new();
    for task in to_search {
        if task.text().to_lowercase().contains(&search) {
            filtered.push(task.clone());
        }
    }
    filtered
}
