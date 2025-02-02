# Anansi
Anansi is a ![todo.txt](https://github.com/todotxt/todo.txt) interface written in rust.

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
let mut list = List::new("path/to/todo.txt");

list.add("x (A) 2012.12.12 2010.10.10 Buy milk @store +cooking recipe:cake");
list.add(Task::new("(B) 2010.10.10 Buy eggs @store +cooking"));

for task in list.completed() {
    println!("{}", task);
    task.prio(); // "A"
    task.contexts(); // ["store"]
    task.projects(); // ["cooking"]
    task.specials(); // [["recipe", "cake"],]
    task.completion_date(); // 2012-12-12
    task.inception_date(); // 2010-10-10
    task.text(); // "Buy milk"
    task.original(); // "x (A) 2012.12.12 2010.10.10 Buy milk @store +cooking recipe:cake"
    task.is_done(); // true

    task.undone(); // mark task as undone
    task.update("x (A) Buy milk @store +cooking"); // update task
}

for task in list.uncompleted() {
    println!("{}", task);
    task.prio(); // "B"
    task.contexts(); // ["store"]
    task.projects(); // ["cooking"]
    task.specials(); // []
    task.completion_date(); // None OR ""
    task.inception_date(); // 2010-10-10
    task.text(); // "Buy eggs"
    task.original(); // "(B) 2010.10.10 Buy eggs @store +cooking"
    task.is_done(); // false

    task.done(); // mark task as done
    task.update("(B) 2010.10.10 Buy eggs \n @store +cooking"); // update task
    // Newlines are automatically stripped from the input.
    task.original(); // "(B) 2010.10.10 Buy eggs @store +cooking"
}

let prio_a = list.by_prio("A"); // ["Buy milk"] -> TaskList
let context = list.by_context("store"); // ["Buy milk", "Buy eggs"] -> TaskList
let project = list.by_project("cooking"); // ["Buy milk", "Buy eggs"] -> TaskList

let combo = list.by_prio("A").by_context("store").by_project("cooking"); // ["Buy milk"] -> TaskList
```

## Comparing tasks
Tasks are compared by their priority in descending order.

```rust
use anansi::Task;

let task1 = Task::new("(A) Buy milk");
let task2 = Task::new("(B) Buy eggs");

assert!(task1 > task2);
assert!(task2 < task1);
```
