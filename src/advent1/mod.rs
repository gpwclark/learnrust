use std::fs::File;
use std::io::BufRead;
use std::path::Path;

pub fn print_nums() {
    let mut v = Vec::new();
    if let Ok(lines) = read_lines("day1-resource.txt") {
        for line in lines {
            if let Ok(l) = line {
                if !l.is_empty() {
                    //println!("line: {}.", l);
                    match l.parse::<u32>() {
                        Ok(num) => v.push(num),
                        _ => (),
                    }
                }
            }
        }
    }
    for i in v.iter() {
        println!("My numbers!: {}.", i)
    }
}
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(std::io::BufReader::new(file).lines())
}
