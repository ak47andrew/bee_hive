use rand::{rng, Rng};
use crate::cli::Mode;
use crate::translator::compiler::CODE_CHARS;

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

    output.iter().filter(
        |x| !x.as_str().trim_start_matches(' ').is_empty()
    ).map(|x| x.clone()).collect()
}

pub fn split_statements(code: &str) -> Vec<String> { lines(cleanup_comments(code)) }

pub fn obfuscate(input: Vec<u8>) -> Vec<u8> {
    let mut rng = rng();
    let mut output = Vec::new();
    let limited = CODE_CHARS.as_bytes();

    let count = rng.random_range(1..=20);
    for _ in 0..count {
        let v = rng.random();
        if !limited.contains(&v) {
            output.push(v);
        }
    }

    for b in input {
        output.push(b);

        let count = rng.random_range(1..=20);
        for _ in 0..count {
            let v = rng.random();
            if !limited.contains(&v) {
                output.push(v);
            }
        }
    }

    output
}

pub fn post_processing(code: &str, mode: Mode) -> Vec<u8> {
    match mode {
        Mode::Normal => {
            let mut co = code.to_string();
            co.retain(|c| CODE_CHARS.contains(c) || c == '\n');
            co.into_bytes()
        }
        Mode::Minimized => {
            let mut co = code.to_string();
            co.retain(|c| CODE_CHARS.contains(c));
            co.into_bytes()
        }
        Mode::Obfuscated => {
            obfuscate(post_processing(code, Mode::Minimized))
        }
        Mode::Extended => {
            code.as_bytes().to_vec()
        }
    }
}
