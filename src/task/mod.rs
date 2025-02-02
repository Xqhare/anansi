mod builder;

use std::collections::BTreeMap;

use builder::deserialize_task;

use crate::Date;

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

impl Task {
    pub fn new<S: AsRef<str>>(text: S) -> Task {
        deserialize_task(text)
    }

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

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn done(&mut self) {
        self.done = true;
    }

    pub fn undone(&mut self) {
        self.done = false;
    }

    pub fn prio(&self) -> String {
        if let Some(priority) = self.priority {
            priority.to_string()
        } else {
            String::new()
        }
    }

    pub fn contexts(&self) -> &Vec<String> {
        &self.context_tags
    }

    pub fn projects(&self) -> &Vec<String> {
        &self.project_tags
    }

    pub fn specials(&self) -> &BTreeMap<String, String> {
        &self.special_tags
    }

    pub fn completion_date(&self) -> String {
        self.completion_date.format_date()
    }

    pub fn inception_date(&self) -> String {
        self.inception_date.format_date()
    }

    pub fn text(&self) -> &String {
        &self.text
    }

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
