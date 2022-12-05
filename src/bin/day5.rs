use itertools::Itertools;
use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Deref, DerefMut},
    process::{exit, ExitCode},
};

fn exit_with_message(message: &str, code: i32) -> ! {
    eprintln!("{message}");
    exit(code)
}

#[derive(Debug, Clone)]
struct Crate(char);

impl Deref for Crate {
    type Target = char;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Crate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone)]
struct Stack(Vec<Crate>);

impl Deref for Stack {
    type Target = Vec<Crate>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Stack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone)]
struct Cargo(Vec<Stack>);

impl Deref for Cargo {
    type Target = Vec<Stack>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Cargo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
struct Operation {
    pub amount: usize,
    pub from: usize,
    pub to: usize,
}

#[derive(Debug)]
struct Operations(Vec<Operation>);

impl Deref for Operations {
    type Target = Vec<Operation>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Operations {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<Vec<String>> for Cargo {
    type Error = ();

    fn try_from(data: Vec<String>) -> Result<Self, Self::Error> {
        // Vec<String> (lines) to Iterator<Item=String> (over lines).
        // Omit the last line which isn't data.
        let cargo = data[..data.len() - 1]
            .iter()
            .map(|line| {
                // String (line) to Iterator over chars (characters).
                let chunked_line = line
                    .chars()
                    // Chars grouped by crates in the line.
                    .chunks(4);
                // Iterator over chunks (crates) to Iterator<Item=Chunk>.

                chunked_line
                    .into_iter()
                    // Iterator<Item=Chunk> to Iterator<Item=Option<Crate>>.
                    .map(|chunk| {
                        chunk
                            .enumerate()
                            .filter(|(index, _)| *index == 1)
                            .map(|(_, value)| {
                                if value.is_alphabetic() {
                                    Some(Crate(value))
                                } else {
                                    None
                                }
                            })
                            .next()
                            .ok_or(())
                    })
                    .collect::<Result<Vec<Option<Crate>>, ()>>()
            })
            .collect::<Result<Vec<_>, ()>>()?;
        let mut crates: Vec<Stack> = Vec::new();
        for stack_item in 0..cargo[0].len() {
            let mut stack: Vec<Crate> = Vec::new();
            for crate_item in 0..cargo.len() {
                if let Some(item) = &cargo[cargo.len() - 1 - crate_item][stack_item] {
                    stack.push(item.clone());
                }
            }
            crates.push(Stack(stack));
        }
        Ok(Cargo(crates))
    }
}

impl TryFrom<Vec<String>> for Operations {
    type Error = ();

    fn try_from(operations_data: Vec<String>) -> Result<Self, Self::Error> {
        let a = operations_data
            .into_iter()
            // Map lines to Move's.
            .map(|line| {
                let groups = line.chars().group_by(|char| char.is_numeric());
                let values = groups
                    .into_iter()
                    // Iterator over Iterators over numeric chars.
                    .filter_map(|(key, value)| {
                        if key {
                            let string_value = value.collect::<String>();
                            if let Ok(parsed_value) = string_value.parse::<usize>() {
                                Some(Ok(parsed_value))
                            } else {
                                Some(Err(()))
                            }
                        } else {
                            None
                        }
                    })
                    .collect::<Result<Vec<usize>, ()>>();
                if let Ok(values) = values {
                    if let [amount, from, to] = values[..] {
                        Ok(Operation { amount, from, to })
                    } else {
                        Err(())
                    }
                } else {
                    Err(())
                }
            })
            .collect::<Result<Vec<Operation>, ()>>()?;
        Ok(Self(a))
    }
}

struct CrateMover9000<'a> {
    crates: &'a mut Cargo,
    operations: &'a Operations,
}

impl<'a> CrateMover9000<'a> {
    fn new(crates: &'a mut Cargo, operations: &'a Operations) -> Self {
        Self { crates, operations }
    }

    fn execute(&mut self) {
        self.operations.iter().for_each(|single| {
            self.move_crates(single);
        });
    }

    fn move_crates(&mut self, operation: &Operation) {
        for _ in 0..operation.amount {
            let popped_crate = self.crates[operation.from - 1]
                .pop()
                .expect("Ran out of crates to remove.");
            self.crates[operation.to - 1].push(popped_crate);
        }
    }
}

struct CrateMover9001<'a> {
    crates: &'a mut Cargo,
    operations: &'a Operations,
}

impl<'a> CrateMover9001<'a> {
    fn new(crates: &'a mut Cargo, operations: &'a Operations) -> Self {
        Self { crates, operations }
    }

    fn execute(&mut self) {
        self.operations.iter().for_each(|single| {
            self.move_crates(single);
        });
    }

    fn move_crates(&mut self, operation: &Operation) {
        let mut reverse = Vec::new();
        for _ in 0..operation.amount {
            let popped_crate = self.crates[operation.from - 1]
                .pop()
                .expect("Ran out of crates to remove.");
            reverse.push(popped_crate);
        }
        for _ in 0..operation.amount {
            self.crates[operation.to - 1].push(reverse.pop().unwrap());
        }
    }
}

fn main() -> ExitCode {
    let filename = "data/day5.txt";
    let Ok(file) = File::open(filename) else {
        exit_with_message(&format!("Can't open file {}.", filename), 1)
    };
    let lines = BufReader::new(file).lines();

    // Split at the double newline.
    // Conceptually: (crate_data, movement_data)
    let groups = lines
        .map(|line| line.expect("Invalid line in file."))
        // Iterator over Iterator<Item=String>
        .group_by(|line| line.is_empty()); // First group is crates, second is operations.
                                           // data[0] are the crates, data[1] the moves.
    let data = groups
        .into_iter()
        // Only keep the groups that contained data.
        .filter(|(empty, _)| !empty)
        .map(|(_, value)| value.collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    // Parsing.
    let Ok(crates) = Cargo::try_from(data[0].clone()) else {
        eprintln!("Cargo data isn't in the correct format.");
        return ExitCode::FAILURE;
    };
    let Ok(operations) = Operations::try_from(data[1].clone()) else {
        eprintln!("Operation data isn't in the correct format.");
        return ExitCode::FAILURE;
    };

    // Part 1
    let mut crates1 = crates.clone();
    CrateMover9000::new(&mut crates1, &operations).execute();
    crates1.iter().for_each(|stack| {
        print!("{}", stack.last().unwrap());
    });
    println!(" with CrateMover9000.");

    // Part 2
    let mut crates2 = crates;
    CrateMover9001::new(&mut crates2, &operations).execute();
    crates2.iter().for_each(|stack| {
        print!("{}", stack.last().unwrap());
    });
    println!(" with CrateMover9001.");

    ExitCode::SUCCESS
}
