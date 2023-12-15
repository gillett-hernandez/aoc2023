use lib::LocalError;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Game<const A: usize, const B: usize> {
    winning_numbers: [u32; A],
    drawn_numbers: [u32; B],
}

impl<const A: usize, const B: usize> Game<A, B> {
    pub fn num_winning(&self) -> u32 {
        let mut n = 0;
        for num in self.drawn_numbers {
            if self.winning_numbers.contains(&num) {
                n += 1;
            }
        }
        n
    }
    pub fn score(&self) -> u32 {
        let num_matching = self.num_winning();
        if num_matching == 0 || num_matching == 1 {
            num_matching
        } else {
            2u32.pow(num_matching - 1)
        }
    }
}

impl<const A: usize, const B: usize> TryFrom<&str> for Game<A, B> {
    type Error = LocalError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        //.map_err(|_| LocalError::default())?
        // .ok_or(LocalError::default())?
        let mut s = value.split(":");
        let id_chunk = s.next().ok_or(LocalError::default())?;
        let numbers_chunk = s.next().ok_or(LocalError::default())?;

        let _ = id_chunk
            .split_whitespace()
            .nth(1)
            .ok_or(LocalError::default())?
            .parse::<usize>()
            .map_err(|_| LocalError::default())?;

        let mut s = numbers_chunk.split("|");
        let mut left_nums = [0; A];
        for (i, left_num) in s
            .next()
            .ok_or(LocalError::default())?
            .split_whitespace()
            .filter_map(|e| (e.len() != 0).then_some(e.parse::<u32>().ok()).flatten())
            .enumerate()
        {
            if i >= A {
                break;
            }
            left_nums[i] = left_num;
        }
        let mut right_nums = [0; B];
        for (i, right_num) in s
            .next()
            .ok_or(LocalError::default())?
            .split_whitespace()
            .filter_map(|e| (e.len() != 0).then_some(e.parse::<u32>().ok()).flatten())
            .enumerate()
        {
            if i >= B {
                break;
            }
            right_nums[i] = right_num;
        }
        Ok(Game {
            winning_numbers: left_nums,
            drawn_numbers: right_nums,
        })
    }
}

fn main() {
    let file = File::open("data/day04.txt").unwrap();
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines().enumerate().map(|(line_number, e)| {
        e.expect(
            format!(
                "string parsing failed when reading line {} from file",
                line_number + 1
            )
            .as_str(),
        )
    }) {
        lines.push(line);
    }

    let mut sum = 0;
    let mut copy_counts = vec![0; lines.len()];
    for (idx, line) in lines.iter().enumerate() {
        let game: Game<10, 25> = line.as_str().try_into().unwrap();
        // let game: Game<5, 8> = line.as_str().try_into().unwrap();

        let num_matching = game.num_winning();

        for i in 0..num_matching {
            let target = idx + i as usize + 1;
            if target >= copy_counts.len() {
                break;
            }
            copy_counts[target] += 1 + copy_counts[idx];
        }

        let score = game.score();
        sum += score;
    }

    println!(
        "{sum}, {}",
        copy_counts.iter().sum::<i32>() + copy_counts.len() as i32
    );
}
