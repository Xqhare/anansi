use std::collections::{BTreeMap, VecDeque};

use crate::{util::deserialise_date, Date};

use super::Task;

pub fn deserialize_task<S: AsRef<str>>(input: S, id: usize) -> Task {
    let original_text = input.as_ref().to_string();
    let mut tokens: VecDeque<&str> = input.as_ref().split_whitespace().collect();

    let done = if tokens[0] == "x" { true } else { false };
    if done {
        let _ = tokens.pop_front();
    }

    let priority: Option<char> = {
        if tokens[0].starts_with('(') && tokens[0].ends_with(')') && tokens[0].len() == 3 {
            let potential_prio = tokens.front().unwrap().chars().nth(1).unwrap();
            if potential_prio.is_uppercase() && potential_prio.is_alphabetic() {
                Some(tokens.pop_front().unwrap().chars().nth(1).unwrap())
            } else {
                None
            }
        } else if tokens[0].starts_with('(') && tokens[0].ends_with(')') && tokens[0].len() == 2 {
            // Fixes bad formatting from old ananke implementation
            // It added `()` if no priority was set
            let _ = tokens.pop_front();
            None
        } else {
            None
        }
    };

    let mut completion_date = {
        let potential_date = deserialise_date(tokens.front().unwrap());
        if potential_date.is_set() {
            let _ = tokens.pop_front();
            potential_date
        } else {
            Date::default()
        }
    };

    let mut inception_date = {
        let potential_date = deserialise_date(tokens.front().unwrap());
        if potential_date.is_set() {
            let _ = tokens.pop_front();
            potential_date
        } else {
            Date::default()
        }
    };

    if completion_date.is_set() && !inception_date.is_set() {
        inception_date = completion_date;
        completion_date = Date::default();
    }

    let mut text = String::new();
    let mut description = String::new();
    let mut context_tags: Vec<String> = Vec::new();
    let mut project_tags: Vec<String> = Vec::new();
    let mut special_tags: BTreeMap<String, String> = BTreeMap::new();
    // This also removes all newline characters
    for token in tokens {
        if token.starts_with('@') && token.len() > 1 {
            text.push_str(token);
            text.push(' ');
            context_tags.push(token.strip_prefix('@').unwrap().to_string());
        } else if token.starts_with('+') && token.len() > 1 {
            text.push_str(token);
            text.push(' ');
            project_tags.push(token.strip_prefix('+').unwrap().to_string());
        } else if token.len() > 2 && !token.starts_with(':') && !token.ends_with(':') && token.contains(':') && !token.contains("::") {
            text.push_str(token);
            text.push(' ');
            let (key, value) = token.split_once(':').unwrap();
            special_tags.insert(key.to_string(), value.to_string());
        } else {
            text.push_str(token);
            text.push(' ');
            description.push_str(token);
            description.push(' ');
        }
    }

    Task {
        id,
        done,
        priority,
        completion_date,
        inception_date,
        text: text.trim_end().to_string(),
        description: description.trim_end().to_string(),
        context_tags,
        project_tags,
        special_tags,
        original_text,
    }
}

