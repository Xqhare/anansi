#![allow(unused_imports)]

use std::collections::BTreeMap;

use crate::{Date, Task};

#[test]
fn basic_deserialisation() {
    let text = "x (A) test +project @context key:value";
    let task = Task::new(text, 0);
    assert_eq!(task.is_done(), true);
    assert_eq!(task.prio(), String::from("A"));
    assert_eq!(task.contexts(), &vec!["context".to_string()]);
    assert_eq!(task.projects(), &vec!["project".to_string()]);
    assert_eq!(task.specials(), &BTreeMap::from([("key".to_string(), "value".to_string())]));
    assert_eq!(task.text(), "test +project @context key:value");
    assert_eq!(task.original(), text);
    assert!(!task.completion_date.is_set());
    assert!(!task.inception_date.is_set());
}

#[test]
fn advanced_deserialisation() {
    let text1 = "(Z) 2020-12-31 lorem ipsum dolor sit amet +project @context key1:value1 key2:value2";
    let task1 = Task::new(text1, 0);
    assert_eq!(task1.is_done(), false);
    assert_eq!(task1.prio(), String::from("Z"));
    assert_eq!(task1.completion_date(), "");
    assert_eq!(task1.inception_date(), "2020-12-31");
    assert_eq!(task1.contexts(), &vec!["context".to_string()]);
    assert_eq!(task1.projects(), &vec!["project".to_string()]);
    assert_eq!(task1.specials(), &BTreeMap::from([("key1".to_string(), "value1".to_string()), ("key2".to_string(), "value2".to_string())]));
    assert_eq!(task1.text(), "lorem ipsum dolor sit amet +project @context key1:value1 key2:value2");
    assert_eq!(task1.original(), text1);

    let text2 = "2010-10-20 2010-10-01 lorem ipsum dolor sit amet +project @context key1:value1 key2:value2";
    let task2 = Task::new(text2, 0);
    assert_eq!(task2.is_done(), false);
    assert_eq!(task2.prio(), "");
    assert_eq!(task2.completion_date(), "2010-10-20");
    assert_eq!(task2.inception_date(), "2010-10-01");
    assert_eq!(task2.contexts(), &vec!["context".to_string()]);
    assert_eq!(task2.projects(), &vec!["project".to_string()]);
    assert_eq!(task2.specials(), &BTreeMap::from([("key1".to_string(), "value1".to_string()), ("key2".to_string(), "value2".to_string())]));
    assert_eq!(task2.text(), "lorem ipsum dolor sit amet +project @context key1:value1 key2:value2");
    assert_eq!(task2.original(), text2);
}

#[test]
fn date_deserialisation() {
    let text1 = "(Z) 2020-12-31 lorem ipsum dolor sit amet +project @context key1:value1 key2:value2";
    let task1 = Task::new(text1, 0);
    assert_eq!(task1.completion_date(), "");
    assert_eq!(task1.inception_date(), "2020-12-31");

    let text2 = "2010-10-20 2010-10-01 lorem ipsum dolor sit amet +project @context key1:value1 key2:value2";
    let task2 = Task::new(text2, 0);
    assert_eq!(task2.completion_date(), "2010-10-20");
    assert_eq!(task2.inception_date(), "2010-10-01");

    let text3 = "lorem ipsum dolor sit amet +project @context key1:value1 key2:value2";
    let task3 = Task::new(text3, 0);
    assert_eq!(task3.completion_date(), "");
    assert_eq!(task3.inception_date(), "");

    let text4 = "(Z) 2020-IT-CM lorem ipsum dolor sit amet +project @context key1:value1 key2:value2";
    let task4 = Task::new(text4, 0);
    assert_eq!(task4.completion_date(), "");
    assert_eq!(task4.inception_date(), "");

    let text5 = "(Z) Meba-pe-la Dorla-tu lorem ipsum dolor sit amet +project @context key1:value1 key2:value2";
    let task5 = Task::new(text5, 0);
    assert_eq!(task5.completion_date(), "");
    assert_eq!(task5.inception_date(), "");
}

#[test]
fn equality() {
    let task1 = Task::new("(A) test", 0);
    let task2 = Task::new("(A) test", 0);
    assert!(task1 == task2);
    let task3 = Task::new("(B) test", 0);
    assert!(task1 != task3);
}

#[test]
fn ordering() {
    let task1 = Task::new("(A) test", 0);
    let task2 = Task::new("(B) test", 0);
    let task3 = Task::new("(Z) test", 0);
    let task4 = Task::new("(A) test", 0);
    assert!(task2 > task3);
    assert!(task1 >= task4);
    assert!(task3 < task2);
    assert!(task1 <= task4);
}

/* #[test]
fn update() {
    let mut task1 = Task::new("(A) test", 0);
    task1.update("x (B) test");
    assert_eq!(task1.is_done(), true);
    assert_eq!(task1.prio(), String::from("B"));
    assert_eq!(task1.text(), "test");
    assert_eq!(task1.original(), "x (B) test");
}

#[test]
fn mark_done() {
    let mut task1 = Task::new("(A) test", 0);
    task1.done(None);
    assert_eq!(task1.is_done(), true);
    assert_eq!(task1.original(), "x (A) test");
    task1.undone();
    assert_eq!(task1.is_done(), false);
    assert_eq!(task1.original(), "(A) test");
} */

#[test]
fn priority() {
    let task1 = Task::new("(A) test", 0);
    assert_eq!(task1.prio(), String::from("A"));
    let task2 = Task::new("(B) test", 0);
    assert_eq!(task2.prio(), String::from("B"));
    let task3 = Task::new("(Z) test", 0);
    assert_eq!(task3.prio(), String::from("Z"));
}

#[test]
fn context_tags() {
    let task1 = Task::new("(A) test @context @context2 @test @test2 @test3 key1:value1 key2:value2", 0); 
    assert_eq!(task1.contexts(), &vec!["context".to_string(), "context2".to_string(), "test".to_string(), "test2".to_string(), "test3".to_string()]);
    let task2 = Task::new("(A) test +project @context1 @context2 key1:value1 key2:value2", 0);
    assert_eq!(task2.contexts().len(), 2);
    let task3 = Task::new("(A) test +project key1:value1 key2:value2 web@email.com @ context1 @context2", 0);
    assert_eq!(task3.contexts().len(), 1);
}

#[test]
fn project_tags() {
    let task1 = Task::new("(A) test +project +project2 +test +test2 +test3 key1:value1 key2:value2", 0); 
    assert_eq!(task1.projects(), &vec!["project".to_string(), "project2".to_string(), "test".to_string(), "test2".to_string(), "test3".to_string()]);
    let task2 = Task::new("(A) test +project +project2 +test +test2 +test3 @context1 @context2 key1:value1 key2:value2", 0);
    assert_eq!(task2.projects().len(), 5);
    let task3 = Task::new("(A) test +project 1 + 2, 1+2 key1:value1 key2:value2 web@email.com @ context1 @context2", 0);
    assert_eq!(task3.projects().len(), 1);
}

#[test]
fn special_tags() {
    let task1 = Task::new("(A) test +project +project2 +test +test2 +test3 key1:value1 key2:value2", 0); 
    assert_eq!(task1.specials(), &BTreeMap::from([("key1".to_string(), "value1".to_string()), ("key2".to_string(), "value2".to_string())]));
    let task2 = Task::new("(A) test +project +project2 +test +test2 +test3 @context1 @context2 key1:value1 key2:value2", 0);
    assert_eq!(task2.specials().len(), 2);
    let task3 = Task::new("(A) test +project test: hello! :2 : testing::test 1 + 2, 1+2 key1:value1 key2:value2 web@email.com @ context1 @context2", 0);
    assert_eq!(task3.specials().len(), 2);
}

#[test]
fn dates() {
    let task1 = Task::new("(A) test +project +project2 +test +test2 +test3 key1:value1 key2:value2", 0); 
    assert_eq!(task1.completion_date(), "");
    assert_eq!(task1.inception_date(), "");

    let task2 = Task::new("(A) 2010-10-20 2010-10-01 test +project +project2 +test +test2 +test3 key1:value1 key2:value2", 0);
    assert_eq!(task2.completion_date(), "2010-10-20");
    assert_eq!(task2.inception_date(), "2010-10-01");

    /* let mut task3 = Task::new("(A) 2020-12-31 test +project +project2 +test +test2 +test3 key1:value1 key2:value2", 0);
    assert_eq!(task3.completion_date(), "");
    assert_eq!(task3.inception_date(), "2020-12-31");
    task3.done(Some(Date::new(2021, 12, 30)));
    assert_eq!(task3.completion_date(), "2021-12-30");
    assert_eq!(task3.inception_date(), "2020-12-31");
    assert_eq!(task3.original(), "x (A) 2021-12-30 2020-12-31 test +project +project2 +test +test2 +test3 key1:value1 key2:value2");
    task3.undone();
    assert_eq!(task3.completion_date(), "");
    assert_eq!(task3.inception_date(), "2020-12-31");
    assert_eq!(task3.original(), "(A) 2020-12-31 test +project +project2 +test +test2 +test3 key1:value1 key2:value2"); */
}

#[test]
fn priorities() {
    let tmp = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];
    for letter in tmp {
        let task = Task::new(format!("({}) test +project +project2 +test +test2 +test3 key1:value1 key2:value2", letter), 0);
        assert_eq!(task.prio(), letter);
    }
}
