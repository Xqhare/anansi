use std::collections::{BTreeMap, VecDeque};

use crate::{deserialise_date, Date};

use super::Task;

pub fn deserialize_task<S: AsRef<str>>(input: S) -> Task {
    let mut tokens: VecDeque<&str> = input.as_ref().split_whitespace().collect();
    let original_text = tokens.clone().into_iter().collect::<Vec<&str>>().join(" ");

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
        } else if !token.ends_with(':') && token.contains(':') && !token.contains("::") && token.len() > 2 {
            text.push_str(token);
            text.push(' ');
            let (key, value) = token.split_once(':').unwrap();
            special_tags.insert(key.to_string(), value.to_string());
        } else {
            text.push_str(token);
            text.push(' ');
        }
    }
    text = text.trim_end().to_string();

    Task {
        done,
        priority,
        completion_date,
        inception_date,
        text,
        context_tags,
        project_tags,
        special_tags,
        original_text,
    }
}

