// fn generate_partitions(n: usize, big_n: usize) -> Vec<Vec<usize>> {
//     let mut out = Vec::new();
//     for i in 0..n {
//         for sub in generate_partitions(n - i, big_n - 1) {
//             let mut local = vec![0; big_n];
//             local[0] = i;
//             for (i, e) in sub.iter().enumerate() {
//                 local[i + 1] = *e;
//             }
//             out.push(local);
//         }
//     }
//     out
// }

struct SpringPattern {
    pub contiguous_strides: Vec<u32>,
    pub total_length: u32,
}

impl SpringPattern {
    fn strides(string: &str) -> Vec<u32> {
        string
            .split(".")
            .filter_map(|e| (e.len() != 0).then_some(e.len() as u32))
            .collect::<Vec<_>>()
    }
    fn generate_all_valid_patterns(&self, test_pattern: &str) -> Vec<String> {
        // just brute force it lol
        // check all possible patterns by iteratively replacing question marks with broken springs or working springs
        // then filter by contiguous

        let all_question_positions = test_pattern
            .chars()
            .enumerate()
            .filter_map(|(i, e)| (e == '?').then_some(i))
            .collect::<Vec<_>>();

        if all_question_positions.len() > 64 {
            panic!();
        }
        let mut out = Vec::new();
        for mask in 0..2u64.pow(all_question_positions.len() as u32) {
            let mut string = test_pattern.to_string();
            for (mask_i, string_i) in all_question_positions.iter().enumerate() {
                if (mask >> mask_i) & 1 == 1 {
                    string.replace_range(*string_i..(1 + *string_i), "#");
                } else {
                    string.replace_range(*string_i..(1 + *string_i), ".");
                }
            }
            if SpringPattern::strides(string.as_str()) == self.contiguous_strides {
                out.push(string);
            }
        }
        out
    }
}

fn main() {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let file = File::open("data/day12.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut all_patterns = Vec::new();
    let mut sum = 0;
    for line in lines.map(|e| e.unwrap()) {
        if line.len() == 0 {
            break;
        }
        // println!("{}", line);
        let mut split = line.split_whitespace();
        let test_pattern = split.next().unwrap().to_string();
        let contiguous_strides = split
            .next()
            .unwrap()
            .split(",")
            .map(|e| e.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let total = test_pattern.len();
        let spring_pattern = SpringPattern {
            contiguous_strides,
            total_length: total as u32,
        };

        let count = spring_pattern
            .generate_all_valid_patterns(test_pattern.as_str())
            .len();
        all_patterns.push((spring_pattern, test_pattern));

        sum += count;
    }
    println!("pt1: {sum}");

    // expand patterns
    for (stride_pattern, test_pattern) in all_patterns.iter_mut() {
        let range = 0..stride_pattern.contiguous_strides.len();
        let l = stride_pattern.total_length;
        let p = test_pattern.clone();

        for _ in 0..5 {
            stride_pattern
                .contiguous_strides
                .extend_from_within(range.clone());
            stride_pattern.total_length += l;
            test_pattern.push_str(p.as_str());
        }
        // println!("{:?} {}", stride_pattern.contiguous_strides, test_pattern);
    }

    // for (sp, tp) in all_patterns.iter() {
    //     println!(
    //         "{}",
    //         2u64.pow(
    //             tp.chars()
    //                 .enumerate()
    //                 .filter_map(|(i, e)| (e == '?').then_some(i))
    //                 .count() as u32
    //         )
    //     );
    // }
}
