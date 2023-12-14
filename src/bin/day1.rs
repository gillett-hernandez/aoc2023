use regex::Regex;

const DIGITS: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_all_matches<'a>(pattern: &Regex, line: &'a str) -> Vec<&'a str> {
    let mut matches = Vec::new();
    for i in 0..line.len() {
        match pattern.find_at(&line, i) {
            Some(p_match) => {
                matches.push(p_match.as_str());
            }
            None => {}
        }
    }
    matches
}

fn main() {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let file = File::open("data/day1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let digit_pattern = "\\d";
    let word_pattern = "(one|two|three|four|five|six|seven|eight|nine)";

    let actual_pattern = if false {
        Regex::new(digit_pattern).expect("failed to compile pattern")
    } else {
        Regex::new(&format!("({digit_pattern}|{word_pattern})")).expect("failed to compile pattern")
    };

    let mut sum = 0;

    while let Some(Ok(line)) = lines.next() {
        let first_and_last = {
            // use custom get_all_matches routine so that we don't skip overlapping patterns
            let all_matches = get_all_matches(&actual_pattern, &line);

            // use map to deref into the main string slice so that the references don't expire when all_matches goes out of scope
            // tried .as_deref but for some reason it didn't work
            let first = all_matches.first().map(|e| *e);
            let last = all_matches.last().map(|e| *e);
            (first, last)
        };

        match first_and_last {
            (Some(first), last) => {
                let first_str = first;
                let first_as_digit = match first_str.parse::<i32>() {
                    // parsed digit
                    Ok(digit) => digit,
                    // no digit parse, find index of actual digit in digits list
                    Err(_) => DIGITS
                        .iter()
                        .enumerate()
                        .find(|&(_, e)| *e == first_str)
                        .map(|(i, _)| 1 + i as i32)
                        .unwrap_or(-1),
                };

                let last_as_digit = last
                    .map(|e| {
                        let as_str = e;
                        match as_str.parse::<i32>() {
                            // parsed digit
                            Ok(digit) => Some(digit),
                            // no digit parse, find index of actual digit in digits list
                            Err(_) => DIGITS
                                .iter()
                                .enumerate()
                                .find(|&(_, e)| *e == as_str)
                                .map(|(i, _)| 1 + i as i32),
                        }
                    })
                    .flatten()
                    .unwrap_or(first_as_digit);
                if first_as_digit == -1 {
                    continue;
                }

                let v = first_as_digit * 10 + last_as_digit;
                println!("{v}");
                sum += v
            }

            _ => continue,
        }
    }
    println!("{sum}");
}
