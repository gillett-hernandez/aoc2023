#![feature(iter_intersperse)]

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Gear {
    neighbors: [Option<usize>; 2],
}

impl Gear {
    // returns whether adding was successful. can fail if there's already two neighbors
    pub fn add_neighbor(&mut self, neighbor_part_number: usize) -> bool {
        let mut added = false;
        if self.neighbors[0].is_none() {
            self.neighbors[0] = Some(neighbor_part_number);
            added = true;
        } else if self.neighbors[1].is_none() {
            self.neighbors[1] = Some(neighbor_part_number);
            added = true;
        }
        added
    }
}

fn main() {
    let file = File::open("data/day03.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines_iter = reader.lines();

    let mut lines = vec![];
    while let Some(Ok(line)) = lines_iter.next() {
        lines.push(line);
    }

    let mut valid_symbols = Vec::new();
    for line in &lines {
        for char in line.chars() {
            if char.is_numeric() || char == '.' {
                continue;
            }
            valid_symbols.push(char);
        }
    }

    let mut separators = valid_symbols.clone();
    separators.push('.');

    // store stars in a vec, with position info and info about neighboring part numbers
    let mut gears = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char.is_numeric() || char == '.' {
                continue;
            }
            if char == '*' {
                gears.insert(
                    (x, y),
                    Gear {
                        neighbors: [None, None],
                    },
                );
            }
        }
    }

    let mut y_idx = 0;
    let mut sum = 0;
    for line in &lines {
        let mut numbers = line
            .split(&separators[..])
            .filter(|e| e.len() != 0)
            .collect::<Vec<&str>>();

        let numbers = numbers.drain(..).collect::<HashSet<&str>>();

        for number in numbers {
            let start_indices = (0..(1 + line.len() - number.len()))
                .filter_map(|idx| {
                    // this first part is to confirm the slice at this index matches the number
                    (&line[idx..idx + number.len()] == number
                        && (
                            // this second part is to confirm that the characters before the beginning or after the end (if extant)
                            // are not digits, thus preventing single digits from matching within other longer numbers on the same line
                            if idx == 0 {
                                // no character at idx-1, so yield true for previous character
                                true
                            } else {
                                let char_before_beginning = &line[idx - 1..idx];
                                // if not numeric
                                !char_before_beginning.chars().next().unwrap().is_numeric()
                            } && if let Some(char_after_end) =
                                line.get(idx + number.len()..idx + number.len() + 1)
                            {
                                !char_after_end.chars().next().unwrap().is_numeric()
                            } else {
                                true
                            }
                        ))
                    .then_some(idx)
                })
                .collect::<Vec<_>>();

            // scan all positions neighboring this number and see if there's a valid symbol
            for start_index in &start_indices {
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
                                || new_y as usize >= lines.len()
                                || new_x as usize >= line.len()
                            {
                                continue;
                            }
                            let char_at_location = (&lines[new_y as usize])
                                .chars()
                                .nth(new_x as usize)
                                .unwrap();
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
            // second pass for gear ratio data gathering
            for start_index in &start_indices {
                let mut local_gears_to_add_to = HashSet::new();
                for char_offset in 0..number.len() {
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
                                || new_y as usize >= lines.len()
                                || new_x as usize >= line.len()
                            {
                                continue;
                            }
                            let char_at_location = (&lines[new_y as usize])
                                .chars()
                                .nth(new_x as usize)
                                .unwrap();
                            if char_at_location == '*' {
                                // found neighboring symbol
                                local_gears_to_add_to.insert((new_x, new_y));
                            }
                        }
                    }
                }
                for (new_x, new_y) in local_gears_to_add_to {
                    println!("adding number {number} as neighbor to gear at ({new_x}, {new_y})");
                    let e = gears.get_mut(&(new_x as usize, new_y as usize)).unwrap();
                    e.add_neighbor(number.parse::<usize>().unwrap());
                }
            }
        }

        y_idx += 1;
    }
    let mut sum_gear_ratio = 0;
    for ((x, y), gear) in gears {
        let neighbors = gear.neighbors.iter().filter_map(|e| *e).collect::<Vec<_>>();
        if neighbors.len() == 2 {
            let ratio = neighbors[0] * neighbors[1];
            println!(
                "adding gear at line {} col {}: ratio {:?}",
                y + 1,
                x + 1,
                ratio
            );
            sum_gear_ratio += ratio;
        }
    }
    println!("{sum}, {sum_gear_ratio}");
}
