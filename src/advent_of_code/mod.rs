mod accounting {
    use std::fs::File;
    use std::io::BufRead;
    use std::path::Path;
    use regex::Regex;

    #[derive(Debug)]
    pub struct Puzzle<T, U> where U: Fn(String) -> Option<T> {
        xform: U,
        day: u32,
    }

    impl<T, U> Puzzle<T, U> where U: Fn(String) -> Option<T> {
        pub fn new(day: u32, xform: U) -> Puzzle<T, U> {
            Puzzle {
                xform,
                day,
            }
        }

        pub fn get_puzzle_data(&self) -> Option<Vec<T>> {
            let filename = format!("day{}-resource.txt", self.day);
            let mut ret = None;
            if let Ok(lines) = read_lines(filename) {
                let mut v: Vec<T> = vec![];
                for line in lines {
                    if let Ok(l) = line {
                        if !l.is_empty() {
                            if let Ok(item) = l.parse() {
                                if let Some(t) = (self.xform)(item) {
                                    v.push(t)
                                }
                            }
                        }
                    }
                }
                ret = Some(v);
            }
            ret
        }
    }

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
    use self::accounting::Puzzle;

    pub fn solve_puzzle_1(num_to_sum_to: u32) -> Vec<(u32, u32)>{
        let puzzle = Puzzle::new(1, |line: String| -> Option<u32> {
            match line.parse() {
                Ok(num) => Some(num),
                _ => None,
            }
        });
        let mut solution = vec![];
        if let Some(v) = puzzle.get_puzzle_data() {
            for i in v.iter() {
                for j in v.iter() {
                    if num_to_sum_to == i + j {
                        let tup: (u32, u32) = (*i, *j);
                        solution.push(tup);
                    }
                }
            }
        }
        solution
    }

    pub fn solve_puzzle_2(num_to_sum_to: u32) -> Vec<(u32, u32, u32)> {
        let puzzle = Puzzle::new(1, |line: String| -> Option<u32> {
            match line.parse() {
                Ok(num) => Some(num),
                _ => None,
            }
        });

        let mut solution = vec![];
        if let Some(v) = puzzle.get_puzzle_data() {
            for i in v.iter() {
                for j in v.iter() {
                    for k in v.iter() {
                        if num_to_sum_to == i + j + k {
                            let tup: (u32, u32, u32) = (*i, *j, *k);
                            solution.push(tup);
                        }
                    }
                }
            }
        }
        solution
    }
}

pub mod advent2 {
    use advent_of_code::accounting;

    pub fn solve_puzzle_1() -> usize {
        let v = accounting::get_password_list();
        let valid_passwords = v.iter().filter(|p| p.is_valid()).count();
        valid_passwords
    }

    pub fn solve_puzzle_2() -> usize {
        let v = accounting::get_password_list();
        let new_valid_passwords = v.iter().filter(|p| p.is_valid_new()).count();
        new_valid_passwords
    }
}

pub mod advent3 {
    use self::accounting::Puzzle;
    use advent_of_code::accounting;

    #[derive(Debug)]
    pub struct CircArr {
        arr: Vec<u8>,
        idx: usize,
        len: usize,
    }

    impl CircArr {
        pub fn new(arr: Vec<u8>) -> CircArr {
            let idx = 0;
            let len = arr.len();
            CircArr {
                arr,
                idx,
                len,
            }
        }

        pub fn get(&self, idx: usize) -> u8 {
            let i: usize;
            if idx >= self.len {
                i = idx % self.len;
            }
            else {
                i = idx;
            }
            self.arr[i]
        }
    }

    impl Iterator for CircArr {
        type Item = u8;

        fn next(&mut self) -> Option<u8> {
            let i: usize;
            if self.idx >= self.len {
                i = &self.idx % &self.len;
            }
            else {
                i = self.idx;
            }
            self.idx = &self.idx + 1;
            let item: Option<&u8> = self.arr.get(i);
            match item {
                Some(val) => Some(*val),
                None => None
            }
        }
    }


    pub fn solve_puzzle_1() -> u32 {
        let puzzle = Puzzle::new(3, |line: String| -> Option<CircArr> {
            let bit_map: Vec<u8> = line.chars().map(|c| {
                if c == '.' {
                    0
                }
                else {
                    1
                }
            }).collect();
            println!("bit map {:?}.", bit_map);
            Some(CircArr::new(bit_map))
        });

        let mut tree_hit = 0;
        let mut start: usize = 0;
        if let Some(puzzle) = puzzle.get_puzzle_data() {
            for i in puzzle.iter() {
                let land_at: u8 = i.get(start);
                start += 3;
                if let 1 = land_at {
                    tree_hit = tree_hit + 1;
                }
            }
            println!("num trees hit: {}.", tree_hit);
        }
        tree_hit
    }

    pub fn solve_puzzle_2() {
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::advent3::CircArr;

    #[test]
    fn solve_aoc1() {
        let num_to_sum_to = 2020;
        let sol1: Vec<(u32, u32)> = advent1::solve_puzzle_1(num_to_sum_to);
        assert!(sol1.len() > 0);
        for s in sol1.iter() {
            assert_eq!(877971, s.0 * s.1);
        }

        let sol2 = advent1::solve_puzzle_2(num_to_sum_to);
        assert!(sol2.len() > 0);
        for s in sol2.iter() {
            assert_eq!(203481432, s.0 * s.1 * s.2);
        }
    }

    #[test]
    fn solve_aoc2() {
        let sol1 = advent2::solve_puzzle_1();
        assert_eq!(sol1, 640);
        let sol2 = advent2::solve_puzzle_2();
        assert_eq!(sol2, 472);
    }

    #[test]
    fn solve_aoc3() {
        let tree_hit = advent3::solve_puzzle_1();
        assert_eq!(244, tree_hit);
        advent3::solve_puzzle_2();
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

    #[test]
    fn test_sqrt() {
        let takeme: f64 = 27.0;
        println!("takene: {}.", takeme.log(3.0));
    }

    #[test]
    fn test_infini_arr() {
        let my_vec: Vec<u8> = vec![0, 0, 0, 0, 1];
        let my_arr = CircArr::new(my_vec);
        println!("Oh my: {:?}.", my_arr.get(18));
        println!("Oh my: {:?}.", my_arr.get(4));
    }
}

