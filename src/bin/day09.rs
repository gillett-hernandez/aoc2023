fn predict_next_term(seq: &Vec<i32>) -> i32 {
    let mut last_differences = seq.clone();
    let mut differences = Vec::with_capacity(seq.len()); // match capacity of seq

    // store offsets of each level, with the 0th offset being the first term in the sequence
    let mut offsets = Vec::new();
    let mut level = 0;
    //ascend levels, computing differences

    loop {
        differences.clear();
        offsets.push(*last_differences.first().unwrap());
        for w in last_differences.windows(2) {
            differences.push(w[1] - w[0]);
        }

        level += 1;

        if differences.iter().all(|e| *e == 0) {
            std::mem::swap(&mut last_differences, &mut differences);
            break;
        }
        std::mem::swap(&mut last_differences, &mut differences);
    }

    // append 0
    last_differences.push(0);
    differences.push(*differences.last().unwrap());
    level -= 1;

    std::mem::swap(&mut last_differences, &mut differences);

    // and reconstruct
    loop {
        last_differences.clear();
        let mut v = offsets[level];
        last_differences.push(v);

        for delta in &differences {
            v += delta;
            last_differences.push(v);
        }
        std::mem::swap(&mut last_differences, &mut differences);
        if level > 0 {
            level -= 1;
        } else {
            break;
        }
    }
    *differences.last().unwrap()
}

// fn predict_previous_term(seq: &Vec<i32>) -> i32 {
//     let mut last_differences = seq.clone();
//     let mut differences = Vec::with_capacity(seq.len()); // match capacity of seq

//     // store offsets of each level, with the 0th offset being the first term in the sequence
//     let mut offsets = Vec::new();
//     let mut level = 0;
//     //ascend levels, computing differences

//     loop {
//         differences.clear();
//         offsets.push(*last_differences.first().unwrap());
//         for w in last_differences.windows(2) {
//             differences.push(w[1] - w[0]);
//         }

//         level += 1;

//         if differences.iter().all(|e| *e == 0) {
//             std::mem::swap(&mut last_differences, &mut differences);
//             break;
//         }
//         std::mem::swap(&mut last_differences, &mut differences);
//     }

//     // append 0
//     last_differences.push(0);
//     differences.push(*differences.last().unwrap());
//     level -= 1;

//     std::mem::swap(&mut last_differences, &mut differences);

//     // and reconstruct
//     loop {
//         last_differences.clear();
//         let mut v = offsets[level];
//         last_differences.push(v);

//         for delta in &differences {
//             v += delta;
//             last_differences.push(v);
//         }
//         std::mem::swap(&mut last_differences, &mut differences);
//         if level > 0 {
//             level -= 1;
//         } else {
//             break;
//         }
//     }
//     *differences.last().unwrap()
// }

fn main() {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let file = File::open("data/day09.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut sequences = Vec::new();

    while let Some(Ok(line)) = lines.next() {
        sequences.push(
            line.split_whitespace()
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<_>>(),
        );
    }

    let mut sum_predicted = 0;
    let mut sum_prior = 0;
    for seq in &sequences {
        // println!("{seq:?} {}\n", predict_next_term(seq));
        sum_predicted += predict_next_term(seq);
        let reversed = seq.iter().cloned().rev().collect::<Vec<_>>();
        sum_prior += predict_next_term(&reversed);
    }
    println!("{sum_predicted}, {sum_prior}");
}
