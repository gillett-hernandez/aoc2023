
fn main() {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let file = File::open("data/day1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();


}
