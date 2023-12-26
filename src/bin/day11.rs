use std::{fmt::Display, iter::once};

struct Universe {
    pub data: Vec<char>,
    pub width: usize,
    pub height: usize,
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            let joined = self.data[y * self.width..(y + 1) * self.width]
                .iter()
                .cloned()
                .collect::<String>();

            let _ = f.write_fmt(format_args!("{}\n", joined.as_str()));
        }
        Ok(())
    }
}

impl Universe {
    fn transpose(&mut self) {
        let mut clone = self.data.clone();
        clone.clear();

        for x in 0..self.width {
            for y in 0..self.height {
                clone.push(self.data[y * self.width + x]);
            }
        }

        self.data = clone;
        std::mem::swap(&mut self.width, &mut self.height);
    }
    fn expand(&mut self, rows_to_expand: Vec<usize>, columns_to_expand: Vec<usize>) {
        // perform row expansion first, for no specific reason

        for row in rows_to_expand.iter().rev() {
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

        self.height += rows_to_expand.len();

        self.transpose();
        // transposing swaps rows and columns, and width and height

        // repeat above step but with columns, but treat them as rows because we transposed
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
    let file = File::open("samples/day11.txt").unwrap();
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

    universe.expand(rows_to_expand, columns_to_expand);

}
