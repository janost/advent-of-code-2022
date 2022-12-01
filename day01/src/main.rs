use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut elves: Vec<u32> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        let mut elfsum : u32 = 0;
        for line in lines {
            if let Ok(str) = line {
                if str.is_empty() {
                    elves.push(elfsum);
                    elfsum = 0;
                } else {
                    elfsum += str.parse::<u32>().unwrap();
                }
            }
        }
    }

    elves.sort();
    elves.reverse();
    println!("{:?}", elves[0..3].to_vec());
    println!("{}", elves[0..3].iter().sum::<u32>());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}