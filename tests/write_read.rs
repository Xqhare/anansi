use anansi::*;

const INPUT_A: [&str; 5] = [
    "x (A) 2022-11-11 2022-01-01 test +proj @cont key:val",
    "x (B) 2022-11-11 2022-01-01 test +proj @cont key:val",
    "x (C) 2022-11-11 2022-01-01 test +proj @cont key:val",
    "x (D) 2022-11-11 2022-01-01 test +proj @cont key:val",
    "x (E) 2022-11-11 2022-01-01 test +proj @cont key:val",
];

const INPUT_B: [&str; 10] = [
    "(A) 2022-01-01 test +proj @cont key:val @cont0",
    "(B) 2022-01-01 test +proj @cont key:val @cont0",
    "(C) 2022-01-01 test +proj @cont key:val @cont0 due:2022-01-10",
    "(D) 2022-01-01 test +proj @cont key:val @cont0",
    "(E) 2022-01-01 test +proj @cont key:val @cont0",
    "(Z) 2022-01-01 test +proj1 @cont1 key1:val",
    "(Y) 2022-01-01 test +proj2 @cont2 key2:val",
    "(X) 2022-01-01 test +proj3 @cont3 key3:val",
    "(W) 2022-01-01 test +proj4 @cont4 key4:val",
    "(V) 2022-01-01 test +proj5 @cont5 key5:val",
];

#[test]
fn write_read_all() {
    let path = "test-data/wra_test.txt";

    if std::path::Path::new(path).exists() {
        std::fs::remove_file(path).unwrap();
    }

    let mut list = List::new(path);
    for input in INPUT_A {
        list.add(input);
    }
    for input in INPUT_B {
        list.add(input);
    }

    let save = list.save();
    assert!(save.is_ok());

    let read_list = List::new(path);
    assert_eq!(read_list.open().len(), 10);
    assert_eq!(read_list.done().len(), 5);

    assert_eq!(read_list.by_project("proj").tasks().len(), 15);
    assert_eq!(read_list.by_project("proj").by_context("cont0").tasks().len(), 5);
    assert_eq!(read_list.by_project("proj").by_context("cont0").by_special("due").tasks().len(), 1);
    assert_eq!(read_list.by_context("cont").tasks().len(), 15);

    let remove = std::fs::remove_file(path);
    assert!(remove.is_ok());
}
