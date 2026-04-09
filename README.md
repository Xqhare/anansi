# Anansi
Anansi is a ![todo.txt](https://github.com/todotxt/todo.txt) interface written in rust.

It is the backend for [Ananke](https://github.com/Xqhare/ananke).

## Features
- Platform agnostic
- Easy to use
- Performant: A ~1.2MB file containing 24k tasks is read in ~45ms (in release mode on my machine)

## Usage
Add Anansi as a dependency to your project.

```toml
[dependencies]
anansi = { git = "https://github.com/Xqhare/anansi" }
```

Then use it in your code.

```rust
use anansi::{List, Task};

// If the path does not exist, it will be created
let mut list = List::new("path/to/todo.txt"); // TaskList

let task_id_0 = list.add("x (A) 2012-12-12 2010-10-10 Buy milk @store +cooking recipe:cake");
let task_id_1 = list.add("(B) 2010-10-10 Buy eggs @store +cooking");

for mut task in list.done() {
    println!("{}", task);

    // Accessors

    task.id(); // 0
    assert_eq!(task.id(), 1);
    task.prio(); // "A"
    assert_eq!(task.prio(), Some('A'));
    task.contexts(); // ["store"]
    assert_eq!(*task.contexts(), vec!["store".to_string()]);
    task.projects(); // ["cooking"]
    assert_eq!(*task.projects(), vec!["cooking".to_string()]);
    task.specials(); // [["recipe", "cake"],]
    task.completion_date(); // 2012-12-12 OR "" if done_with_date(date) was not called
    assert_eq!(task.completion_date(), "2012-12-12");
    task.inception_date(); // 2010-10-10
    assert_eq!(task.inception_date(), "2010-10-10");
    task.text(); // "Buy milk @store +cooking recipe:cake"
    assert_eq!(task.text(), "Buy milk @store +cooking recipe:cake");
    task.is_done(); // true
    assert!(task.is_done());

    // Mutators

    task.update_prio('B'); // update priority
    assert_eq!(task.prio(), Some('B'));
    task.update_context_tags(vec!["store".to_string(), "grocery".to_string()]); // update contexts
    assert_eq!(*task.contexts(), vec!["store".to_string(), "grocery".to_string()]);
    task.update_project_tags(vec!["cooking".to_string(), "kitchen".to_string()]); // update projects
    assert_eq!(*task.projects(), vec!["cooking".to_string(), "kitchen".to_string()]);
    task.update_special_tags([("recipe".to_string(), "pancake".to_string())].into()); // update specials
    task.update_completion_date("2012-12-12"); // update completion date
    assert_eq!(task.completion_date(), "2012-12-12");
    task.update_inception_date("2010-10-10"); // update inception date
    assert_eq!(task.inception_date(), "2010-10-10");
    task.update_text("Buy eggs @store +cooking"); // update text
    assert_eq!(task.text(), "Buy eggs @store +cooking");
    task.done(None); // mark task as done
    task.done(Some("2012-12-12".into())); // mark task as done with date - Only works if inception date is set
    assert!(task.is_done());
    task.undone(); // mark task as undone
}

for task in list.open() {
    // Same API as above
}

let task = list.get(task_id_0).unwrap(); // Task

let prio = list.by_prio("A"); // TaskList
let context = list.by_context("store"); // TaskList
let project = list.by_project("cooking"); // TaskList
let special = list.by_special("recipe"); // TaskList

let combo = list.by_prio("A").by_context("store").by_project("cooking").set_path("new/path/todo.txt"); // TaskList

combo.save();

list.save();

let list1 = List::load("path/to/todo.txt"); // List
let list2 = List::load("path/to/other/todo.txt"); // List
assert_ne!(list1, list2);

std::fs::remove_file("path/to/other/todo.txt");
std::fs::remove_file("path/to/todo.txt");
```

## Comparing tasks
Tasks are compared by their priority in descending order.

```rust
use anansi::Task;

let task1 = Task::new("(A) Buy milk", 0);
let task2 = Task::new("(B) Buy eggs", 1);

assert!(task1 > task2);
assert!(task2 < task1);
```

## Task equality
Tasks are equal if they have the same text.

```rust
use anansi::Task;

let task1 = Task::new("(A) Buy milk", 0);
let task2 = Task::new("(A) Buy milk", 1);
assert!(task1 == task2);

let task3 = Task::new("x (A) Buy milk", 2);
assert!(task1 != task3);
```
