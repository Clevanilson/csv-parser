use csv_parser::core::parse_line;

#[test]
fn should_split_string_by_comma() {
    let line = "1,2";
    let result = parse_line(line);
    let expect = vec!["1", "2"];

    assert_eq!(result, expect);
}

#[test]
fn should_ignore_comma_inside_quotes() {
    let line = "1,2,\"3,4\"";
    let result = parse_line(line);
    let expect = vec!["1", "2", "3,4"];

    assert_eq!(result, expect);
}

#[test]
fn should_handle_empty_fields_at_the_beginning() {
    let line = ",2,3";
    let result = parse_line(line);
    let expect = vec!["", "2", "3"];

    assert_eq!(result, expect);
}

#[test]
fn should_handle_empty_fields_at_the_middle() {
    let line = "1,2,,4";
    let result = parse_line(line);
    let expect = vec!["1", "2", "", "4"];

    assert_eq!(result, expect);
}

#[test]
fn should_handle_empty_fields_at_the_end() {
    let line = "1,2,3,";
    let result = parse_line(line);
    let expect = vec!["1", "2", "3", ""];

    assert_eq!(result, expect);
}
