use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::Itertools;


fn main() {
    //first();
    second();
    
}

fn first() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut sum_prio = 0;
        for line in lines {
            if let Ok(str) = line {
                let (first, second) = str.split_at(str.chars().count()/2);
                let both = share_char(first, second);
                println!("{} | {} ==> {}", first, second, both);
                sum_prio += str_priority(both.as_str());
            }
        }
        println!("Sum: {}", sum_prio);
    }
}

fn second() {
    if let Ok(mut lines) = read_lines("./input.txt") {
        let mut sum_prio = 0;
        while let (Some(line1), Some(line2), Some(line3)) = (lines.next(), lines.next(), lines.next()) {
            // do things with line1 and line2
            let shared = share_char_three_strings(line1.unwrap().as_str(), line2.unwrap().as_str(), line3.unwrap().as_str());
            println!("{}", shared);
            sum_prio += str_priority(shared.as_str());
        }
        println!("Sum: {}", sum_prio);
    }
}

fn str_priority(str: &str) -> u32 {
    let mut prio = 0;
    for c in str.chars() { 
        prio += char_priority(&c);
    }
    return prio;
}

fn char_priority(c: &char) -> u32 {
    if c.is_uppercase() {
        return *c as u32 - 38; 
    } else {
        return *c as u32 - 96;
    }
}

fn share_char(a: &str, b: &str) -> String {
    return a.chars().filter(|c| b.contains(*c)).unique().collect::<String>();
}

fn share_char_three_strings(a: &str, b: &str, c: &str) -> String {
    return a.chars().filter(|ch| b.contains(*ch) && c.contains(*ch)).unique().collect::<String>();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}