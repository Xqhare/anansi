use anansi::*;

#[test]
fn basic_todo() {
    let list = List::new("test-data/basic-todo.txt");
    
    assert_eq!(list.open().len(), 5);
    assert_eq!(list.done().len(), 4);

    assert_eq!(list.by_prio("A").tasks().len(), 2);
    assert_eq!(list.by_prio("b").tasks().len(), 1);

    assert_eq!(list.by_project("project1").tasks().len(), 6);
    assert_eq!(list.by_context("context2").tasks().len(), 4);
    assert_eq!(list.by_special("keyword").tasks().len(), 1);

    assert_eq!(list.by_project("project1").by_context("context3").tasks().len(), 3);
    assert_eq!(list.by_project("project1").by_context("context3").by_special("keyword").tasks().len(), 1);

    let complete_task = list.by_project("project1").by_context("context3").by_special("keyword").done()[0].clone();
    assert_eq!(complete_task.is_done(), true);
    assert_eq!(complete_task.prio(), "A");
    assert_eq!(complete_task.completion_date(), "2020-10-21");
    assert_eq!(complete_task.inception_date(), "2020-10-20");
    assert_eq!(complete_task.contexts(), &vec!["context3".to_string()]);
    assert_eq!(complete_task.projects(), &vec!["project1".to_string()]);
    assert_eq!(complete_task.specials().get("keyword").unwrap(), "value");
    assert_eq!(complete_task.text(), "Task description +project1 @context3 keyword:value");
    assert_eq!(complete_task.description(), "Task description");
    assert_eq!(complete_task.original(), "x (A) 2020-10-21 2020-10-20 Task description +project1 @context3 keyword:value");
}

#[test]
fn complex_todo() {
    let list = List::new("test-data/complex-todo.txt");

    assert_eq!(list.open().len(), 50);
    assert_eq!(list.done().len(), 18);
}

#[test]
fn trailing_newline() {
    let list = List::new("test-data/trailing-newline-todo.txt");
    assert_eq!(list.open().len(), 11);
}

#[test]
fn leading_newline() {
    let list = List::new("test-data/leading-newline-todo.txt");
    assert_eq!(list.open().len(), 8);
}

#[test]
fn very_large_todo() {
    let start = std::time::Instant::now();
    let list = List::new("test-data/very-large-todo.txt");
    assert_eq!(list.open().len(), 17724);
    assert_eq!(list.done().len(), 6276);
    let elapsed = start.elapsed().as_micros();

    println!("Parsing todo took {} us / {} ms", elapsed, elapsed / 1000);
    // 200ms is 200_000us
    assert!(elapsed < 215_000);
}
