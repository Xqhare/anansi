use anansi::{vec::sort_vec_task, *};

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
    let id = list.max_id();
    let new_task1 = Task::new("Task 1", id);
    list.push_task(new_task1.clone());
    assert_eq!(&new_task1, list.get(id).unwrap());
}

#[test]
fn sort_vec() {
    let mut list = List::new("list.txt");
    list.add("Task 1");
    list.add("x Task 2");
    list.add("Task 3");
    list.add("(A) Task 8");
    list.add("(B) Task 9");
    list.add("(Z) Task 10");
    list.add("(A) Task 11");
    list.add("(B) Task 12");

    let mut to_sort = list.tasks();
    let sort_by = SortBy::Priority;

    sort_vec_task(&mut to_sort, sort_by);

    assert_eq!(to_sort[0].to_string(), "(A) Task 8");
    assert_eq!(to_sort[1].to_string(), "(A) Task 11");
    assert_eq!(to_sort[2].to_string(), "(B) Task 9");
    assert_eq!(to_sort[3].to_string(), "(B) Task 12");
    assert_eq!(to_sort[4].to_string(), "(Z) Task 10");
    assert_eq!(to_sort[5].to_string(), "Task 1");
    assert_eq!(to_sort[6].to_string(), "x Task 2");
    assert_eq!(to_sort[7].to_string(), "Task 3");
}
