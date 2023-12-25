/*
   | is a vertical pipe connecting north and south.
   - is a horizontal pipe connecting east and west.
   L is a 90-degree bend connecting north and east.
   J is a 90-degree bend connecting north and west.
   7 is a 90-degree bend connecting south and west.
   F is a 90-degree bend connecting south and east.
   . is ground; there is no pipe in this tile.
   S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
*/

use std::{
    collections::HashMap,
    fmt::{Display, Write},
    iter::once,
};

#[derive(Copy, Clone, PartialEq, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn iter_variants() -> impl Iterator<Item = Direction> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];
        DIRECTIONS.into_iter()
    }
}

impl Into<(isize, isize)> for Direction {
    fn into(self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }
}

impl TryFrom<(isize, isize)> for Direction {
    type Error = ();
    fn try_from(value: (isize, isize)) -> Result<Self, Self::Error> {
        match (value.0.signum(), value.1.signum()) {
            (-1, 0) => Ok(Direction::West),
            (1, 0) => Ok(Direction::East),
            (0, -1) => Ok(Direction::North),
            (0, 1) => Ok(Direction::South),
            _ => Err(()),
        }
    }
}

const OFFSET_MAP: [(char, Option<[Direction; 2]>); 8] = [
    ('|', Some([Direction::North, Direction::South])),
    ('-', Some([Direction::East, Direction::West])),
    ('L', Some([Direction::North, Direction::East])),
    ('J', Some([Direction::North, Direction::West])),
    ('7', Some([Direction::South, Direction::West])),
    ('F', Some([Direction::South, Direction::East])),
    ('.', None),
    ('S', None),
];

#[derive(Clone, Debug)]
struct TileMap {
    data: Vec<char>,
    width: usize,
    height: usize,
}

impl Display for TileMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match f.alternate() {
            true => f
                .debug_struct("Tilemap")
                .field("data", &self.data)
                .field("width", &self.width)
                .field("height", &self.height)
                .finish(),
            false => {
                for y in 0..self.height {
                    for char in &self.data[y * self.width..(y + 1) * self.width] {
                        f.write_char(*char)?;
                    }
                    f.write_char('\n')?;
                }
                Ok(())
            }
        }
    }
}

impl TileMap {
    pub fn get_char_at(&self, pos: (usize, usize)) -> Option<char> {
        self.data.get(pos.1 * self.width + pos.0).copied()
    }
    pub fn get_connected_pipes_at(&self, pos: (usize, usize)) -> Option<[(usize, usize); 2]> {
        // print!("getting pipes connected to ({}, {})", x, y);
        let Some(char_at_pos) = self.get_char_at(pos) else {
            println!(", char not found");
            return None;
        };
        // println!(", char = {}", char_at_pos);
        let mut maybe_valid_direction_pair = None;
        for (c, offset) in &OFFSET_MAP {
            if char_at_pos == 'S' {
                // valid directions for S character are ones that point into it, due to premises declared by the problem statement
                let mut directions = [None, None];
                for direction in Direction::iter_variants() {
                    let offset: (isize, isize) = direction.into();
                    let temp_pos = (
                        (pos.0 as isize + offset.0) as usize,
                        (pos.1 as isize + offset.1) as usize,
                    );
                    if let Some(connected_positions) = self.get_connected_pipes_at(temp_pos) {
                        for connected_pos in connected_positions {
                            if connected_pos == pos {
                                // found neighboring pipe pointing into this one
                                let count_some = directions.iter().filter_map(|e| *e).count();
                                if count_some >= 2 {
                                    panic!("found more than 2 neighboring pipes pointing into S");
                                } else {
                                    // if count_some == 0, then assigning will assign to the first entry
                                    // the subsequent iteration that finds the right pipe would then have count_some == 1, assigning to the next of the two slots
                                    // any subsequent iteration that finds a pipe pointing into S would then cause a panic
                                    directions[count_some] = Some(direction);
                                }
                            }
                        }
                    }
                }
                // println!("{:?}", directions);
                maybe_valid_direction_pair = Some([
                    directions[0].expect("somehow didn't have two directions"),
                    directions[1].expect("somehow didn't have two directions"),
                ]);
                break;
            } else if char_at_pos == *c {
                maybe_valid_direction_pair = *offset;
                break;
            }
        }

        let Some(pair) = maybe_valid_direction_pair else {
            return None;
        };

        let offset0: (isize, isize) = pair[0].into();
        let offset1: (isize, isize) = pair[1].into();

        Some([
            (
                (pos.0 as isize + offset0.0) as usize,
                (pos.1 as isize + offset0.1) as usize,
            ),
            (
                (pos.0 as isize + offset1.0) as usize,
                (pos.1 as isize + offset1.1) as usize,
            ),
        ])
    }

    /// structure of return value is (pos, distance_from_start)
    pub fn traverse(&self, initial_position: (usize, usize)) -> Vec<((usize, usize), usize)> {
        assert!(self.get_char_at(initial_position).unwrap_or('.') == 'S');
        let mut positions_and_distances: HashMap<(usize, usize), usize> = HashMap::new();

        // unwrapping because according to the problem statement, S alwyas has two connected pipes
        for traversal_branch in self.get_connected_pipes_at(initial_position).unwrap() {
            // traverse both ways, so that the distance stored is the minimum of the distance result for each traversal direction
            let mut last_pos = initial_position;
            let mut cur_pos = traversal_branch;
            let mut distance = 1usize;
            while cur_pos != initial_position {
                // on the first iteration of this loop, distance is 1 because cur_pos is already 1 away from the initial position
                // println!("{last_pos:?} -> {cur_pos:?}, cur dist {distance}");
                positions_and_distances
                    .entry(cur_pos)
                    .and_modify(|e| *e = (*e).min(distance))
                    .or_insert(distance);
                if let Some(current_neighbors) = self.get_connected_pipes_at(cur_pos) {
                    // println!("connected neighbors = {current_neighbors:?}");
                    for neighbor in current_neighbors {
                        if neighbor == last_pos {
                            continue;
                        }
                        // move to next position along loop

                        // println!("moving to {neighbor:?}, last pos = {cur_pos:?}");
                        last_pos = cur_pos;
                        cur_pos = neighbor;
                        distance += 1;
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        positions_and_distances.into_iter().collect::<Vec<_>>()
    }
}

fn main() {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let file = File::open("data/day10.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut tiledata = Vec::new();
    let mut map_width = 0;
    let mut map_height = 0;

    for line in lines.map(|e| e.unwrap()) {
        if map_width == 0 {
            map_width = line.len() + 2;

            // prepend line of '.'s
            tiledata.extend(once('.').cycle().take(map_width));
        }
        // prepend and append a vertical line of dots on the left and right respectively
        tiledata.push('.');
        tiledata.extend(line.chars());
        tiledata.push('.');
        map_height += 1;
    }
    // append line of '.'s
    tiledata.extend(once('.').cycle().take(map_width));

    // correct map height to new height, after adding dots
    map_height += 2;

    let mut map = TileMap {
        data: tiledata,
        width: map_width,
        height: map_height,
    };

    let animal_idx = map
        .data
        .iter()
        .enumerate()
        .find_map(|(i, e)| (*e == 'S').then_some(i))
        .unwrap();
    let animal_position = (animal_idx % map.width, animal_idx / map.width);

    // pt 1
    let mut max_dist = 0;
    let mut max_point = animal_position;
    let mut traversal = map.traverse(animal_position);
    for (point, distance) in &traversal {
        if *distance > max_dist {
            max_point = *point;
            max_dist = *distance;
        }
    }
    println!("{max_point:?} {max_dist}");

    // using traversal data from pt 1, work on pt 2
    // insert starting point so that there's no holes in the loop
    traversal.push((animal_position, 0));

    let neighbors = map.get_connected_pipes_at(animal_position).unwrap();
    let mut directions = [Direction::West, Direction::West];
    for (i, neighbor) in neighbors.iter().enumerate() {
        let offset = (
            neighbor.0 as isize - animal_position.0 as isize,
            neighbor.1 as isize - animal_position.1 as isize,
        );
        let as_direction = Direction::try_from(offset).unwrap();
        directions[i] = as_direction;
    }

    let matching_character = OFFSET_MAP
        .iter()
        .find_map(|(c, maybe_offset)| {
            if let Some(offset) = maybe_offset {
                if (offset[0] == directions[0] && offset[1] == directions[1])
                    || (offset[0] == directions[1] && offset[0] == directions[1])
                {
                    Some(*c)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap();

    map.data[animal_position.1 * map.width + animal_position.0] = matching_character;

    let traversal = traversal.into_iter().collect::<HashMap<_, _>>();
    for y in 0..map.height {
        for x in 0..map.width {
            if !traversal.contains_key(&(x, y)) {
                // delete irrelevant pipes
                map.data[y * map.width + x] = '.';
            }
        }
    }

    let mut sum_inner = 0;
    for y in 0..map.height {
        let mut is_outside = true;
        for x in 0..map.width {
            let c = map.data[y * map.width + x];
            // use polygon point testing method, with the point for testing being slightly up (north) and left (west) and pointing to the right
            if c == '|' || c == 'J' || c == 'L' {
                // found boundary
                is_outside = !is_outside;
            }
            if c == '.' {
                match is_outside {
                    true => map.data[y * map.width + x] = 'O',
                    false => {
                        sum_inner += 1;
                        map.data[y * map.width + x] = 'I'
                    }
                }
            }
        }
    }
    println!("{}", sum_inner);
}
