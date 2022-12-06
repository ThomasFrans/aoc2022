use std::{
    env,
    fmt::{Debug, Display},
};

use crate::message::ElfMessage;

struct ProgramArguments {
    filename: String,
}

impl ProgramArguments {
    fn from_env() -> Result<ProgramArguments, String> {
        let args = env::args();
        // Skip the program name.
        let mut args = args.skip(1);

        Ok(Self {
            filename: args.next().ok_or("Couldn't parse arguments.")?,
        })
    }
}

mod message {
    use itertools::Itertools;

    const HEADER_SIZE: usize = 4;
    const START_OF_MESSAGE_HEADER_SIZE: usize = 14;

    pub struct ElfMessage {
        pub data: String,
    }

    impl ElfMessage {
        pub fn header(&self) -> Result<String, &'static str> {
            let data = self.data.chars().collect::<Vec<_>>();
            for chunk in 0..data.len() - HEADER_SIZE {
                let chunked = &data[chunk..chunk + HEADER_SIZE];
                if chunked.iter().all_unique() {
                    return Ok(data[0..chunk + HEADER_SIZE].iter().collect::<String>());
                }
            }
            Err("Couldn't find header.")
        }

        pub fn start_of_message_header(&self) -> Result<String, &'static str> {
            // Copying is ok because iterating over &char would probably require the
            // same amount of information to be created per char, since it has to
            // know the length of a char.
            let data = self.data.chars().collect::<Vec<_>>();
            for chunk in 0..data.len() - START_OF_MESSAGE_HEADER_SIZE {
                let chunked = &data[chunk..chunk + START_OF_MESSAGE_HEADER_SIZE];
                if chunked.iter().all_unique() {
                    return Ok(data[0..chunk + START_OF_MESSAGE_HEADER_SIZE]
                        .iter()
                        .collect::<String>());
                }
            }
            Err("Couldn't find start of message header.")
        }
    }
}

struct Error {
    error: Box<dyn Display>,
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl<T: Display + 'static> From<T> for Error {
    fn from(error: T) -> Self {
        Self {
            error: Box::new(error),
        }
    }
}

fn main() -> Result<(), Error> {
    let arguments = ProgramArguments::from_env()?;

    let input = std::fs::read_to_string(&arguments.filename)?;

    let message = ElfMessage { data: input };

    // Part 1
    println!("Length of header is {}.", message.header()?.len());

    // Part2
    println!(
        "Length of start of message header is {}.",
        message.start_of_message_header()?.len()
    );
    Ok(())
}
