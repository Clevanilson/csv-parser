use std::collections::HashMap;

use csv_parser::core::{parse_row, TableRow};

#[test]
fn should_parse_row() {
    let headers = vec!["name".to_string(), "age".to_string()];
    let line = "John Doe,26";
    let row: TableRow = parse_row(line, &headers);
    let mut expect: TableRow = HashMap::new();

    expect.insert("name".to_string(), "John Doe".to_string());
    expect.insert("age".to_string(), "26".to_string());

    assert_eq!(row, expect);
}

#[test]
#[should_panic("Header does not exist")]
fn should_panic_if_headers_does_not_match() {
    let headers = vec!["Name".to_string()];
    let line = "John Doe,26";
    parse_row(line, &headers);
}
