#![allow(unused_imports)]

use std::collections::BTreeMap;

use crate::{Date, Task};

#[test]
fn mangled_string_old_ananke_prio_encoding() {
    let text0 = "() 2020-12-31 test +project @context key:value";
    let task0 = Task::new(text0, 0);
    assert_eq!(task0.is_done(), false);
    assert_eq!(task0.prio(), None);
    assert_eq!(task0.inception_date(), "2020-12-31");
    assert_eq!(task0.completion_date(), "");

    let text1 = "x () 2022-11-11 2020-12-31 test +project @context key:value";
    let task1 = Task::new(text1, 0);
    assert_eq!(task1.is_done(), true);
    assert_eq!(task1.prio(), None);
    assert_eq!(task1.inception_date(), "2020-12-31");
    assert_eq!(task1.completion_date(), "2022-11-11");
}

/// Regression test
#[test]
fn empty_string() {
    let task = Task::new("", 0);
    assert_eq!(task.is_done(), false);
    assert_eq!(task.prio(), None);
    assert_eq!(task.inception_date(), "");
    assert_eq!(task.completion_date(), "");
}

#[test]
fn basic_deserialisation() {
    let text = "x (A) test +project @context key:value";
    let task = Task::new(text, 0);
    assert_eq!(task.is_done(), true);
    assert_eq!(task.prio(), Some('A'));
    assert_eq!(task.contexts(), &vec!["context".to_string()]);
    assert_eq!(task.projects(), &vec!["project".to_string()]);
    assert_eq!(
        task.specials(),
        &BTreeMap::from([("key".to_string(), "value".to_string())])
    );
    assert_eq!(task.text(), "test +project @context key:value");
    assert_eq!(task.to_string(), text);
    assert!(!task.completion_date.is_set());
    assert!(!task.inception_date.is_set());
}

#[test]
fn advanced_deserialisation() {
    let text1 =
        "(Z) 2020-12-31 lorem ipsum dolor sit amet +project @context key1:value1 key2:value2";
    let task1 = Task::new(text1, 0);
    assert_eq!(task1.is_done(), false);
    assert_eq!(task1.prio(), Some('Z'));
    assert_eq!(task1.completion_date(), "");
    assert_eq!(task1.inception_date(), "2020-12-31");
    assert_eq!(task1.contexts(), &vec!["context".to_string()]);
    assert_eq!(task1.projects(), &vec!["project".to_string()]);
    assert_eq!(
        task1.specials(),
        &BTreeMap::from([
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string())
        ])
    );
    assert_eq!(
        task1.text(),
        "lorem ipsum dolor sit amet +project @context key1:value1 key2:value2"
    );
    assert_eq!(task1.to_string(), text1);

    let text2 = "x 2010-10-20 2010-10-01 lorem ipsum dolor sit amet +project @context key1:value1 key2:value2";
    let task2 = Task::new(text2, 0);
    assert_eq!(task2.is_done(), true);
    assert_eq!(task2.prio(), None);
    assert_eq!(task2.completion_date(), "2010-10-20");
    assert_eq!(task2.inception_date(), "2010-10-01");
    assert_eq!(task2.contexts(), &vec!["context".to_string()]);
    assert_eq!(task2.projects(), &vec!["project".to_string()]);
    assert_eq!(
        task2.specials(),
        &BTreeMap::from([
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string())
        ])
    );
    assert_eq!(
        task2.text(),
        "lorem ipsum dolor sit amet +project @context key1:value1 key2:value2"
    );
    assert_eq!(task2.to_string(), text2);
}

#[test]
fn date_deserialisation() {
    let text1 =
        "(Z) 2020-12-31 lorem ipsum dolor sit amet +project @context key1:value1 key2:value2";
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

    let text4 =
        "(Z) 2020-IT-CM lorem ipsum dolor sit amet +project @context key1:value1 key2:value2";
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

#[test]
fn priority() {
    let task1 = Task::new("(A) test", 0);
    assert_eq!(task1.prio(), Some('A'));
    let task2 = Task::new("(B) test", 0);
    assert_eq!(task2.prio(), Some('B'));
    let task3 = Task::new("(Z) test", 0);
    assert_eq!(task3.prio(), Some('Z'));
}

#[test]
fn context_tags() {
    let task1 = Task::new(
        "(A) test @context @context2 @test @test2 @test3 key1:value1 key2:value2",
        0,
    );
    assert_eq!(
        task1.contexts(),
        &vec![
            "context".to_string(),
            "context2".to_string(),
            "test".to_string(),
            "test2".to_string(),
            "test3".to_string()
        ]
    );
    let task2 = Task::new(
        "(A) test +project @context1 @context2 key1:value1 key2:value2",
        0,
    );
    assert_eq!(task2.contexts().len(), 2);
    let task3 = Task::new(
        "(A) test +project key1:value1 key2:value2 web@email.com @ context1 @context2",
        0,
    );
    assert_eq!(task3.contexts().len(), 1);
}

#[test]
fn project_tags() {
    let task1 = Task::new(
        "(A) test +project +project2 +test +test2 +test3 key1:value1 key2:value2",
        0,
    );
    assert_eq!(
        task1.projects(),
        &vec![
            "project".to_string(),
            "project2".to_string(),
            "test".to_string(),
            "test2".to_string(),
            "test3".to_string()
        ]
    );
    let task2 = Task::new(
        "(A) test +project +project2 +test +test2 +test3 @context1 @context2 key1:value1 key2:value2",
        0,
    );
    assert_eq!(task2.projects().len(), 5);
    let task3 = Task::new(
        "(A) test +project 1 + 2, 1+2 key1:value1 key2:value2 web@email.com @ context1 @context2",
        0,
    );
    assert_eq!(task3.projects().len(), 1);
}

#[test]
fn special_tags() {
    let task1 = Task::new(
        "(A) test +project +project2 +test +test2 +test3 key1:value1 key2:value2",
        0,
    );
    assert_eq!(
        task1.specials(),
        &BTreeMap::from([
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string())
        ])
    );
    let task2 = Task::new(
        "(A) test +project +project2 +test +test2 +test3 @context1 @context2 key1:value1 key2:value2",
        0,
    );
    assert_eq!(task2.specials().len(), 2);
    let task3 = Task::new(
        "(A) test +project test: hello! :2 : testing::test 1 + 2, 1+2 key1:value1 key2:value2 web@email.com @ context1 @context2",
        0,
    );
    assert_eq!(task3.specials().len(), 2);
}

#[test]
fn dates() {
    let task1 = Task::new(
        "(A) test +project +project2 +test +test2 +test3 key1:value1 key2:value2",
        0,
    );
    assert_eq!(task1.completion_date(), "");
    assert_eq!(task1.inception_date(), "");

    let task2 = Task::new(
        "(A) 2010-10-20 2010-10-01 test +project +project2 +test +test2 +test3 key1:value1 key2:value2",
        0,
    );
    assert_eq!(task2.completion_date(), "2010-10-20");
    assert_eq!(task2.inception_date(), "2010-10-01");
}

#[test]
fn priorities() {
    let tmp = vec![
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z",
    ];
    for letter in tmp {
        let task = Task::new(
            format!(
                "({}) test +project +project2 +test +test2 +test3 key1:value1 key2:value2",
                letter
            ),
            0,
        );
        assert_eq!(task.prio(), Some(letter.chars().next().unwrap()));
    }
}
