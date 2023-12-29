fn generate_partitions(n: usize, big_n: usize) -> Vec<Vec<usize>> {
    let mut out = Vec::new();
    for i in 0..n {
        for sub in generate_partitions(n-i, big_n-1) {
            let mut local = vec![0; big_n];
            local[0]=i;
            for (i, e) in sub.iter().enumerate(){
                local[i+1]=*e;
            }
            out.push(local);
        }
    }
    out
}

struct SpringPattern {
    pub contiguous_strides: Vec<u32>,
    pub total_length: u32,
}

impl SpringPattern {
    fn generate_all_valid_patterns(&self, test_pattern: &str) -> Vec<String>{
        // every stride can have any nonzero number of working springs between it and the next stride
        // additionally, the first and last strides can have any nonnegative number of working springs before and after respectively
        // which means we want to generate all partitions subject to various constraints. this can be done recursively or iteratively
        let num_free_working_springs = self.total_length - self.contiguous_strides.iter().sum::<u32>() - (self.contiguous_strides.len()-1) as u32;
        println!("{num_free_working_springs}");
        vec![]
    }
}

fn main() {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let file = File::open("samples/day12.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    
    let mut all_patterns = Vec::new();
    for line in lines.map(|e| e.unwrap()) {
        if line.len() == 0 {
            break;
        }
        println!("{}", line);
        let mut split = line.split_whitespace();
        let test_pattern = split.next().unwrap().to_string();
        let contiguous_strides = split.next().unwrap().split(",").map(|e| e.parse::<u32>().unwrap()).collect::<Vec<_>>();
        let total = test_pattern.len();
        let spring_pattern = SpringPattern{
            contiguous_strides,
            total_length: total as u32
        };
        spring_pattern.generate_all_valid_patterns(test_pattern.as_str());
        all_patterns.push((spring_pattern, test_pattern));
    }
}