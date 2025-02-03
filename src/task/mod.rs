mod builder;
mod test;

use std::collections::BTreeMap;

use builder::deserialize_task;

use crate::Date;

#[derive(Debug, Clone)]
pub struct Task {
    // storing this boolean saves loading the original text and checking if it starts with 'x'
    done: bool,
    // takes up max 4 bytes, no reason to optimise into a u8
    priority: Option<char>,
    completion_date: Date,
    inception_date: Date,
    text: String,
    context_tags: Vec<String>,
    project_tags: Vec<String>,
    // storing key-value pairs for special tags
    special_tags: BTreeMap<String, String>,
    original_text: String,
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.prio() == other.prio()
    }

    fn ne(&self, other: &Self) -> bool {
        self.prio() != other.prio()
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_prio_as_int = self.prio().into_bytes();
        let other_prio_as_int = other.prio().into_bytes();
        // IMPORTANT: A has highest priority, but is the smallest byte value
        // so we flip the comparison
        other_prio_as_int.partial_cmp(&self_prio_as_int)
    }

    fn lt(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(std::cmp::Ordering::Less)
    }

    fn le(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(std::cmp::Ordering::Less)
            || self.partial_cmp(other) == Some(std::cmp::Ordering::Equal)
    }

    fn gt(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(std::cmp::Ordering::Greater)
    }

    fn ge(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(std::cmp::Ordering::Greater)
            || self.partial_cmp(other) == Some(std::cmp::Ordering::Equal)
    }
}

impl Task {
    /// Creates a new task from the given text.
    ///
    /// Input will be deserialised according to the 'todo.txt' format.
    pub fn new<S: AsRef<str>>(text: S) -> Task {
        deserialize_task(text)
    }

    /// Updates the task with the given text.
    ///
    /// Input will be deserialised according to the 'todo.txt' format.
    pub fn update<S: AsRef<str>>(&mut self, text: S) {
        let new_task = deserialize_task(text);
        self.done = new_task.done;
        self.priority = new_task.priority;
        self.completion_date = new_task.completion_date;
        self.inception_date = new_task.inception_date;
        self.text = new_task.text;
        self.context_tags = new_task.context_tags;
        self.project_tags = new_task.project_tags;
        self.special_tags = new_task.special_tags;
        self.original_text = new_task.original_text;
    }

    /// Returns `true` if the task is done.
    pub fn is_done(&self) -> bool {
        self.done
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
    /// # Example
    /// 
    /// ```
    /// use anansi::Task;
    /// 
    /// let mut task = anansi::Task::new("(A) 2022-01-01 test");
    /// assert_eq!(task.is_done(), false);
    /// assert_eq!(task.inception_date(), "2022-01-01");
    /// task.done(Some("2022-11-11".into()));
    /// assert_eq!(task.is_done(), true);
    /// assert_eq!(task.completion_date(), "2022-11-11");
    /// ```
    ///
    pub fn done(&mut self, completion_date: Option<Date>) {
        if !self.done {
            self.done = true;
            if self.inception_date.is_set() {
                if let Some(date) = completion_date {
                    self.original_text = format!("x ({}) {} {} {}", self.prio(), date, self.inception_date, self.text);
                    self.completion_date = date;
                } else {
                    self.original_text = format!("x {}", self.original_text);
                }
            } else {
                self.original_text = format!("x ({}) {}", self.prio(), self.text);
            }
        }
    }

    /// Marks the task as undone.
    ///
    /// If a task has already been marked as undone, nothing will happen.
    ///
    /// # Example
    /// 
    /// ```
    /// use anansi::Task;
    /// 
    /// let mut task = anansi::Task::new("x (A) 2022-11-11 2022-01-01 test");
    /// assert_eq!(task.is_done(), true);
    /// assert_eq!(task.completion_date(), "2022-11-11");
    /// task.undone();
    /// assert_eq!(task.is_done(), false);
    /// assert_eq!(task.completion_date(), "");
    /// ```
    ///
    pub fn undone(&mut self) {
        if self.done {
            self.done = false;
            if self.completion_date.is_set() {
                self.completion_date = Date::default();
                self.original_text = format!("({}) {} {}", self.prio(), self.inception_date, self.text);
            } else {
                self.original_text = self.original_text.replacen("x ", "", 1);
            }
        }
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
    /// let task = anansi::Task::new("(A) test");
    /// assert_eq!(task.prio(), "A");
    /// ```
    pub fn prio(&self) -> String {
        if let Some(priority) = self.priority {
            priority.to_string()
        } else {
            String::new()
        }
    }

    /// Returns all context tags of the task.
    ///
    /// # Example
    /// 
    /// ```
    /// use anansi::Task;
    /// 
    /// let task = anansi::Task::new("(A) test @air @home");
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
    /// let task = anansi::Task::new("(A) test +air +home");
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
    /// let task = anansi::Task::new("(A) test due:tomorrow");
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
    /// let task = anansi::Task::new("x (A) 2022-11-11 2022-01-01 test");
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
    /// let task = anansi::Task::new("x (A) 2022-11-11 2022-01-01 test");
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
    /// let task = anansi::Task::new("x (A) 2022-11-11 2022-01-01 test +proj @cont key:val");
    /// assert_eq!(task.text(), "test +proj @cont key:val");
    /// ```
    pub fn text(&self) -> &String {
        &self.text
    }

    /// Returns the original text of the task.
    /// This includes dates, priority and done status.
    ///
    /// # Example
    /// 
    /// ```
    /// use anansi::Task;
    /// 
    /// let task = anansi::Task::new("x (A) 2022-11-11 2022-01-01 test +proj @cont key:val");
    /// assert_eq!(task.original(), "x (A) 2022-11-11 2022-01-01 test +proj @cont key:val");
    /// ```
    pub fn original(&self) -> &String {
        &self.original_text
    }
}

// ---------------------------------------------------------------
//                        Display implementation
// ---------------------------------------------------------------
impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.original())
    }
}

impl From<&str> for Task {
    fn from(text: &str) -> Self {
        deserialize_task(text)
    }
}

