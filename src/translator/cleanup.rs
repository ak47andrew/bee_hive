fn cleanup_comments(code: &str) -> String {
    code
        .lines()
        .map(|line| {
            let mut in_string = false;
            let mut result = String::new();

            for c in line.chars() {
                match c {
                    '"' => {
                        in_string = !in_string;
                        result.push(c);
                    }
                    '#' if !in_string => {
                        break;
                    }
                    _ => result.push(c),
                }
            }

            result
        })
        .collect::<Vec<_>>()
        .join("\n")
}


fn lines(code: String) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut tmp = String::new();

    for statement in code.split(';'){
        tmp.push_str(statement);
        if tmp.matches('"').count() % 2 == 0{
            output.push(tmp);
            tmp = String::new();
        }
    }

    return output.iter().filter(
        |x| !x.as_str().trim_start_matches(' ').is_empty()
    ).map(|x| x.clone()).collect();
}

pub fn split_statements(code: &str) -> Vec<String> { lines(cleanup_comments(code)) }
