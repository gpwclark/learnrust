mod accounting {
    use std::fs::File;
    use std::io::BufRead;
    use std::path::Path;
    use regex::Regex;

    pub fn get_password_list() -> Vec<Password> {
        get_resource_generic("day2-resource.txt", |line| Password::new(&line))
    }

    fn make_password(line: &str) -> Option<Password> {
        let mut p: Option<Password> = None;
        if let Ok(reg) = Regex::new(r"([0-9]+)-([0-9]+) ([a-z]+): ([a-z]+)") {
            if let Some(caps) = reg.captures(&line) {
                let text1 = caps.get(1).map_or("0", |m| m.as_str());
                let text2 = caps.get(2).map_or("0", |m| m.as_str());
                let text3 = caps.get(3).map_or("", |m| m.as_str());
                let text4 = caps.get(4).map_or("", |m| m.as_str());
                if let Ok(min) = text1.parse() {
                    if let Ok(max) = text2.parse() {
                        p = Some(Password {
                            min,
                            max,
                            req: String::from(text3),
                            passwd: String::from(text4),
                        });
                    }
                }
            }
        }
        p
    }

    #[derive(Debug)]
    pub struct Password {
        min: u32,
        max: u32,
        req: String,
        passwd: String,
    }

    impl Password {
        pub fn new(line: &str) -> Option<Password> {
            make_password(&line)
        }

        pub fn is_valid_new(&self) -> bool {
            let req = &self.req;
            let assigned_indices: String = self.passwd.chars().enumerate().filter(|char_enum| {
                let (sze, _) = char_enum;
                *sze == (self.min - 1) as usize || *sze == (self.max - 1) as usize
            }).map(|char_enum| {
                let (_, chr) = char_enum;
                chr
            }).collect();
            let count = assigned_indices.matches(req).count() as u32;
            count == 1
        }

        pub fn is_valid(&self) -> bool {
            let req = &self.req;
            let cnt = self.passwd.matches(req).count() as u32;
            cnt >= self.min && cnt <= self.max
        }
    }

    fn get_resource_generic<T>(res: &str, eval: fn(String) -> Option<T>) -> Vec<T> {
        let mut v: Vec<T> = vec![];
        if let Ok(lines) = read_lines(res) {
            for line in lines {
                if let Ok(l) = line {
                    if !l.is_empty() {
                        if let Ok(item) = l.parse() {
                            if let Some(t) = eval(item) {
                                v.push(t)
                            }
                        }
                    }
                }
            }
        }
        v
    }

    pub fn get_expense_report() -> Vec<u32> {
        get_resource_generic("day1-resource.txt", |line| {
            match line.parse() {
                Ok(num) => Some(num),
                _ => None,
            }
        })
    }

    // The output is wrapped in a Result to allow matching on errors
    // Returns an Iterator to the Reader of the lines of the file.
    fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<File>>>
        where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(std::io::BufReader::new(file).lines())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        #[should_panic(expected = "fail, got None!")]
        fn test_bad_password_line_parse() {
            let mystr = "1-9 xxwjgxtmrzxzmkx";
            match make_password(&mystr) {
                None => panic!("fail, got None!"),
                _=> panic!("uh oh."),
            }
        }

        #[test]
        fn test_password_line_parse() {
            let mystr = "1-9 x: xwjgxtmrzxzmkx";
            match make_password(&mystr) {
                None => panic!("fail"),
                Some(p) => {
                    assert_eq!(1, p.min);
                    assert_eq!(9, p.max);
                    assert_eq!("x", p.req);
                    assert_eq!("xwjgxtmrzxzmkx", p.passwd);
                    println!("p {:?}, is valid new: {}.", p, p.is_valid_new())
                }
            }
        }

        #[test]
        fn pass_closure() {

        }
    }
}

pub mod advent1 {
    use advent_of_code::accounting;

    pub fn solve_puzzle_1(num_to_sum_to: u32) {
        let v = accounting::get_expense_report();
        let mut solution;
        for i in v.iter() {
            for j in v.iter() {
                if num_to_sum_to == i + j {
                    solution = i * j;
                    println!("we did it {} + {}. {}.", i, j, solution);
                }
            }
        }
    }

    pub fn solve_puzzle_2(num_to_sum_to: u32) {
        let v = accounting::get_expense_report();
        for i in v.iter() {
            for j in v.iter() {
                for k in v.iter() {
                    if num_to_sum_to == i + j + k {
                        println!("Found solution. {} {} {}, and: {}.", i, j, k, i * j *k);
                    }
                }
            }
        }
    }
}

pub mod advent2 {
    use advent_of_code::accounting;

    pub fn solve_puzzle() {
        let v = accounting::get_password_list();
        let valid_passwords = v.iter().filter(|p| p.is_valid()).count();
        println!("num valid: {}.", valid_passwords);
        let new_valid_passwords = v.iter().filter(|p| p.is_valid_new()).count();
        println!("num valid_new : {}.", new_valid_passwords)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_aoc1() {
        let num_to_sum_to = 2020;
        advent1::solve_puzzle_1(num_to_sum_to);
        advent1::solve_puzzle_2(num_to_sum_to);
    }

    #[test]
    fn solve_aoc2() {
        advent2::solve_puzzle();
    }

    #[test]
    fn test_usze() {
        let one: usize = 1;
        println!("lksadf: {}.", one == one)
    }

    #[test]
    fn test_matching() {
        println!("count: {}.", "alkjalslkja".matches("a").count());
        println!("meow: {:?}.", "alkjalslkja".chars().enumerate().filter(|enumer| {
            let (s, _) = enumer;
            *s == 1 || *s == 3
        }).map(|enumer| {
            let (_, c) = enumer;
            c
        }).collect::<String>());
    }
}

