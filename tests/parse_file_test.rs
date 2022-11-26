use std::collections::HashMap;

use csv_parser::core::{TableRow, parse_file};

#[test]
fn should_parse_file() {
    let table = parse_file("tests/parse_file_test.csv");
    let mut row1: TableRow = HashMap::new();
    let mut row2: TableRow = HashMap::new();
    let mut row3: TableRow = HashMap::new();

    row1.insert("name".to_string(), "John Doe".to_string());
    row1.insert("age".to_string(), "26".to_string());
    row2.insert("name".to_string(), "Makoto Niijima".to_string());
    row2.insert("age".to_string(), "18".to_string());
    row3.insert("name".to_string(), "Geralt of Rivia".to_string());
    row3.insert("age".to_string(), "130".to_string());
    let expect = vec![
        row1,
        row2,
        row3
    ];

    assert_eq!(table, expect);
}

#[test]
#[should_panic("Failed to read: test.csv")]
fn should_panic_if_file_does_not_exists() {
    parse_file("test.csv");
}
