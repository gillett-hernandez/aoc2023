use lib::LocalError;
use std::fs::File;
use std::io::{BufRead, BufReader};

// enum BallColor {
//     Blue,
//     Red,
//     Green,
// }

#[derive(Copy, Clone, Debug)]
struct Pull {
    // a pull from a sack of colored balls
    pub balls: [usize; 3],
}

#[derive(Clone, Debug)]
struct Game {
    pub id: usize,
    pub pulls: Vec<Pull>,
}

impl TryFrom<&str> for Pull {
    type Error = LocalError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // rgb order
        let mut balls = [0; 3];
        for color_count_pair in value.split(", ") {
            let mut split = color_count_pair.split_whitespace().map(|e| e.trim());
            let count = split
                .next()
                .ok_or(LocalError::default())?
                .parse::<usize>()
                .map_err(|_| LocalError::default())?;

            let color = split.next().ok_or(LocalError::default())?;

            match color {
                "red" => balls[0] = count,
                "green" => balls[1] = count,
                "blue" => balls[2] = count,
                _ => return Err(LocalError::default()),
            }
        }
        Ok(Pull { balls })
    }
}

impl TryFrom<&str> for Game {
    type Error = LocalError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split = value.split(":");
        let game_id_chunk = split.next().ok_or(LocalError::default())?;
        let game_data_chunk = split.next().ok_or(LocalError::default())?;

        let game_id = game_id_chunk
            .split(" ")
            .last()
            .ok_or(LocalError::default())?
            .parse::<usize>()
            .map_err(|_| LocalError::default())?;

        let mut pulls = Vec::new();
        for pull in game_data_chunk.split(";") {
            let pull: Pull = pull.trim().try_into()?;
            // did not successfully parse pull into pull
            pulls.push(pull);
        }
        Ok(Game { id: game_id, pulls })
    }
}

fn main() {
    let file = File::open("data/day2.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut sum_valid_game_ids = 0;
    let mut sum_game_powers = 0; // not filtered by valid games for the specified limit

    let maxs = [12, 13, 14];
    while let Some(Ok(line)) = lines.next() {
        let Ok(game): Result<Game, LocalError> = line.as_str().try_into() else {
            continue;
        };

        let mut is_valid = true;
        let mut game_min_balls = [0; 3];
        for pull in &game.pulls {
            for i in 0..3 {
                if pull.balls[i] > maxs[i] {
                    // not valid game
                    is_valid = false;
                }
                if pull.balls[i] > game_min_balls[i] {
                    game_min_balls[i] = pull.balls[i];
                }
            }
        }
        let game_min_power = game_min_balls.iter().product::<usize>();

        sum_game_powers += game_min_power;

        if is_valid {
            sum_valid_game_ids += game.id;
        }
    }
    println!("{sum_valid_game_ids}, {sum_game_powers}");
}
