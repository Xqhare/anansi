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
