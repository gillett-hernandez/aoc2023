use std::{fmt::Display, iter::once};

#[derive(Copy, Clone, Debug)]
enum HorizontalCell {
    Galaxy,
    Space(Range<i32>),
}

enum VerticalCell {
    Line(Vec<HorizontalCell>),
    Space(Range<i32>),
}

struct Universe {
    pub data: Vec<VerticalCell>,
    pub width: usize,
    pub height: usize,
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
        if self.width > 100 || self.height > 100 {
            
            return Ok(());
        }
        // need to rewrite printing logic
        } else {
            let _ = f.write_fmt(format_args!("Universe(Vec with {} * sizeof(Cell) bytes, {}, {})", self.data.iter().map(|e| e.len()).sum::<usize>(), self.width, self.height));
        }
        Ok(())
    }
}

impl Universe {
    fn expand(&mut self, rows_to_expand: Vec<usize>, columns_to_expand: Vec<usize>, expansiom_multiplier: usize) {
        // perform row expansion first, for no specific reason

        for row in rows_to_expand.iter().rev() {
            // iter in reverse order so that expanding the vec doesn't cause our indexes to become invalid
            // which is what would happen if we expand an earlier row and then go to a later row

            // each of the rows to expand should each be already empty space
            
            let row = self.data[*row];
            
        }

        self.height += rows_to_expand.len();

        println!("{}", self);
        self.transpose();
        println!("{}", self);
        // swaps width and height

        // repeat above step but with columns
        for row in columns_to_expand.iter().rev() {
            // iter in reverse order so that expanding the vec doesn't cause our indexes to become invalid
            // which is what would happen if we expand an earlier row and then go to a later row

            self.data.extend(once('.').cycle().take(self.width));

            let row_begin_index = *row * self.width;
            let row_end_index = row_begin_index + self.width;

            // shift all elements down a row
            for i in (row_end_index..self.data.len()).rev() {
                self.data[i] = self.data[i - self.width];
            }
        }
        self.height += columns_to_expand.len();

        // transpose again
        self.transpose();
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

    let mut width = 0; // setting to 0 to satisfy compiler
                       // but the value will always be set on the first iteration of the following loop, since data will be empty.

    let mut height = 0;
    for line in lines.map(|e| e.unwrap()) {
        if data.len() == 0 {
            width = line.len();
        }
        data.extend(line.chars());
        height += 1;
    }

    let mut universe = Universe {
        data,
        width,
        height,
    };

    let mut rows_to_expand = Vec::new();
    for y in 0..height {
        if universe.data[y * width..(y + 1) * width]
            .iter()
            .all(|e| *e != '#')
        {
            rows_to_expand.push(y);
        }
    }

    let mut columns_to_expand = Vec::new();
    for x in 0..width {
        if universe
            .data
            .iter()
            .skip(x)
            .step_by(width)
            .all(|e| *e != '#')
        {
            columns_to_expand.push(x);
        }
    }
    println!("{:?}, {:?}", rows_to_expand, columns_to_expand);

    universe.expand(rows_to_expand, columns_to_expand);

    println!("{}", universe);
    
    let mut galaxies = Vec::new();
    for y in 0..universe.height {
        for x in 0..universe.width {
            if universe.data[y * universe.width + x] == '#' {
                galaxies.push((x as isize,y as isize));
            }
        }
    }
    
    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in 0..i {
            let gi = galaxies[i];
            let gj = galaxies[j];
            
            sum += (gi.0 - gj.0).abs() + (gi.1-gj.1).abs();
        }
    }
    println!("{sum}");
    
    //pt 2 will likely require a sparse matrix implementation of some sort
    
}