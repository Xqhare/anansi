use anansi::*;

#[test]
fn simple_use() {
    let mut list = List::new("list.txt");
    list.add("Task 1");
    list.add("x Task 2");
    list.add("Task 3");
    assert_eq!(list.open().len(), 2);
    assert_eq!(list.done().len(), 1);
}

#[test]
fn simple_extensive_use() {
    let mut list = List::new("list.txt");
    // 25 tasks
    list.add("(A) Task 1");
    list.add("(A) Task 2 @air");
    list.add("(B) Task 3 @AIR");
    list.add("(Z) Task 4 @AirCraft");
    list.add("(A) Task 5");
    list.add("(B) Task 6");
    list.add("(Z) Task 7");
    list.add("(A) Task 8");
    list.add("(B) Task 9");
    list.add("(Z) Task 10");
    list.add("(A) Task 11");
    list.add("(B) Task 12");
    list.add("(Z) Task 13");
    list.add("(A) Task 14");
    list.add("(B) Task 15");
    list.add("(Z) Task 16");
    list.add("(A) Task 17");
    list.add("(B) Task 18");
    list.add("(Z) Task 19");
    list.add("(A) Task 20");
    list.add("(B) Task 21");
    list.add("(Z) Task 22");
    list.add("(A) Task 23");
    list.add("(B) Task 24");
    list.add("(Z) Task 25");

    assert_eq!(list.by_prio("a").tasks().len(), 9);
    assert_eq!(list.by_prio("b").tasks().len(), 8);
    assert_eq!(list.by_prio("z").tasks().len(), 8);
    assert_eq!(list.done().len(), 0);
    assert_eq!(list.open().len(), 25);
    assert_eq!(list.by_context("aircraft").tasks().len(), 1);
    assert_eq!(list.by_context("air").tasks().len(), 3);
    assert_eq!(list.by_special("due").tasks().len(), 0);
    assert_eq!(list.by_project("proj").tasks().len(), 0);
    let new_task1 = Task::new("Task 1", 0);
    assert!(list.update(new_task1.clone(), 0).is_ok());
    assert_eq!(new_task1, list.get(0).unwrap());
}
