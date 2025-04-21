#![allow(unused_imports)]

use crate::SortBy;

use super::List;

#[test]
fn simple_write() {
    let text0 = "2020-12-31 test +project @context key:value";
    let mut task0 = List::new("no_prio.txt");
    task0.add(text0);
    let write = task0.save();
    assert!(write.is_ok());
    std::fs::remove_file("no_prio.txt").unwrap();
}

#[test]
fn sort_by_prio_unique() {
    let mut list = List::new("prio_sorting.txt");
    list.add("(Z) Task 1");
    list.add("(X) Task 2");
    list.add("(Q) Task 3");
    list.add("(L) Task 4");
    list.add("(J) Task 5");
    list.add("(H) Task 6");
    list.add("(C) Task 7");
    list.add("(A) Task 8");

    let sorted_tasks = list.sort(SortBy::Priority);
    println!("{:?}", sorted_tasks);
    assert_eq!(sorted_tasks[0].original(), "(A) Task 8");
    assert_eq!(sorted_tasks[1].original(), "(C) Task 7");
    assert_eq!(sorted_tasks[2].original(), "(H) Task 6");
    assert_eq!(sorted_tasks[3].original(), "(J) Task 5");
    assert_eq!(sorted_tasks[4].original(), "(L) Task 4");
    assert_eq!(sorted_tasks[5].original(), "(Q) Task 3");
    assert_eq!(sorted_tasks[6].original(), "(X) Task 2");
    assert_eq!(sorted_tasks[7].original(), "(Z) Task 1");
}

#[test]
fn sort_by_priority_non_unique() {
    let mut list = List::new("prio_sorting.txt");
    // 8 tasks for Z, 1 task for X, 1 task for Q, 5 tasks for L, 5 tasks for J, 2 tasks for H
    list.add("(Z) Task 1");
    list.add("(Z) Task 2");
    list.add("(Z) Task 3");
    list.add("(Z) Task 4");
    list.add("(Z) Task 5");
    list.add("(Z) Task 6");
    list.add("(Z) Task 7");
    list.add("(Z) Task 8");
    list.add("(X) Task 9");
    list.add("(Q) Task 10");
    list.add("(L) Task 11");
    list.add("(L) Task 12");
    list.add("(L) Task 13");
    list.add("(L) Task 14");
    list.add("(L) Task 15");
    list.add("(L) Task 16");
    list.add("(L) Task 17");
    list.add("(L) Task 18");
    list.add("(L) Task 19");
    list.add("(L) Task 20");
    list.add("(J) Task 21");
    list.add("(J) Task 22");
    list.add("(J) Task 23");
    list.add("(J) Task 24");
    list.add("(J) Task 25");
    list.add("(H) Task 26");
    list.add("(H) Task 27");

    let sorted_tasks = list.sort(SortBy::Priority);
    assert_eq!(sorted_tasks[0].original(), "(H) Task 26");
    assert_eq!(sorted_tasks[1].original(), "(H) Task 27");
    assert_eq!(sorted_tasks[2].original(), "(J) Task 21");
    assert_eq!(sorted_tasks[3].original(), "(J) Task 22");
    assert_eq!(sorted_tasks[4].original(), "(J) Task 23");
    assert_eq!(sorted_tasks[5].original(), "(J) Task 24");
    assert_eq!(sorted_tasks[6].original(), "(J) Task 25");
    assert_eq!(sorted_tasks[7].original(), "(L) Task 11");
    assert_eq!(sorted_tasks[8].original(), "(L) Task 12");
    assert_eq!(sorted_tasks[9].original(), "(L) Task 13");
    assert_eq!(sorted_tasks[10].original(), "(L) Task 14");
    assert_eq!(sorted_tasks[11].original(), "(L) Task 15");
    assert_eq!(sorted_tasks[12].original(), "(L) Task 16");
    assert_eq!(sorted_tasks[13].original(), "(L) Task 17");
    assert_eq!(sorted_tasks[14].original(), "(L) Task 18");
    assert_eq!(sorted_tasks[15].original(), "(L) Task 19");
    assert_eq!(sorted_tasks[16].original(), "(L) Task 20");
    assert_eq!(sorted_tasks[17].original(), "(Q) Task 10");
    assert_eq!(sorted_tasks[18].original(), "(X) Task 9");
    assert_eq!(sorted_tasks[19].original(), "(Z) Task 1");
    assert_eq!(sorted_tasks[20].original(), "(Z) Task 2");
    assert_eq!(sorted_tasks[21].original(), "(Z) Task 3");
    assert_eq!(sorted_tasks[22].original(), "(Z) Task 4");
    assert_eq!(sorted_tasks[23].original(), "(Z) Task 5");
    assert_eq!(sorted_tasks[24].original(), "(Z) Task 6");
    assert_eq!(sorted_tasks[25].original(), "(Z) Task 7");
    assert_eq!(sorted_tasks[26].original(), "(Z) Task 8");
}

#[test]
fn sort_by_inception_date_unique() {
    let mut list = List::new("date_sorting.txt");
    list.add("2022-01-08 (A) Task 1");
    list.add("2022-01-07 (A) Task 2");
    list.add("2022-01-06 (A) Task 3");
    list.add("2022-01-05 (A) Task 4");
    list.add("2022-01-04 (A) Task 5");
    list.add("2022-01-03 (A) Task 6");
    list.add("2022-01-02 (A) Task 7");
    list.add("2022-01-01 (A) Task 8");

    let sorted_tasks = list.sort(SortBy::InceptionDate);
    assert_eq!(sorted_tasks[0].original(), "2022-01-01 (A) Task 8");
    assert_eq!(sorted_tasks[1].original(), "2022-01-02 (A) Task 7");
    assert_eq!(sorted_tasks[2].original(), "2022-01-03 (A) Task 6");
    assert_eq!(sorted_tasks[3].original(), "2022-01-04 (A) Task 5");
    assert_eq!(sorted_tasks[4].original(), "2022-01-05 (A) Task 4");
    assert_eq!(sorted_tasks[5].original(), "2022-01-06 (A) Task 3");
    assert_eq!(sorted_tasks[6].original(), "2022-01-07 (A) Task 2");
    assert_eq!(sorted_tasks[7].original(), "2022-01-08 (A) Task 1");
}

#[test]
fn sort_by_inception_date_non_unique() {
    let mut list = List::new("date_sorting.txt");
    list.add("2022-01-02 (A) Task 9");
    list.add("2022-01-02 (B) Task 10");
    list.add("2022-01-02 (C) Task 11");
    list.add("2022-01-02 (D) Task 12");
    list.add("2022-01-02 (E) Task 13");
    list.add("2022-01-02 (F) Task 14");
    list.add("2022-01-02 (G) Task 15");
    list.add("2022-01-02 (H) Task 16");

    list.add("2022-01-01 (A) Task 1");
    list.add("2022-01-01 (B) Task 2");
    list.add("2022-01-01 (C) Task 3");
    list.add("2022-01-01 (D) Task 4");
    list.add("2022-01-01 (E) Task 5");
    list.add("2022-01-01 (F) Task 6");
    list.add("2022-01-01 (G) Task 7");
    list.add("2022-01-01 (H) Task 8");

    let sorted_tasks = list.sort(SortBy::InceptionDate);
    assert_eq!(sorted_tasks[0].original(), "2022-01-01 (A) Task 1");
    assert_eq!(sorted_tasks[1].original(), "2022-01-01 (B) Task 2");
    assert_eq!(sorted_tasks[2].original(), "2022-01-01 (C) Task 3");
    assert_eq!(sorted_tasks[3].original(), "2022-01-01 (D) Task 4");
    assert_eq!(sorted_tasks[4].original(), "2022-01-01 (E) Task 5");
    assert_eq!(sorted_tasks[5].original(), "2022-01-01 (F) Task 6");
    assert_eq!(sorted_tasks[6].original(), "2022-01-01 (G) Task 7");
    assert_eq!(sorted_tasks[7].original(), "2022-01-01 (H) Task 8");
    assert_eq!(sorted_tasks[8].original(), "2022-01-02 (A) Task 9");
    assert_eq!(sorted_tasks[9].original(), "2022-01-02 (B) Task 10");
    assert_eq!(sorted_tasks[10].original(), "2022-01-02 (C) Task 11");
    assert_eq!(sorted_tasks[11].original(), "2022-01-02 (D) Task 12");
    assert_eq!(sorted_tasks[12].original(), "2022-01-02 (E) Task 13");
    assert_eq!(sorted_tasks[13].original(), "2022-01-02 (F) Task 14");
    assert_eq!(sorted_tasks[14].original(), "2022-01-02 (G) Task 15");
    assert_eq!(sorted_tasks[15].original(), "2022-01-02 (H) Task 16");
}

#[test]
fn sort_by_completion_date_unique() {
    let mut list = List::new("date_sorting.txt");
    list.add("x 2022-02-18 2022-01-08 (A) Task 1");
    list.add("x 2022-02-17 2022-01-07 (A) Task 2");
    list.add("x 2022-02-16 2022-01-06 (A) Task 3");
    list.add("x 2022-02-15 2022-01-05 (A) Task 4");
    list.add("x 2022-02-14 2022-01-04 (A) Task 5");
    list.add("x 2022-02-13 2022-01-03 (A) Task 6");
    list.add("x 2022-02-12 2022-01-02 (A) Task 7");
    list.add("x 2022-02-11 2022-01-01 (A) Task 8");

    let sorted_tasks = list.sort(SortBy::CompletionDate);
    assert_eq!(sorted_tasks[0].original(), "x 2022-02-11 2022-01-01 (A) Task 8");
    assert_eq!(sorted_tasks[1].original(), "x 2022-02-12 2022-01-02 (A) Task 7");
    assert_eq!(sorted_tasks[2].original(), "x 2022-02-13 2022-01-03 (A) Task 6");
    assert_eq!(sorted_tasks[3].original(), "x 2022-02-14 2022-01-04 (A) Task 5");
    assert_eq!(sorted_tasks[4].original(), "x 2022-02-15 2022-01-05 (A) Task 4");
    assert_eq!(sorted_tasks[5].original(), "x 2022-02-16 2022-01-06 (A) Task 3");
    assert_eq!(sorted_tasks[6].original(), "x 2022-02-17 2022-01-07 (A) Task 2");
    assert_eq!(sorted_tasks[7].original(), "x 2022-02-18 2022-01-08 (A) Task 1");
}

#[test]
fn sort_by_completion_date_non_unique() {
    let mut list = List::new("date_sorting.txt");
    list.add("x 2022-02-18 2022-01-08 (A) Task 1");
    list.add("x 2022-02-18 2022-01-08 (A) Task 2");
    list.add("x 2022-02-18 2022-01-08 (A) Task 3");
    list.add("x 2022-02-18 2022-01-08 (A) Task 4");
    list.add("x 2022-02-18 2022-01-08 (A) Task 5");
    list.add("x 2022-02-18 2022-01-08 (A) Task 6");
    list.add("x 2022-02-18 2022-01-08 (A) Task 7");
    list.add("x 2022-02-18 2022-01-08 (A) Task 8");

    list.add("x 2022-02-08 2022-01-08 (A) Task 9");
    list.add("x 2022-02-08 2022-01-08 (A) Task 10");
    list.add("x 2022-02-08 2022-01-08 (A) Task 11");
    list.add("x 2022-02-08 2022-01-08 (A) Task 12");
    list.add("x 2022-02-08 2022-01-08 (A) Task 13");
    list.add("x 2022-02-08 2022-01-08 (A) Task 14");
    list.add("x 2022-02-08 2022-01-08 (A) Task 15");
    list.add("x 2022-02-08 2022-01-08 (A) Task 16");

    let sorted_tasks = list.sort(SortBy::CompletionDate);
    assert_eq!(sorted_tasks[0].original(), "x 2022-02-08 2022-01-08 (A) Task 9");
    assert_eq!(sorted_tasks[1].original(), "x 2022-02-08 2022-01-08 (A) Task 10");
    assert_eq!(sorted_tasks[2].original(), "x 2022-02-08 2022-01-08 (A) Task 11");
    assert_eq!(sorted_tasks[3].original(), "x 2022-02-08 2022-01-08 (A) Task 12");
    assert_eq!(sorted_tasks[4].original(), "x 2022-02-08 2022-01-08 (A) Task 13");
    assert_eq!(sorted_tasks[5].original(), "x 2022-02-08 2022-01-08 (A) Task 14");
    assert_eq!(sorted_tasks[6].original(), "x 2022-02-08 2022-01-08 (A) Task 15");
    assert_eq!(sorted_tasks[7].original(), "x 2022-02-08 2022-01-08 (A) Task 16");
    assert_eq!(sorted_tasks[8].original(), "x 2022-02-18 2022-01-08 (A) Task 1");
    assert_eq!(sorted_tasks[9].original(), "x 2022-02-18 2022-01-08 (A) Task 2");
    assert_eq!(sorted_tasks[10].original(), "x 2022-02-18 2022-01-08 (A) Task 3");
    assert_eq!(sorted_tasks[11].original(), "x 2022-02-18 2022-01-08 (A) Task 4");
    assert_eq!(sorted_tasks[12].original(), "x 2022-02-18 2022-01-08 (A) Task 5");
    assert_eq!(sorted_tasks[13].original(), "x 2022-02-18 2022-01-08 (A) Task 6");
    assert_eq!(sorted_tasks[14].original(), "x 2022-02-18 2022-01-08 (A) Task 7");
    assert_eq!(sorted_tasks[15].original(), "x 2022-02-18 2022-01-08 (A) Task 8");
}
