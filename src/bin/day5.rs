use std::{fs::File, process::ExitCode, io::{BufReader, BufRead}, fmt::Display};
use itertools::Itertools;


#[derive(Debug, Clone)]
struct Crate(char);
#[derive(Debug, Clone)]
struct Stack(Vec<Crate>);
#[derive(Debug, Clone)]
struct Crates(Vec<Stack>);
#[derive(Debug)]
struct Move {
    pub amount: usize,
    pub from: usize,
    pub to: usize,
}
#[derive(Debug)]
struct Moves(Vec<Move>);

impl Display for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<Vec<String>> for Crates {
    type Error = ();

    fn try_from(data: Vec<String>) -> Result<Self, Self::Error> {
        // Vec<String> (lines) to Iterator<Item=String> (over lines).
        let cargo = data[..data.len()-1].into_iter()
            .map(|line| {
                // String (line) to Iterator over chars (characters).
                let chunked_line = line.chars()
                    // Chars grouped by crates in the line.
                    .chunks(4);
                // Iterator over chunks (crates) to Iterator<Item=Chunk>.
                let crate_vec = chunked_line.into_iter()
                    // Iterator<Item=Chunk> to Iterator<Item=Option<Crate>>.
                    .map(|chunk| {
                        chunk.enumerate()
                            .filter(|(index, _)| *index == 1)
                            .map(|(_, value)| {
                                if value.is_alphabetic() {
                                    Some(Crate(value))
                                } else {
                                    None
                                }
                            })
                            .next().unwrap()
                    })
                    .collect::<Vec<Option<Crate>>>();
                    crate_vec
            })
            .collect_vec();
        let mut crates: Vec<Stack> = Vec::new();
        for stack_item in 0..cargo[0].len() {
            let mut stack: Vec<Crate> = Vec::new();
            for crate_item in 0..cargo.len() {
                if let Some(item) = &cargo[cargo.len()-1-crate_item][stack_item] {
                    stack.push(item.clone());
                }
            }
            crates.push(Stack(stack));
        }
        Ok(Crates(crates))
    }
}

impl TryFrom<Vec<String>> for Moves {
    type Error = ();

    fn try_from(moves_data: Vec<String>) -> Result<Self, Self::Error> {
        let a = moves_data.into_iter()
            // Map lines to Move's.
            .map(|line| {
                let groups = line.chars()
                    .group_by(|char| char.is_numeric());
                let values = groups.into_iter()
                    // Iterator over Iterators over numeric chars.
                    .filter_map(|(key, value)| {
                        if key {
                            let string_value = value.collect::<String>();
                            Some(string_value.parse::<usize>().unwrap())
                        } else {
                            None
                        }
                    })
                    .collect_vec();
                Move {
                    amount: values[0],
                    from: values[1],
                    to: values[2],
                }
            })
            .collect_vec();
        Ok(Self(a))
    }
}

fn main() -> ExitCode {
    let file = File::open("data/day5.txt").unwrap();
    let lines = BufReader::new(file).lines();

    // Split at the double newline.
    let groups = lines
        .map(|line| line.unwrap())
        // Iterator over Iterator<Item=String>
        .group_by(|line| line.is_empty());   // First group is crates, second is operations.

    // data[0] are the crates, data[1] the moves.
    let data = groups
        .into_iter()
        // Only keep the groups that contained data.
        .filter(|(empty, _)| !empty)
        .map(|(_, value)| value.collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    let crates = Crates::try_from(data[0].clone()).unwrap();
    let moves = Moves::try_from(data[1].clone()).unwrap();

    // Part 1
    let mut crates1 = crates.clone();
    moves.0.iter()
        .for_each(|single| {
            for _ in 0..single.amount {
                let popped_crate = crates1.0[single.from-1].0.pop().unwrap();
                crates1.0[single.to-1].0.push(popped_crate);
            }
        });
    crates1.0.iter()
        .for_each(|stack| {
            print!("{}", stack.0.last().unwrap());
        });
    println!(" with CrateMover9000.");

    // Part 2
    let mut crates2 = crates.clone();
    moves.0.iter()
        .for_each(|single| {
            let mut reverse = Vec::new();
            for _ in 0..single.amount {
                let popped_crate = crates2.0[single.from-1].0.pop().unwrap();
                reverse.push(popped_crate);
            }
            for _ in 0..single.amount {
                crates2.0[single.to-1].0.push(reverse.pop().unwrap());
            }
        });
    crates2.0.iter()
        .for_each(|stack| {
            print!("{}", stack.0.last().unwrap());
        });
    println!(" with CrateMover9001.");

    ExitCode::SUCCESS
}
