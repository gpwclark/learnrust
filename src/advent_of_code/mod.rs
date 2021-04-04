mod accounting {
    use std::fs::File;
    use std::io::BufRead;
    use std::path::Path;
    use regex::Regex;

    pub fn get_password_list() -> Vec<Password> {
        get_password_resource("day2-resource.txt")
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
    }

    fn get_password_resource(res: &str) -> Vec<Password> {
        let mut v = vec![];
        if let Ok(lines) = read_lines(res) {
            for line in lines {
                if let Ok(l) = line {
                    if !l.is_empty() {
                        match l.parse::<String>() {
                            Ok(pwd_string) => {
                                if let Some(p) = Password::new(&pwd_string) {
                                    v.push(p)
                                }
                            },
                            _ => (),
                        }
                    }
                }
            }
        }
        v
    }

    fn get_resource(res: &str) -> Vec<u32> {
        let mut v = vec![];
        if let Ok(lines) = read_lines(res) {
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

    pub fn get_expense_report() -> Vec<u32> {
        get_resource("day1-resource.txt")
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
            let p = make_password(mystr);
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
                }
            }
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

    pub fn solve_puzzle_1() {
        let v = accounting::get_password_list();
        for i in v.iter() {
            println!("i: {:?}.", i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_aoc1() {
        //let num_to_sum_to = 2020;
        //advent1::solve_puzzle_1(num_to_sum_to);
        //advent1::solve_puzzle_2(num_to_sum_to);
        advent2::solve_puzzle_1();
    }
}

