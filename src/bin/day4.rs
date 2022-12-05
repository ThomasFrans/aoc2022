use std::{fs::File, io::{BufReader, BufRead}, fmt::Display};

use itertools::Itertools;

#[derive(Debug)]
struct Group {
    first: (u32, u32),
    second: (u32, u32),
}

impl Group {
    pub fn contains_total_overlap(&self) -> bool {
        (self.first.0 >= self.second.0 && self.first.1 <= self.second.1) ||
            (self.second.0 >= self.first.0 && self.second.1 <= self.first.1)
    }

    pub fn contains_overlap(&self) -> bool {
        (self.first.0 >= self.second.0 && self.first.0 <= self.second.1) ||
            (self.first.1 >= self.second.0 && self.first.1 <= self.second.1) ||
            self.contains_total_overlap()
    }
}

impl Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Elf 1: {}-{}\nElf 2: {}-{}", self.first.0, self.first.1, self.second.0, self.second.1)
    }
}

fn parse_line(line: &str) -> Group {
    let number_groups = line.chars().group_by(|char| {
        char.is_digit(10)
    });
    let numbers = number_groups.into_iter()
        .filter(|group| {
            group.0
        })
        .map(|number| {
            number.1.collect::<String>().parse::<u32>().unwrap()
        });
    let numbers = numbers.collect::<Vec<u32>>();
    Group { first: (numbers[0], numbers[1]), second: (numbers[2], numbers[3])}
}

fn main() -> Result<(), ()> {
    let file = File::open("data/day4.txt").unwrap();
    let mut total_overlaps = 0;
    let mut total_partial_overlaps = 0;

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let group = parse_line(&line);
        if group.contains_total_overlap() {
            total_overlaps += 1;
        }
        if group.contains_overlap() {
            total_partial_overlaps += 1;
        }
    }

    println!("Total overlaps: {total_overlaps}.");
    println!("Total partial overlaps: {total_partial_overlaps}.");

    Ok(())
}
