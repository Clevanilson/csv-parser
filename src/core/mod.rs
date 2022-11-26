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
