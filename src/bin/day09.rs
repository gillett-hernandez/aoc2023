fn predict_next_term(seq: &Vec<i32>) -> i32 {
    let mut differences = seq.clone();
    differences.clear();
    let mut last_differences = differences.clone();
    //ascend levels, computing differences
    loop {
        
    }
    // append 0
    
    // descend levels, computing the next term at each level
    loop {
        
    }
    differences.last().unwrap()
}
fn main() {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let file = File::open("samples/day09.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut sequences = Vec::new();
    
    while let Some(Ok(line)) = lines.next() {
      sequences.push(line.split_whitespace().map(|e|e.parse::<i32>().unwrap()).collect::<Vec<_>>());
    }
    
    for seq in &sequences {
        println!("{}", predict_next_term(seq));
    }
    
    println!("{sequences:?}");
}
