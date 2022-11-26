pub mod types;

use std::fs::read_to_string;
use std::collections::HashMap;

pub use types::*;

pub fn parse_file(path: &str) -> Table {
    let file = read_to_string(path).expect(&format!("Failed to read: {}", path));
    let mut headers: Vec<String> = Vec::new();
    let mut table: Table = Vec::new();

    for (index, line) in file.lines().enumerate() {
        if index == 0 {
            headers = parse_line(line);
        }
        else {
            let row = parse_row(line, &headers);
            table.push(row);
        }
    }

    return table;
}


pub fn parse_row(line: &str, headers: &Vec<String>) -> TableRow {
    let mut row: TableRow = HashMap::new();
    let mut it_is_in_quotes = false;
    let mut initial_index = 0;
    let mut header_index = 0;

    for (index, char) in line.chars().enumerate() {
        match char {
            '"' => it_is_in_quotes = !it_is_in_quotes,
            ',' => {
                if !it_is_in_quotes {
                    let value = &line[initial_index..index];
                    let value = remove_comma(value);
                    let header = headers.get(header_index).expect("Header does not exist");
                    row.insert(header.clone(), value);
                    initial_index = index + 1;
                    header_index = header_index + 1;
                }
            },
            _ => {},
        }
    }

    let value = &line[initial_index..];
    let value = remove_comma(value);
    let header = headers.get(header_index).expect("Header does not exist");
    row.insert(header.clone(), value);


    return row;
}

pub fn parse_line(line: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut it_is_in_quotes = false;
    let mut initial_index = 0;

    for (index, char) in line.chars().enumerate() {
        match char {
            '"' => it_is_in_quotes = !it_is_in_quotes,
            ',' => {
                if !it_is_in_quotes {
                    let value = &line[initial_index..index];
                    let value = remove_comma(value);
                    tokens.push(value);
                    initial_index = index + 1;
                }
            },
            _ => {},
        }
    }

    let value = &line[initial_index..];
    let value = remove_comma(value);
    tokens.push(value);

    return tokens;
}

fn remove_comma(string: &str) -> String {
    return string.replace("\"", "");
}
