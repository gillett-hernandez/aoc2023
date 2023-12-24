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

#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Into<(isize, isize)> for Direction {
    fn into(self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
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

struct TileMap {
    data: Vec<char>,
    width: usize,
    height: usize,
}

impl TileMap {
    pub fn get_connected_pipes_at(&self, x: usize, y: usize) -> [Option<(usize, usize)>; 2] {
        let Some(char_at_pos) = self.data.get(y * self.width + x) else {
            return [None, None];
        };
        let mut maybe_valid_direction_pair = None;
        for (c, offset) in &OFFSET_MAP {
            if *char_at_pos == *c {
                maybe_valid_direction_pair = *offset;
                break;
            }
        }

        let mut res = [None, None];

        let Some(pair) = maybe_valid_direction_pair else {
            return res;
        };

        let offset0: (isize, isize) = pair[0].into();
        let offset1: (isize, isize) = pair[1].into();

        res
    }
}

fn main() {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let file = File::open("data/day1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut tiledata = Vec::new();
    let mut map_width = 0;
    let mut map_height = 0;

    for line in lines.map(|e| e.unwrap()) {
        if map_width == 0 {
            map_width = line.len();
        }
        tiledata.extend(line.chars());
        map_height += 1;
    }

    let map = TileMap {
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
    
}
