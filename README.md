# CSV Parser

### Learning Rust by building a simple CSV parser

The CSV parser can read from an `file.csv` and return a `HashMap Vector`.

_This project is for learning purpose only and by any means should be used in production._

## Features

- convert string
- convert csv file
- convert string into hashmap

## Usage

Parsing string lines

```rust
use csv_parser::core::parse_line;

fn main() {
    parse_line("Hello, world!");
    // ["hello", "world!"]
}

```

Parsing string into hashmaps vectors

```rust
use csv_parser::core::parse_row;

fn main() {
    let headers = vec!["name".to_string(), "age".to_string()];
    let line = "John Doe,26";
    parse_row(line, &headers);
    // [{"name": "John Doe", "age": "26" }]

    parse_row(line, &headers).get("name").unwrap();
    // "John Doe"
}

```

Parsing files

```rust
use csv_parser::core::parse_file;

fn main() {
    parse_file("my-csv.csv");
    // [
    //      {"age": "26", "name": "John Doe"},
    //      {"age": "18", "name": "Makoto Niijima" },
    //      {"age": "130", "name": "Geralt of Rivia"}
    // ]
}

```
