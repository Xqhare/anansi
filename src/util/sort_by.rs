use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum SortBy {
    Priority,
    InceptionDate,
    CompletionDate,
}

impl Into<String> for SortBy {
    fn into(self) -> String {
        match self {
            SortBy::Priority => "Priority".to_string(),
            SortBy::InceptionDate => "Inception date".to_string(),
            SortBy::CompletionDate => "Completion date".to_string(),
        }
    }
}

impl From<String> for SortBy {
    fn from(input: String) -> Self {
        match input.as_str() {
            "Priority" => SortBy::Priority,
            "Inception date" => SortBy::InceptionDate,
            "Completion date" => SortBy::CompletionDate,
            _ => panic!("Invalid SortBy"),
        }
    }
}

impl Display for SortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<String>::into(*self))
    }
}
