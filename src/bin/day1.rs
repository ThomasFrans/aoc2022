use std::{
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
};

use itertools::Itertools;

fn main() -> Result<(), String> {
    let file_location = "data/day1.txt";

    let Ok(file) = File::open(file_location) else {
        return Err(format!(
            "Failed to open file {}, maybe it doesn't exist.",
            file_location
        ));
    };

    let file_reader = BufReader::new(file);

    let groups = file_reader
        .lines()
        .map(|line| {
            let Ok(line) = line else {
                eprintln!("Line {line:?} isn't a valid string.");
                exit(1);
            };
            line
        })
        .group_by(|line| line.is_empty());

    let elves = groups
        .into_iter()
        .enumerate()
        .filter(|(index, _)| *index % 2 == 0)
        .map(|(_, other)| other);

    let calories = elves
        .map(|(_, group)| {
            let mut total = 0;
            for item in group {
                total += item.parse::<u32>().unwrap();
            }
            total
        })
        .sorted()
        .rev()
        .collect::<Vec<_>>();

    let total = calories[0] + calories[1] + calories[2];

    println!("total top 3 cals: {}", total);

    Ok(())
}
