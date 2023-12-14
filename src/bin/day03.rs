#![feature(iter_intersperse)]

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("data/day03.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut data = vec![];
    while let Some(Ok(line)) = lines.next() {
        data.push(line);
    }

    let mut valid_symbols = Vec::new();
    for line in &data {
        for char in line.chars() {
            if char.is_numeric() || char == '.' {
                continue;
            }
            valid_symbols.push(char);
        }
    }

    let mut separators = valid_symbols.clone();
    separators.push('.');

    let mut y_idx = 0;
    let mut sum = 0;
    for line in &data {
        // line.split(&["!@#$%^&*_+=-."][..])
        let mut numbers = line
            .split(&separators[..])
            .filter(|e| e.len() != 0)
            .collect::<Vec<&str>>();

        // let mut should_print_stuff = false;
        // if numbers.len() != numbers.iter().map(|e| *e).collect::<HashSet<&str>>().len() {
        //     println!(
        //         "{}",
        //         numbers
        //             .iter()
        //             .map(|e| *e)
        //             .intersperse(&" ")
        //             .collect::<String>()
        //     );
        //     should_print_stuff = true;
        // }
        println!(
            "numbers in line = {}",
            numbers
                .iter()
                .map(|e| *e)
                .intersperse(&" ")
                .collect::<String>()
        );

        let numbers = numbers.drain(..).collect::<HashSet<&str>>();

        for number in numbers {
            let start_indices = (0..(1 + line.len() - number.len())).filter_map(|idx| {
                // first part is to confirm the slice at this index matches the number
                (&line[idx..idx + number.len()] == number
                    && (
                        // second part is to confirm that the characters before the beginning or after the end (if extant)
                        // are not digits, thus preventing single digits from matching within other longer numbers on the same line
                        if idx == 0 {
                            true
                        } else {
                            if let Some(char_before_beginning) = line.get(idx - 1..idx) {
                                // not numeric
                                !char_before_beginning.chars().next().unwrap().is_numeric()
                            } else {
                                true
                            }
                        } && if let Some(char_after_end) =
                            line.get(idx + number.len()..idx + number.len() + 1)
                        {
                            !char_after_end.chars().next().unwrap().is_numeric()
                        } else {
                            true
                        }
                    ))
                .then_some(idx)
            });

            // scan all positions neighboring this number and see if there's a valid symbol

            for start_index in start_indices {
                println!("{start_index}");
                'outer: for char_offset in 0..number.len() {
                    let x_idx = start_index + char_offset;
                    for x_offset in [-1, 0, 1] {
                        for y_offset in [-1, 0, 1] {
                            if x_offset == 0 && y_offset == 0 {
                                continue;
                            }
                            let new_y = y_idx as i32 + y_offset;
                            let new_x = x_idx as i32 + x_offset;

                            if new_y < 0
                                || new_x < 0
                                || new_y as usize >= data.len()
                                || new_x as usize >= line.len()
                            {
                                continue;
                            }
                            let char_at_location =
                                (&data[new_y as usize]).chars().nth(new_x as usize).unwrap();
                            if valid_symbols.contains(&char_at_location) {
                                // found neighboring symbol
                                println!("adding number {number} because symbol {char_at_location} ({new_x}, {new_y})");

                                sum += number.parse::<usize>().unwrap();
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }

        y_idx += 1;
    }
    println!("{sum}");
}
