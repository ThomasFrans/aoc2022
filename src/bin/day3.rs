use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

trait RuckSack {
    fn first_compartment(&self) -> &str;

    fn second_compartment(&self) -> &str;

    fn shared_item_accross_compartments(&self) -> char;
}

fn item_to_priority(item: char) -> Result<u32, ()> {
    if item.is_ascii_lowercase() {
        Ok((item as u8 - b'a' + 1) as u32)
    } else if item.is_ascii_uppercase() {
        Ok((item as u8 - b'A' + 27) as u32)
    } else {
        Err(())
    }
}

fn shared_character(elves: Vec<String>) -> Result<char, ()> {
    if elves.len() < 2 {
        Err(())
    } else {
        let first_elf = &elves[0];
        let mut found = None;
        first_elf.chars().for_each(|char| {
            if elves[1..].iter().all(|string| string.contains(char)) {
                found = Some(char)
            }
        });
        if let Some(char) = found {
            Ok(char)
        } else {
            Err(())
        }
    }
}

impl RuckSack for String {
    fn first_compartment(&self) -> &str {
        self.split_at(self.len() / 2).0
    }

    fn second_compartment(&self) -> &str {
        self.split_at(self.len() / 2).1
    }

    fn shared_item_accross_compartments(&self) -> char {
        println!(
            "First: {}, Second: {}",
            self.first_compartment(),
            self.second_compartment()
        );
        for item in self.first_compartment().chars() {
            if self.second_compartment().contains(item) {
                return item;
            }
        }
        panic!();
    }
}

fn main() {
    let input = File::open("data/day3.txt").unwrap();

    let reader = BufReader::new(input);
    let mut total_priority = 0;

    for line in reader.lines() {
        let shared = line.unwrap().shared_item_accross_compartments();
        let priority = item_to_priority(shared).unwrap();
        println!("Shared: {} with value {}.", shared, priority);
        total_priority += priority;
    }

    let input = File::open("data/day3.txt").unwrap();
    let reader = BufReader::new(input);

    let _groups: Vec<Vec<String>> = Vec::new();

    let mut total = 0;

    for group in &reader.lines().chunks(3) {
        total += item_to_priority(
            shared_character(group.map(|string| string.unwrap()).collect_vec()).unwrap(),
        )
        .unwrap();
    }

    println!("The total priority of all the items is {total_priority}.");
    println!("The total priority of all the items in the groups is {total}.");
}
