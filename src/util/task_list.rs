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
