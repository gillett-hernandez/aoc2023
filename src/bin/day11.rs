use std::{fmt::Display, mem::size_of};

#[derive(Clone, PartialEq, Debug)]
enum HorizontalCell {
    Galaxy,
    Space(usize),
}

#[derive(Clone, PartialEq, Debug)]
enum VerticalCell {
    Line(Vec<HorizontalCell>),
    Space(usize),
}

struct Universe {
    pub data: Vec<VerticalCell>,
    pub width: usize,
    pub height: usize,
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            if self.width > 1000 || self.height > 1000 {
                return Ok(());
            }
            for vcell in self.data.iter() {
                match vcell {
                    VerticalCell::Line(hcells) => {
                        print!("[");
                        for hcell in hcells {
                            match hcell {
                                HorizontalCell::Galaxy => print!("#, "),
                                HorizontalCell::Space(width) => print!("Space({width}), "),
                            }
                        }
                        println!("],");
                    }
                    VerticalCell::Space(height) => {
                        println!("VerticalCell({}, {height})", self.width);
                    }
                }
            }
        } else {
            let _ = f.write_fmt(format_args!(
                "Universe(Vec with {} bytes, {}, {})",
                self.data
                    .iter()
                    .map(|e: &VerticalCell| {
                        let mut size = 0;
                        match e {
                            VerticalCell::Line(l) => {
                                size += l.len() * size_of::<HorizontalCell>();
                            }
                            VerticalCell::Space(_) => size += 16,
                        }
                        size
                    })
                    .sum::<usize>(),
                self.width,
                self.height
            ));
        }
        Ok(())
    }
}

impl Universe {
    fn expand(
        &mut self,
        rows_to_expand: Vec<usize>,
        columns_to_expand: Vec<usize>,
        expansion_multiplier: usize,
    ) {
        // perform row expansion first, for no specific reason

        for row in rows_to_expand.iter().rev() {
            // iter in reverse order so that expanding the vec doesn't cause our indexes to become invalid
            // which is what would happen if we expand an earlier row and then go to a later row

            // each of the rows to expand should each be already empty space

            let row = &mut self.data[*row];
            if let VerticalCell::Line(arr) = row.clone() {
                if arr.len() == 1 {
                    if let HorizontalCell::Space(_) = &arr[0] {
                        // found a vertical cell composed of a single line of empty space
                        *row = VerticalCell::Space(expansion_multiplier); // convert to a big chunk of empty space, with size equal to the expansion amount
                    }
                }
            }
        }

        self.height += rows_to_expand.len() * (expansion_multiplier - 1);

        for column in columns_to_expand.iter().rev() {
            for vcell in self.data.iter_mut() {
                match vcell {
                    VerticalCell::Line(hcells) => {
                        let mut i = 0;
                        for hcell in hcells.iter_mut() {
                            match hcell {
                                HorizontalCell::Galaxy => i += 1,
                                HorizontalCell::Space(width) => {
                                    if *column >= i && *column < i + *width {
                                        // found space chunk containing `column`
                                        // expand it
                                        *width += expansion_multiplier - 1;
                                        // break, since we already found `column`
                                        break;
                                    }
                                    i += *width;
                                }
                            }
                        }
                    }
                    VerticalCell::Space(_) => (), // do nothing if we encounter a space cell, as that can be thought of as auto expanding to fill horizontal space
                }
            }
        }
        self.width += columns_to_expand.len() * (expansion_multiplier - 1);
    }

    fn all_galaxy_positions(&self) -> Vec<(usize, usize)> {
        let mut out = Vec::new();
        let mut y = 0;
        for vcell in &self.data {
            match vcell {
                VerticalCell::Line(hcells) => {
                    let mut x = 0;
                    for hcell in hcells {
                        match hcell {
                            HorizontalCell::Galaxy => {
                                out.push((x, y));
                                x += 1;
                            }
                            HorizontalCell::Space(width) => x += width,
                        }
                    }
                    y += 1;
                }
                VerticalCell::Space(height) => {
                    y += height;
                }
            }
        }
        out
    }
}

fn main() {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let file = File::open("data/day11.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    // just put it into a vec
    let mut data = Vec::new();

    let mut width = 0; // setting to 0 to satisfy compiler "maybe uninit" warning
                       // but the value will always be set on the first iteration of the following loop, since data will be empty.

    let mut height = 0;
    for line in lines.map(|e| e.unwrap()) {
        if data.len() == 0 {
            width = line.len();
        }
        data.extend(line.chars());
        height += 1;
    }

    let mut v_cells = Vec::new();

    for y in 0..height {
        let mut h_cells = Vec::new();
        let chars = &data[y * width..(y + 1) * width];
        let mut last_galaxy_index = chars
            .iter()
            .enumerate()
            .find_map(|e| (*e.1 == '#').then_some(e.0))
            .unwrap_or(chars.len());
        if last_galaxy_index != 0 {
            h_cells.push(HorizontalCell::Space(last_galaxy_index));
        }

        if last_galaxy_index < chars.len() {
            h_cells.push(HorizontalCell::Galaxy);
        }
        let mut i = last_galaxy_index + 1;
        while i < chars.len() {
            if chars[i] == '#' {
                if last_galaxy_index + 1 < i {
                    h_cells.push(HorizontalCell::Space(i - last_galaxy_index));
                }
                h_cells.push(HorizontalCell::Galaxy);
                last_galaxy_index = i;
            }
            i += 1;
        }

        if last_galaxy_index + 1 < i {
            h_cells.push(HorizontalCell::Space(i - last_galaxy_index - 1));
        }
        println!("{:?}", h_cells);
        v_cells.push(VerticalCell::Line(h_cells));
    }

    let mut universe = Universe {
        data: v_cells,
        width,
        height,
    };

    let mut rows_to_expand = Vec::new();
    for y in 0..height {
        if data[y * width..(y + 1) * width].iter().all(|e| *e != '#') {
            rows_to_expand.push(y);
        }
    }

    let mut columns_to_expand = Vec::new();
    for x in 0..width {
        if data.iter().skip(x).step_by(width).all(|e| *e != '#') {
            columns_to_expand.push(x);
        }
    }

    println!();
    println!();
    universe.expand(rows_to_expand, columns_to_expand, 2);

    println!("{:#}", universe);
    println!("width {}, height {}", universe.width, universe.height);

    let galaxies = universe
        .all_galaxy_positions()
        .into_iter()
        .map(|e| (e.0 as isize, e.1 as isize))
        .collect::<Vec<_>>();

    for g in &galaxies {
        println!("{:?}", g);
    }
    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in 0..i {
            let gi = galaxies[i];
            let gj = galaxies[j];

            sum += (gi.0 - gj.0).abs() + (gi.1 - gj.1).abs();
        }
    }
    println!("{sum}");

    //pt 2 will likely require a sparse matrix implementation of some sort
}
