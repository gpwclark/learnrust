mod accounting {
    pub(crate) const num_to_sum_to: u32 = 2020;

    use std::fs::File;
    use std::io::BufRead;
    use std::path::Path;

    pub fn get_expense_report() -> Vec<u32> {
        let mut v = vec![];
        if let Ok(lines) = read_lines("day1-resource.txt") {
            for line in lines {
                if let Ok(l) = line {
                    if !l.is_empty() {
                        match l.parse::<u32>() {
                            Ok(num) => v.push(num),
                            _ => (),
                        }
                    }
                }
            }
        }
        v
    }

    // The output is wrapped in a Result to allow matching on errors
    // Returns an Iterator to the Reader of the lines of the file.
    fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<File>>>
        where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(std::io::BufReader::new(file).lines())
    }
}

pub mod advent1 {
    use advent_of_code::accounting;

    pub fn solve() {
        let v = accounting::get_expense_report();
        let mut solution;
        for i in v.iter() {
            for j in v.iter() {
                if accounting::num_to_sum_to == i + j {
                    solution = i * j;
                    println!("we did it {} + {}. {}.", i, j, solution);
                }
            }
        }
    }
}

pub mod advent2 {
    use advent_of_code::accounting;

    pub fn solve() {
        let v = accounting::get_expense_report();
        for i in v.iter() {
            for j in v.iter() {
                for k in v.iter() {
                    if accounting::num_to_sum_to == i + j + k {
                        println!("Found solution. {} {} {}, and: {}.", i, j, k, i * j *k);
                    }
                }
            }
        }
    }
}
