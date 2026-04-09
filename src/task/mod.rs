mod accessors;
mod builder;
mod mutators;
mod test;

use std::collections::BTreeMap;

use builder::deserialize_task;

use crate::Date;

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
/// assert_eq!(task1.prio(), Some('A'));
/// assert_eq!(task1.completion_date(), "");
/// assert_eq!(task1.inception_date(), "");
/// assert_eq!(task1.text(), "test");
/// assert!(task1.projects().is_empty());
/// assert!(task1.contexts().is_empty());
/// assert!(task1.specials().is_empty());
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
    done: bool,
    priority: Option<char>,
    completion_date: Date,
    inception_date: Date,
    // The text of the task, with the tags but without the head (prio, dates, done)
    text: String,
    context_tags: Vec<String>,
    project_tags: Vec<String>,
    special_tags: BTreeMap<String, String>,
    // complete text (including `x` dates etc.)
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
        // Prio is always ASCII
        let self_prio_as_int = self.prio().unwrap_or(' ') as u8;
        let other_prio_as_int = other.prio().unwrap_or(' ') as u8;
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
    /// To add a Task to a `TaskList`, please use `TaskList::add()` instead.
    /// To update a Task inside a `TaskList`, please use `TaskList::update()` instead.
    ///
    /// Do not use this constructor directly if you want to add a task to a `TaskList`.
    pub fn new<S: AsRef<str>>(text: S, id: usize) -> Task {
        deserialize_task(text, id)
    }

    /// Returns the id of the task.
    pub fn id(&self) -> usize {
        self.id
    }
    /// Updates the id of the task and returns the updated task.
    pub fn with_id(mut self, id: usize) -> Task {
        self.id = id;
        self
    }
}

// ---------------------------------------------------------------
//                        Display implementation
// ---------------------------------------------------------------
impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.original_text)
    }
}

impl From<(&str, usize)> for Task {
    fn from(value: (&str, usize)) -> Self {
        Task::new(value.0, value.1)
    }
}
