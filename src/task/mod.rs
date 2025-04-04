mod builder;
mod test;

use std::collections::BTreeMap;

use builder::deserialize_task;

use crate::{error::AnansiError, Date};

/// Represents a single task.
/// A task is a single line in the todo.txt file.
///
/// To build a task, you can either use `Task::new(Into<String>)` or `Task::from<&str>`.
/// The supplied text will be deserialised according to the 'todo.txt' format.
///
/// You cannot interact with the underlying data of a task directly, but you can use `Task::update()` for the same effect.
/// Any data supplied to the `update()` function will be deserialised just like with `Task::new()`.
/// - You can think of it more of a `overwrite` function as all costs associated with construction apply as well.
///
/// Task implements Ordering and Equivalence traits.
/// Tasks are ordered and sorted by their priority only, with the highest priority being higher and thus bigger.
/// So `A > B` and `B > C`, put generally `A > Z`.
///
/// The equality however takes the entire task text into account.
///
/// # Example
/// ```
/// use anansi::Task;
///
/// let task1 = Task::new("(A) test", 0);
/// assert_eq!(task1.is_done(), false);
/// assert_eq!(task1.prio(), "A");
/// assert_eq!(task1.completion_date(), "");
/// assert_eq!(task1.inception_date(), "");
/// assert_eq!(task1.text(), "test");
/// assert_eq!(task1.description(), "test");
/// assert!(task1.projects().is_empty());
/// assert!(task1.contexts().is_empty());
/// assert!(task1.specials().is_empty());
/// assert_eq!(task1.original(), "(A) test");
///
/// let task2 = Task::new("x (A) test", 0);
/// let task3 = Task::new("(Z) test", 0);
/// let task4 = Task::new("test", 0);
/// let task5 = Task::new("(A) test", 0);
/// assert!(task4 > task2);
/// assert!(task2 > task3 && task4 > task2);
/// assert!(task1 >= task2);
/// assert!(task4 != task2);
/// assert!(task1 == task5);
/// ```
#[derive(Debug, Clone)]
pub struct Task {
    id: usize,
    // storing this boolean saves loading the original text and checking if it starts with 'x'
    done: bool,
    // takes up max 4 bytes, no reason to optimise into a u8
    priority: Option<char>,
    completion_date: Date,
    inception_date: Date,
    // The text of the task, with the tags but without the head (prio, dates, done)
    text: String,
    // The description of the task, without the tags and head
    description: String,
    context_tags: Vec<String>,
    project_tags: Vec<String>,
    // storing key-value pairs for special tags
    special_tags: BTreeMap<String, String>,
    // complete text
    original_text: String,
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.original_text == other.original_text
    }

    fn ne(&self, other: &Self) -> bool {
        self.original_text != other.original_text
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
    ///
    /// Also needs an id of type `usize`.
    /// To add a Task to a TaskList, please use `TaskList::add()` instead.
    /// To update a Task inside a TaskList, please use `TaskList::update()` instead.
    ///
    /// Do not use this constructor directly if you want to add a task to a TaskList.
    pub fn new<S: AsRef<str>>(text: S, id: usize) -> Task {
        deserialize_task(text, id)
    }

    /* /// Updates the task with the given text.
    ///
    /// Input will be deserialised according to the 'todo.txt' format.
    pub fn update<S: AsRef<str>>(&mut self, text: S) {
        let new_task = deserialize_task(text, self.id);
        self.done = new_task.done;
        self.priority = new_task.priority;
        self.completion_date = new_task.completion_date;
        self.inception_date = new_task.inception_date;
        self.text = new_task.text;
        self.context_tags = new_task.context_tags;
        self.project_tags = new_task.project_tags;
        self.special_tags = new_task.special_tags;
        self.original_text = new_task.original_text;
    } */

    /// Returns `true` if the task is done.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Returns the id of the task.
    pub fn id(&self) -> usize {
        self.id
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
                    task.original_text = format!("x ({}) {} {} {}", task.prio(), date, task.inception_date, task.text);
                    task.completion_date = date;
                } else {
                    return Err(
                        AnansiError { 
                            title: "Missing completion date".to_string(), 
                            message: "If a Task has a inception date set, the standard requires a completion date to be set as well.".to_string() 
                        });
                }
            } else {
                task.original_text = format!("x ({}) {}", task.prio(), task.text);
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
                task.original_text = format!("({}) {} {}", task.prio(), task.inception_date, task.text);
            } else {
                task.original_text = task.original_text.replacen("x ", "", 1);
            }
        }
        task
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
    pub fn text(&self) -> &String {
        &self.text
    }

    /// Returns the description of the task.
    /// This includes tags, but excludes dates, priority and done status.
    ///
    /// # Example
    /// 
    /// ```
    /// use anansi::Task;
    /// 
    /// let task = anansi::Task::new("x (A) 2022-11-11 2022-01-01 test +proj @cont key:val", 0);
    /// assert_eq!(task.description(), "test");
    /// ```
    pub fn description(&self) -> &String {
        &self.description
    }

    /// Returns the original text of the task.
    /// This includes dates, priority and done status.
    ///
    /// # Example
    /// 
    /// ```
    /// use anansi::Task;
    /// 
    /// let task = anansi::Task::new("x (A) 2022-11-11 2022-01-01 test +proj @cont key:val", 0);
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

impl From<(&str, usize)> for Task {
    fn from(value: (&str, usize)) -> Self {
        Task::new(value.0, value.1)
    }
}

