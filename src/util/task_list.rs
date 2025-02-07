use crate::Task;

/// Represents a filtered list of tasks.
///
/// Never constructed directly.
///
/// Can be obtained with any `by_*` method, be it inside the `List` or `TaskList`.
///
/// If you want to update an existing task, make sure to use `Task::update()` on a task within `List` and not `TaskList`.
#[derive(Debug, Clone)]
pub struct TaskList {
    tasks: Vec<Task>,
}

impl PartialEq for TaskList {
    fn eq(&self, other: &Self) -> bool {
        if self.tasks.len() != other.tasks.len() {
            return false;
        }
        for (i, task) in self.tasks.iter().enumerate() {
            if task.original() == other.tasks[i].original() {
                return true;
            }
        }
        return false;
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl TaskList {
    /// Create a new task list from a vector of tasks
    pub fn new(tasks: Vec<Task>) -> TaskList {
        TaskList { tasks }
    }

    /// Get all tasks
    pub fn tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    /// Filter tasks by priority
    /// 
    /// Will filter both open and done tasks
    /// This will filter by checking if the predicate is contained within any priority tag
    ///
    /// The filter is case insensitive.
    ///
    /// Can be chained with any other `by_*` method.
    ///
    /// # Example
    /// ```
    /// use anansi::{List, TaskList};
    ///
    /// let mut list = List::new("list.txt");
    /// list.add("(A) Task 1");
    /// list.add("(A) Task 2 @air");
    /// list.add("(B) Task 3 @AIR");
    /// list.add("(Z) Task 4 @AirCraft");
    /// let filtered = list.by_prio("a");
    /// assert_eq!(filtered.tasks().len(), 2);
    /// ```
    pub fn by_prio<S: Into<String>>(&self, prio: S) -> TaskList {
        let prio = prio.into().to_uppercase();
        TaskList::new(self.tasks
            .iter()
            .filter(|task| task.prio() == prio)
            .cloned()
            .collect()
        )
    }

    /// Filter tasks by context
    /// 
    /// Will filter both open and done tasks
    /// This will filter by checking if the predicate is contained within any context tag
    ///
    /// The filter is case insensitive.
    ///
    /// Can be chained with any other `by_*` method.
    ///
    /// # Example
    /// ```
    /// use anansi::{List, TaskList};
    /// 
    /// let mut list = List::new("list.txt");
    /// list.add("Task 1");
    /// list.add("Task 2 @air");
    /// list.add("Task 3 @AIR");
    /// list.add("x Task 4 @AirCraft");
    /// let filtered = list.by_context("air");
    /// assert_eq!(filtered.tasks().len(), 3);
    ///
    /// let chained = list.by_context("air").by_context("craft");
    /// assert_eq!(chained.tasks().len(), 1);
    /// assert_eq!(chained, TaskList::new(vec!["x Task 4 @AirCraft".into()]));
    /// ```
    pub fn by_context<S: Into<String>>(&self, context: S) -> TaskList {
        let context = context.into();
        TaskList::new(self.tasks
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
            .collect()
        )
    }

    /// Filter tasks by project
    /// 
    /// Will filter both open and done tasks
    /// This will filter by checking if the predicate is contained within any project tag
    ///
    /// The filter is case insensitive.
    ///
    /// Can be chained with any other `by_*` method.
    ///
    /// # Example
    /// ```
    /// use anansi::{List, TaskList};
    /// 
    /// let mut list = List::new("list.txt");
    /// list.add("Task 1");
    /// list.add("Task 2 +home");
    /// list.add("Task 3 +HOME");
    /// list.add("x Task 4 +homeImprovements");
    /// let filtered = list.by_project("home");
    /// assert_eq!(filtered.tasks().len(), 3);
    ///
    /// let chained = list.by_project("home").by_project("improvements");
    /// assert_eq!(chained.tasks().len(), 1);
    /// assert_eq!(chained, TaskList::new(vec!["x Task 4 +homeImprovements".into()]));
    /// ```
    pub fn by_project<S: Into<String>>(&self, project: S) -> TaskList {
        let project = project.into();
        TaskList::new(self.tasks
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
            .collect()
        )
    }

    /// Filter tasks by special
    /// 
    /// Will filter both open and done tasks
    /// This will filter by checking if the predicate is contained within any special tag
    ///
    /// The filter is case insensitive.
    ///
    /// Can be chained with any other `by_*` method.
    ///
    /// # Example
    /// ```
    /// use anansi::{List, TaskList};
    ///
    /// let mut list = List::new("path/to/list.json");
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
    /// assert_eq!(list.by_special("due"), TaskList::new(vec!["Task 2 due:tomorrow".into(), "Task 3 DUE:31.12".into(), "x Task 4 assignment_due:2020-01-01".into()]));
    ///
    /// assert_eq!(list.by_special("assignment").tasks().len(), 1);
    ///
    /// let chained = list.by_special("due").by_special("assignment");
    /// assert_eq!(chained.tasks().len(), 1);
    /// assert_eq!(chained, TaskList::new(vec!["x Task 4 assignment_due:2020-01-01".into()]));
    /// ```
    pub fn by_special<S: Into<String>>(&self, special: S) -> TaskList {
        let special = special.into();
        TaskList::new(self.tasks
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
            .collect()
        )
    }
}
