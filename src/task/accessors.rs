use std::collections::BTreeMap;

use crate::Task;

impl Task {
    /// Returns `true` if the task is done.
    pub fn is_done(&self) -> bool {
        self.done
    }
    /// Returns the priority of the task.
    ///
    /// If the task has no priority, an empty string will be returned.
    ///
    /// # Example
    ///
    /// ```
    /// use anansi::Task;
    ///
    /// let task = anansi::Task::new("(A) test", 0);
    /// assert_eq!(task.prio(), Some('A'));
    /// ```
    pub fn prio(&self) -> Option<char> {
        self.priority
    }
    /// Returns all context tags of the task.
    ///
    /// # Example
    ///
    /// ```
    /// use anansi::Task;
    ///
    /// let task = anansi::Task::new("(A) test @air @home", 0);
    /// assert_eq!(task.contexts(), &vec!["air", "home"]);
    /// ```
    pub fn contexts(&self) -> &Vec<String> {
        &self.context_tags
    }
    /// Returns all project tags of the task.
    ///
    /// # Example
    ///
    /// ```
    /// use anansi::Task;
    ///
    /// let task = anansi::Task::new("(A) test +air +home", 0);
    /// assert_eq!(task.projects(), &vec!["air", "home"]);
    /// ```
    pub fn projects(&self) -> &Vec<String> {
        &self.project_tags
    }
    /// Returns all special tags of the task as a map.
    ///
    /// # Example
    ///
    /// ```
    /// use anansi::Task;
    ///
    /// let task = anansi::Task::new("(A) test due:tomorrow", 0);
    /// assert_eq!(task.specials().get("due").unwrap(), "tomorrow");
    /// ```
    pub fn specials(&self) -> &BTreeMap<String, String> {
        &self.special_tags
    }
    /// Returns the completion date of the task.
    ///
    /// If the task is not done or the completion date is not set, an empty string will be returned.
    ///
    /// # Example
    ///
    /// ```
    /// use anansi::Task;
    ///
    /// let task = anansi::Task::new("x (A) 2022-11-11 2022-01-01 test", 0);
    /// assert_eq!(task.completion_date(), "2022-11-11");
    /// ```
    pub fn completion_date(&self) -> String {
        self.completion_date.format_date()
    }
    /// Returns the inception date of the task.
    ///
    /// If the task is not done or the inception date is not set, an empty string will be returned.
    ///
    /// # Example
    ///
    /// ```
    /// use anansi::Task;
    ///
    /// let task = anansi::Task::new("x (A) 2022-11-11 2022-01-01 test", 0);
    /// assert_eq!(task.inception_date(), "2022-01-01");
    /// ```
    pub fn inception_date(&self) -> String {
        self.inception_date.format_date()
    }
    /// Returns the text of the task.
    /// This includes tags, but excludes dates, priority and done status.
    ///
    /// # Example
    ///
    /// ```
    /// use anansi::Task;
    ///
    /// let task = anansi::Task::new("x (A) 2022-11-11 2022-01-01 test +proj @cont key:val", 0);
    /// assert_eq!(task.text(), "test +proj @cont key:val");
    /// ```
    pub fn text(&self) -> &str {
        &self.text
    }
}
