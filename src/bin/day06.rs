#![feature(iter_intersperse)]
use std::fs::File;
use std::io::{BufRead, BufReader};

fn ways(race_time_ms: u64, current_record_distance_mm: u64) -> u64 {
    // boat charges up speed at a rate of 1mm/1ms per ms
    // but every ms spent charging reduces time available to actually go

    let discriminant =
        ((race_time_ms * race_time_ms - 4 * current_record_distance_mm) as f32).sqrt();

    let t0_start = ((race_time_ms as f32 - discriminant) / 2.0).floor() as u64;
    let t1_end = ((race_time_ms as f32 + discriminant) / 2.0).ceil() as u64;

    let mut t0 = t0_start;
    while t0 * (race_time_ms - t0) <= current_record_distance_mm {
        t0 += 1;
    }

    let mut t1 = t1_end;
    while t1 * (race_time_ms - t1) <= current_record_distance_mm {
        t1 -= 1;
    }

    1 + t1 - t0
}

fn main() {
    let file = File::open("data/day06.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // pt 1
    let times = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|e| e.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let distances = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|e| e.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut prod = 1;
    for (time, distance) in times.iter().zip(distances.iter()) {
        prod *= ways(*time, *distance);
    }
    println!("{prod}");

    // pt 2
    let time = times
        .into_iter()
        .map(|e| e.to_string())
        .intersperse("".to_string())
        .collect::<String>().parse::<u64>().unwrap();
    let distance = distances
        .into_iter()
        .map(|e| e.to_string())
        .intersperse("".to_string())
        .collect::<String>().parse::<u64>().unwrap();
    println!("{}", ways(time, distance));
}
