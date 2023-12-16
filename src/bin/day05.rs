#![feature(iter_next_chunk)]

use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Copy, Clone, Debug)]
struct MapDataEntry {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

fn map_seed_to_location(mut seed: u64, maps: &HashMap<&str, (&str, Vec<MapDataEntry>)>) -> u64 {
    let mut current_layer = "seed";
    while current_layer != "location" {
        let (new_layer, map_data) = &maps[current_layer];

        for map_data_entry in map_data {
            if (map_data_entry.source_range_start
                ..(map_data_entry.source_range_start + map_data_entry.range_length))
                .contains(&seed)
            {
                let new_seed = seed - map_data_entry.source_range_start
                    + map_data_entry.destination_range_start;

                seed = new_seed;
                break;
            }
        }
        current_layer = new_layer;
    }
    seed
}

fn main() {
    let file = File::open("data/day05.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    let _ = reader
        .read_to_string(&mut data)
        .expect("failed to read data into string");

    let mut maps = HashMap::new();

    let Some(first_eol) = data.find("\n") else {
        return;
    };
    let seeds = data[0..first_eol]
        .split(" ")
        .skip(1)
        .map(|e| e.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    println!("{seeds:?}");

    // parse map chunks
    let map_chunks = data[first_eol..].split("\n\n").skip(1).collect::<Vec<_>>();
    for chunk in map_chunks {
        let mut iter = chunk.split("\n");

        // parse label into pieces
        let Some(full_label) = iter.next() else {
            continue;
        };
        let (source, target) = {
            let shortened = &full_label[0..full_label.len() - 5];
            let mut split = shortened.split("-");
            (split.next().unwrap(), split.nth(1).unwrap())
        };

        // parse MapDataEntry's
        let data = iter
            .map(|e| {
                let mut tmp_dest = [0; 3];
                e.split_whitespace()
                    .map(|e| e.parse::<u64>().unwrap())
                    .enumerate()
                    .for_each(|(i, e)| {
                        tmp_dest[i] = e;
                    });

                MapDataEntry {
                    destination_range_start: tmp_dest[0],
                    source_range_start: tmp_dest[1],
                    range_length: tmp_dest[2],
                }
            })
            .collect::<Vec<_>>();

        maps.insert(source, (target, data));
    }

    for map in &maps {
        println!("{map:?}");
    }

    let mut min_location_from_initial = u64::MAX;

    for v in seeds.clone() {
        let location = map_seed_to_location(v, &maps);
        if location < min_location_from_initial {
            min_location_from_initial = location;
            println!("(seeds) new minimum location found: {min_location_from_initial}");
        }
    }

    let mut min_location_in_spans = u64::MAX;
    let mut iter = seeds.iter();
    while let Ok(window) = iter.next_chunk::<2>() {
        // for v in *window[0]..(*window[0] + *window[1]) {
        //     let location = map_seed_to_location(v, &maps);
        //     if location < min_location_in_spans {
        //         min_location_in_spans = location;
        //         println!("(spans) new minimum location found: {min_location_in_spans}");
        //     }
        // }
        let min_for_range = (*window[0]..(*window[0] + *window[1]))
            .into_par_iter()
            .map(|e| map_seed_to_location(e, &maps))
            .min()
            .unwrap();
        if min_for_range < min_location_in_spans {
            min_location_in_spans = min_for_range;
            println!("(spans) new minimum location found: {min_location_in_spans}");
        }
    }
    println!("min location from single seeds is {min_location_from_initial}");
    println!("min location from spans is {min_location_in_spans}");
}
