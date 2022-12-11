use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::Add,
};

#[derive(Debug)]
enum Move {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

fn parse_moves(input: &str) -> Vec<Move> {
    let mut result = Vec::new();
    input.lines().for_each(|line| {
        let mut parts = line.split(' ');
        match parts.next().unwrap() {
            "U" => result.push(Move::Up(parts.next().unwrap().parse::<usize>().unwrap())),
            "D" => result.push(Move::Down(parts.next().unwrap().parse::<usize>().unwrap())),
            "L" => result.push(Move::Left(parts.next().unwrap().parse::<usize>().unwrap())),
            "R" => result.push(Move::Right(parts.next().unwrap().parse::<usize>().unwrap())),
            _ => panic!(),
        }
    });
    result
}

struct RopeSimulation<'a> {
    visited_positions: HashSet<(i32, i32)>,
    moves: &'a [Move],
    head_coordinate: (i32, i32),
    tail_coordinate: (i32, i32),
}

impl<'a> From<&'a [Move]> for RopeSimulation<'a> {
    fn from(moves: &'a [Move]) -> Self {
        let mut result = Self {
            visited_positions: HashSet::new(),
            moves,
            head_coordinate: (0, 0),
            tail_coordinate: (0, 0),
        };

        for change in result.moves {
            match change {
                Move::Up(amount) => {
                    for i in 0..*amount {
                        if result.head_coordinate.1 > result.tail_coordinate.1 {
                            result.tail_coordinate.1 += 1;
                            result.tail_coordinate.0 = result.head_coordinate.0;
                        }
                        result.visited_positions.insert(result.tail_coordinate);
                        result.head_coordinate.1 += 1;
                    }
                }
                Move::Down(amount) => {
                    for i in 0..*amount {
                        if result.head_coordinate.1 < result.tail_coordinate.1 {
                            result.tail_coordinate.1 -= 1;
                            result.tail_coordinate.0 = result.head_coordinate.0;
                        }
                        result.visited_positions.insert(result.tail_coordinate);
                        result.head_coordinate.1 -= 1;
                    }
                }
                Move::Left(amount) => {
                    for i in 0..*amount {
                        if result.head_coordinate.0 < result.tail_coordinate.0 {
                            result.tail_coordinate.0 -= 1;
                            result.tail_coordinate.1 = result.head_coordinate.1;
                        }
                        result.visited_positions.insert(result.tail_coordinate);
                        result.head_coordinate.0 -= 1;
                    }
                }
                Move::Right(amount) => {
                    for i in 0..*amount {
                        if result.head_coordinate.0 > result.tail_coordinate.0 {
                            result.tail_coordinate.0 += 1;
                            result.tail_coordinate.1 = result.head_coordinate.1;
                        }
                        result.visited_positions.insert(result.tail_coordinate);
                        result.head_coordinate.0 += 1;
                    }
                }
            }
        }

        result
    }
}

struct Rope<const S: usize> {
    segments: [XY; S],
}

// (0, 0) (1, 0) (2, 0) (3, 0)
// (0, 1) (1, 1) (2, 1) (3, 1)
// (0, 2) (1, 2) (2, 2) (3, 2)
// (0, 3) (1, 3) (2, 3) (3, 3)
impl<const S: usize> Rope<S> {
    fn attach_to(&mut self, this: usize, other: usize) {
        // Scuffed but it works.
        let other = self.segments[other].clone();
        let this = &mut self.segments[this];
        if other.y - this.y == 2 && other.x - this.x == 2 {
            this.y += 1;
            this.x += 1;
        } else if this.y - other.y == 2 && other.x - this.x == 2 {
            this.y -= 1;
            this.x += 1;
        } else if other.y - this.y == 2 && this.x - other.x == 2 {
            this.y += 1;
            this.x -= 1;
        } else if this.y - other.y == 2 && this.x - other.x == 2 {
            this.y -= 1;
            this.x -= 1;
        } else if other.y - this.y == 2 {
            this.y += 1;
            this.x = other.x;
        // Other above this.
        } else if this.y - other.y == 2 {
            this.y -= 1;
            this.x = other.x;
        // Other right of this.
        } else if other.x - this.x == 2 {
            this.x += 1;
            this.y = other.y;
        // Other left of this.
        } else if this.x - other.x == 2 {
            this.x -= 1;
            this.y = other.y;
        }
    }

    fn apply_move(&mut self, movement: Direction) {
        let movement = match movement {
            Direction::Up => XY { x: 0, y: -1 },
            Direction::Down => XY { x: 0, y: 1 },
            Direction::Left => XY { x: -1, y: 0 },
            Direction::Right => XY { x: 1, y: 0 },
        };
        self.segments[0].x += movement.x;
        self.segments[0].y += movement.y;
        for i in 0..self.segments.len() {
            if i > 0 {
                println!("attach {i} to {}", i - 1);
                self.attach_to(i, i - 1);
            }
        }
    }
}

impl<const S: usize> From<[XY; S]> for Rope<S> {
    fn from(segments: [XY; S]) -> Self {
        Self { segments }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Move2 {
    amount: usize,
    direction: Direction,
}

#[derive(Copy, Clone, Hash, Eq, Ord, PartialEq, PartialOrd)]
struct XY {
    pub x: i32,
    pub y: i32,
}

fn main() {
    let input = fs::read_to_string("data/day9.txt").unwrap();

    let moves = parse_moves(&input);

    let simulation = RopeSimulation::from(moves.as_slice());

    let mut rope = Rope::from([XY { x: 0, y: 0 }; 10]);
    let mut visited = HashSet::new();
    visited.insert(XY { x: 0, y: 0 });

    for single in &moves {
        match single {
            Move::Up(amount) => {
                for _ in 0..*amount {
                    rope.apply_move(Direction::Up);
                    let coord = rope.segments.last().unwrap().clone();
                    visited.insert(coord);
                }
            }
            Move::Down(amount) => {
                for _ in 0..*amount {
                    rope.apply_move(Direction::Down);
                    let coord = rope.segments.last().unwrap().clone();
                    visited.insert(coord);
                }
            }
            Move::Left(amount) => {
                for _ in 0..*amount {
                    rope.apply_move(Direction::Left);
                    let coord = rope.segments.last().unwrap().clone();
                    visited.insert(coord);
                }
            }
            Move::Right(amount) => {
                for _ in 0..*amount {
                    rope.apply_move(Direction::Right);
                    let coord = rope.segments.last().unwrap().clone();
                    visited.insert(coord);
                }
            }
        }
    }

    println!("total positions: {}", simulation.visited_positions.len());
    println!("total positions for 10: {}", visited.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test2() {
        let input = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";

        let moves = parse_moves(&input);

        let simulation = RopeSimulation::from(moves.as_slice());

        let mut rope = Rope::from([XY { x: 0, y: 0 }; 10]);
        let mut visited = HashSet::new();

        for single in &moves {
            match single {
                Move::Up(amount) => {
                    for _ in 0..*amount {
                        rope.apply_move(Direction::Up);
                        let coord = rope.segments.last().unwrap().clone();
                        visited.insert(coord);
                    }
                }
                Move::Down(amount) => {
                    for _ in 0..*amount {
                        rope.apply_move(Direction::Down);
                        let coord = rope.segments.last().unwrap().clone();
                        visited.insert(coord);
                    }
                }
                Move::Left(amount) => {
                    for _ in 0..*amount {
                        rope.apply_move(Direction::Left);
                        let coord = rope.segments.last().unwrap().clone();
                        visited.insert(coord);
                    }
                }
                Move::Right(amount) => {
                    for _ in 0..*amount {
                        rope.apply_move(Direction::Right);
                        let coord = rope.segments.last().unwrap().clone();
                        visited.insert(coord);
                    }
                }
            }
        }

        println!("total positions: {}", simulation.visited_positions.len());
        println!("total positions for 10: {}", visited.len());
        assert_eq!(visited.len(), 36);
    }
}
