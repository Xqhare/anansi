use std::collections::BTreeMap;

use crate::{AnansiError, Date, Task};

impl Task {
    /// Updates the id of the task.
    ///
    /// Does not update the id of the task in the `TaskList`.
    /// Use with caution.
    pub fn update_id(&mut self, id: usize) {
        self.id = id;
    }
    /// Updates the priority of the task.
    ///
    /// # Example
    ///
    /// ```
    /// use anansi::Task;
    ///
    /// let mut task = anansi::Task::new("x (A) test", 0);
    /// task.update_prio('B');
    /// assert_eq!(task.prio(), Some('B'));
    /// ```
    pub fn update_prio(&mut self, new_prio: char) {
        self.priority = Some(new_prio);
    }
    /// Updates the context tags of the task.
    ///
    /// This overwrites the existing context tags.
    ///
    /// # Example
    ///
    /// ```
    /// use anansi::Task;
    ///
    /// let mut task = anansi::Task::new("x (A) test", 0);
    /// task.update_context_tags(vec!["air".into()]);
    /// assert_eq!(task.contexts().len(), 1);
    /// ```
    pub fn update_context_tags(&mut self, new_context: Vec<String>) {
        self.context_tags = new_context;
    }
    /// Updates the project tags of the task.
    ///
    /// This overwrites the existing project tags.
    ///
    /// # Example
    ///
    /// ```
    /// use anansi::Task;
    ///
    /// let mut task = anansi::Task::new("x (A) test", 0);
    /// task.update_project_tags(vec!["air".into()]);
    /// assert_eq!(task.projects().len(), 1);
    /// ```
    pub fn update_project_tags(&mut self, new_project: Vec<String>) {
        self.project_tags = new_project;
    }
    /// Updates the special tags of the task.
    ///
    /// This overwrites the existing special tags.
    ///
    /// # Example
    ///
    /// ```
    /// use anansi::Task;
    ///
    /// let mut task = anansi::Task::new("x (A) test", 0);
    /// task.update_special_tags(vec![("due".into(), "2020-01-01".into())].into_iter().collect());
    /// assert_eq!(task.specials().len(), 1);
    /// ```
    pub fn update_special_tags(&mut self, new_special: BTreeMap<String, String>) {
        self.special_tags = new_special;
    }
    /// Adds a context tag.
    ///
    /// # Example
    ///
    /// ```
    /// use anansi::Task;
    ///
    /// let mut task = anansi::Task::new("x (A) test", 0);
    /// task.add_context_tag("air".into());
    /// assert_eq!(task.contexts().len(), 1);
    /// ```
    pub fn add_context_tag(&mut self, new_context: String) {
        self.context_tags.push(new_context);
    }
    /// Adds a project tag.
    ///
    /// # Example
    ///
    /// ```
    /// use anansi::Task;
    ///
    /// let mut task = anansi::Task::new("x (A) test", 0);
    /// task.add_project_tag("air".into());
    /// assert_eq!(task.projects().len(), 1);
    /// ```
    pub fn add_project_tag(&mut self, new_project: String) {
        self.project_tags.push(new_project);
    }
    /// Adds a special tag.
    ///
    /// # Example
    ///
    /// ```
    /// use anansi::Task;
    ///
    /// let mut task = anansi::Task::new("x (A) test", 0);
    /// task.add_special_tag("due".into(), "2020-01-01".into());
    /// assert_eq!(task.specials().len(), 1);
    /// ```
    pub fn add_special_tag(&mut self, key: String, value: String) {
        self.special_tags.insert(key, value);
    }
    /// Removes a context tag.
    ///
    /// # Example
    ///
    /// ```
    /// use anansi::Task;
    ///
    /// let mut task = anansi::Task::new("x (A) test", 0);
    /// task.add_context_tag("air".into());
    /// assert_eq!(task.contexts().len(), 1);
    /// task.remove_context_tag("air".into());
    /// assert_eq!(task.contexts().len(), 0);
    /// ```
    pub fn remove_context_tag(&mut self, tag: String) {
        self.context_tags.retain(|t| t != &tag);
    }
    /// Removes a project tag.
    ///
    /// # Example
    ///
    /// ```
    /// use anansi::Task;
    ///
    /// let mut task = anansi::Task::new("x (A) test", 0);
    /// task.add_project_tag("air".into());
    /// assert_eq!(task.projects().len(), 1);
    /// task.remove_project_tag("air".into());
    /// assert_eq!(task.projects().len(), 0);
    /// ```
    pub fn remove_project_tag(&mut self, tag: String) {
        self.project_tags.retain(|t| t != &tag);
    }
    /// Removes a special tag by key.
    ///
    /// # Example
    ///
    /// ```
    /// use anansi::Task;
    ///
    /// let mut task = anansi::Task::new("x (A) test", 0);
    /// task.add_special_tag("due".into(), "2022-11-11".into());
    /// assert_eq!(task.specials().len(), 1);
    /// task.remove_special_tag("due".into());
    /// assert_eq!(task.specials().len(), 0);
    /// ```
    pub fn remove_special_tag(&mut self, key: String) {
        self.special_tags.remove(&key);
    }
    /// Updates the completion date of the task.
    ///
    /// # Returns
    ///
    /// Returns an error if the task is not done.
    ///
    /// # Example
    ///
    /// ```
    /// use anansi::Task;
    ///
    /// let mut task = anansi::Task::new("x (A) test", 0);
    /// assert!(task.update_completion_date("2022-11-11").is_ok());
    /// assert_eq!(task.completion_date(), "2022-11-11");
    /// let mut undone_task = anansi::Task::new("(A) test", 0);
    /// assert!(undone_task.update_completion_date("2022-11-11").is_err());
    /// ```
    pub fn update_completion_date<D: Into<Date>>(
        &mut self,
        new_date: D,
    ) -> Result<(), AnansiError> {
        if !self.done {
            return Err(AnansiError::MissingCompletionDate(self.id));
        }
        self.completion_date = new_date.into();
        Ok(())
    }
    /// Updates the inception date of the task.
    ///
    /// # Example
    /// ```
    /// use anansi::Task;
    ///
    /// let mut task = anansi::Task::new("(A) 2022-01-01 test", 0);
    /// task.update_inception_date("2022-11-11");
    /// assert_eq!(task.inception_date(), "2022-11-11");
    /// ```
    pub fn update_inception_date<D: Into<Date>>(&mut self, new_date: D) {
        self.inception_date = new_date.into();
    }
    /// Updates the text of the task.
    ///
    /// The task will be updated to reflect the new text.
    ///
    /// # Example
    ///
    /// ```
    /// use anansi::Task;
    ///
    /// let mut task = anansi::Task::new("x (A) 2022-01-01 2022-11-11 test", 0);
    /// task.update_text("new text");
    /// assert_eq!(task.text(), "new text");
    /// assert_eq!(task.is_done(), true);
    /// assert_eq!(task.prio(), Some('A'));
    /// assert_eq!(task.completion_date(), "2022-11-11");
    /// assert_eq!(task.inception_date(), "2022-01-01");
    /// ```
    pub fn update_text<S: Into<String>>(&mut self, new_text: S) {
        let new_text = new_text.into();
        self.text = new_text.clone();
        let new_task = Task::from((self.to_string().as_str(), 0usize));
        self.original_text = new_task.original_text;
        self.context_tags = new_task.context_tags;
        self.project_tags = new_task.project_tags;
        self.special_tags = new_task.special_tags;
    }
    /// Marks the task as done.
    ///
    /// If a completion date is given, it will be stored in the task.
    ///
    /// Should you supply a completion date and the inception date is not set, the supplied
    /// date will be dropped.
    ///
    /// To supply a date, simply format it as `YYYY-MM-DD` and call `.into()` as you see in the
    /// example.
    ///
    /// If a task has already been marked as done, nothing will happen.
    ///
    /// # Arguments
    ///
    /// * `completion_date` - The date the task has been completed.
    ///
    /// # Example
    ///
    /// ```
    /// use anansi::Task;
    ///
    /// let task = anansi::Task::new("(A) 2022-01-01 test", 0);
    /// assert_eq!(task.is_done(), false);
    /// assert_eq!(task.inception_date(), "2022-01-01");
    /// let done = task.done(Some("2022-11-11".into())).unwrap();
    /// assert_eq!(done.is_done(), true);
    /// assert_eq!(done.completion_date(), "2022-11-11");
    /// ```
    ///
    /// ```
    /// # use anansi::Task;
    /// let task = anansi::Task::new("(A) 2022-01-01 test", 0);
    /// let done = task.done(None);
    /// assert_eq!(done.is_ok(), false);
    /// ```
    ///
    pub fn done(&self, completion_date: Option<Date>) -> Result<Self, AnansiError> {
        let mut task = self.clone();
        if !task.done {
            task.done = true;
            if task.inception_date.is_set() {
                if let Some(date) = completion_date {
                    task.original_text = format!(
                        "x ({}) {} {} {}",
                        task.prio().unwrap_or(' '),
                        date,
                        task.inception_date,
                        task.text
                    );
                    task.completion_date = date;
                } else {
                    return Err(AnansiError::MissingCompletionDate(self.id));
                }
            } else {
                task.original_text = format!("x ({}) {}", task.prio().unwrap_or(' '), task.text);
            }
        }
        Ok(task)
    }
    /// Marks the task as undone.
    ///
    /// Will remove any inception date if there is one.
    /// If a task has already been marked as undone, nothing will happen.
    ///
    /// # Returns
    /// Returns a copy of the task, which is now no longer marked as done.
    ///
    /// # Example
    ///
    /// ```
    /// use anansi::Task;
    ///
    /// let task = anansi::Task::new("x (A) 2022-11-11 2022-01-01 test", 0);
    /// assert_eq!(task.is_done(), true);
    /// assert_eq!(task.completion_date(), "2022-11-11");
    /// let undone = task.undone();
    /// assert_eq!(undone.is_done(), false);
    /// assert_eq!(undone.completion_date(), "");
    /// ```
    ///
    pub fn undone(&self) -> Self {
        let mut task = self.clone();
        if task.done {
            task.done = false;
            if task.completion_date.is_set() {
                task.completion_date = Date::default();
                task.original_text = format!(
                    "({}) {} {}",
                    task.prio().unwrap_or(' '),
                    task.inception_date,
                    task.text
                );
            } else {
                task.original_text = task.original_text.replacen("x ", "", 1);
            }
        }
        task
    }
}
