use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn gcd(mut a:usize, mut b:usize) -> usize {
    //best to use euclidean algorithm
    if a < b {
        std::mem::swap(&mut a, &mut b);
    }
    let mut remainder = a % b;
    while a % remainder != 0 && b % remainder != 0 {
        std::mem::swap(&mut a, &mut b);
        std::mem::swap(&mut b, &mut remainder);
        remainder = a % b;
    }
    remainder
}

fn main() {
    let file = File::open("data/day08.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut network = HashMap::new();

    let pattern = lines.next().unwrap().unwrap();

    let mut pattern_inf_iter = pattern.chars().into_iter().cycle();

    while let Some(Ok(line)) = lines.next() {
        if line.len() < 1 {
            continue;
        }
        let mut split = line.split("=");
        let Some(node) = split.next().map(|e| e.trim()) else {
            break;
        };
        let Some(connections) = split.next() else {
            break;
        };
        let connections = connections.trim();
        let mut split = connections.split(", ");
        let left_connection = split.next().unwrap()[1..].to_owned();
        let right_connection = split.next().unwrap()[..3].to_owned();

        network.insert(node.to_owned(), (left_connection, right_connection));
    }

    // println!("{:?}", network);

    let mut current_node = "AAA";
    let mut step_count = 0;
    while current_node != "ZZZ" {
        current_node = match pattern_inf_iter.next().unwrap() {
            'L' => network[current_node].0.as_str(),
            'R' => network[current_node].1.as_str(),
            _ => panic!(),
        };
        step_count += 1;
    }
    println!("took {step_count} steps to reach ZZZ");

    let starting_nodes = network
        .keys()
        .filter(|e| e.chars().nth(2).unwrap() == 'A')
        .map(|e| e.as_str())
        .collect::<Vec<_>>();

    let mut cycle_lengths = Vec::new();
    for starting_node in starting_nodes {
        let mut pattern_inf_iter = pattern.chars().into_iter().cycle();
        let mut current_node = starting_node;
        let mut step_count = 0usize;
        while current_node.chars().nth(2).unwrap() != 'Z' {
            current_node = match pattern_inf_iter.next().unwrap() {
                'L' => network[current_node].0.as_str(),
                'R' => network[current_node].1.as_str(),
                _ => panic!(),
            };
            step_count += 1;
        }
        cycle_lengths.push(step_count);
    }

    // compute lcm of numbers
    let lcm: usize = cycle_lengths.into_iter().reduce(|a,b| a*b/gcd(a,b)).unwrap();
    println!("{lcm}");
}
