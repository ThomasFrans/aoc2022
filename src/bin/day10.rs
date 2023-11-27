use std::fs;

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

#[derive(Debug)]
struct Program {
    instructions: Vec<Instruction>,
}

impl TryFrom<&str> for Program {
    type Error = &'static str;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut result = Program {
            instructions: Vec::with_capacity(input.lines().count()),
        };
        for line in input.lines() {
            let mut parts = line.split(' ');
            match parts.next().unwrap() {
                "noop" => result.instructions.push(Instruction::Noop),
                "addx" => result
                    .instructions
                    .push(Instruction::Addx(parts.next().unwrap().parse().unwrap())),
                _ => panic!(),
            }
        }
        Ok(result)
    }
}

struct Cpu<'a> {
    instructions: &'a [Instruction],
    current_instruction: usize,
    instruction_cycle: u32,
    register: i32,
}

impl<'a> Cpu<'_> {
    fn run_cycle(&mut self) -> Result<(), ()> {
        match self.instructions[self.current_instruction] {
            Instruction::Noop => {
                if self.current_instruction + 1 < self.instructions.len() {
                    self.current_instruction += 1;
                    Ok(())
                } else {
                    Err(())
                }
            }
            Instruction::Addx(amount) => {
                if self.instruction_cycle == 0 {
                    self.instruction_cycle += 1;
                    Ok(())
                } else if self.current_instruction + 1 < self.instructions.len() {
                    self.current_instruction += 1;
                    self.instruction_cycle = 0;
                    self.register += amount;
                    Ok(())
                } else {
                    Err(())
                }
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("data/day10.txt").unwrap();

    let program = Program::try_from(input.as_str()).unwrap();

    let mut cpu = Cpu {
        instructions: &program.instructions,
        current_instruction: 0,
        instruction_cycle: 0,
        register: 1,
    };

    let mut cycle = 1;
    let mut result = 0;
    while cpu.run_cycle().is_ok() {
        cycle += 1;
        println!("amount: {cycle}");
        if (cycle + 20) % 40 == 0 {
            println!("added result");
            result += cycle * cpu.register;
        }
    }

    println!("signal: {result}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

        let program = Program::try_from(input).unwrap();

        let mut cpu = Cpu {
            instructions: &program.instructions,
            current_instruction: 0,
            instruction_cycle: 0,
            register: 1,
        };

        let mut cycle = 1;
        let mut result = 0;
        while cpu.run_cycle().is_ok() {
            cycle += 1;
            println!("amount: {cycle}");
            if (cycle + 20) % 40 == 0 {
                println!("added result");
                result += cycle * cpu.register;
            }
        }

        println!("signal: {result}");
        assert_eq!(result, 13140);
    }
}
