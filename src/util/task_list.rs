use crate::Task;

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
    pub fn new(tasks: Vec<Task>) -> TaskList {
        TaskList { tasks }
    }

    pub fn tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn by_prio<S: Into<String>>(&self, prio: S) -> TaskList {
        let prio = prio.into();
        TaskList::new(self.tasks
            .iter()
            .filter(|task| task.prio() == prio)
            .cloned()
            .collect()
        )
    }
}
