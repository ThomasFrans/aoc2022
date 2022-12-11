use std::fs;

type Tree = u8;

#[derive(Debug)]
struct Trees(Vec<Vec<Tree>>);

impl Trees {
    fn is_tree_visible(&self, row: usize, column: usize) -> bool {
        let height = self.0[row][column];
        let mut start_y = true;
        let mut end_y = true;
        let mut start_x = true;
        let mut end_x = true;
        for row_index in 0..row {
            if self.0[row_index][column] >= height {
                start_y = false;
            }
        }
        for row_index in row + 1..self.rows() {
            if self.0[row_index][column] >= height {
                end_y = false;
            }
        }
        for column_index in 0..column {
            if self.0[row][column_index] >= height {
                start_x = false;
            }
        }
        for column_index in column + 1..self.columns() {
            if self.0[row][column_index] >= height {
                end_x = false;
            }
        }
        start_x || end_x || start_y || end_y
    }

    fn rows(&self) -> usize {
        self.0.len()
    }

    fn columns(&self) -> usize {
        self.0[0].len()
    }

    fn visible_trees(&self) -> usize {
        let mut total = 0;
        for i in 0..self.rows() {
            for j in 0..self.columns() {
                if self.is_tree_visible(i, j) {
                    total += 1;
                }
            }
        }
        total
    }

    fn tree_scenic_score(&self, row: usize, column: usize) -> usize {
        let own_height = self.0[row][column];
        let mut viewing_distance_top = 0;
        let mut viewing_distance_right = 0;
        let mut viewing_distance_bottom = 0;
        let mut viewing_distance_left = 0;
        for i in (0..column).rev() {
            if self.0[row][i] < own_height {
                viewing_distance_left += 1;
            } else {
                viewing_distance_left += 1;
                break;
            }
        }
        for i in column + 1..self.columns() {
            if self.0[row][i] < own_height {
                viewing_distance_right += 1;
            } else {
                viewing_distance_right += 1;
                break;
            }
        }
        for i in (0..row).rev() {
            if self.0[i][column] < own_height {
                viewing_distance_top += 1;
            } else {
                viewing_distance_top += 1;
                break;
            }
        }
        for i in row + 1..self.rows() {
            if self.0[i][column] < own_height {
                viewing_distance_bottom += 1;
            } else {
                viewing_distance_bottom += 1;
                break;
            }
        }

        viewing_distance_top
            * viewing_distance_right
            * viewing_distance_bottom
            * viewing_distance_left
    }

    fn best_scenic_score(&self) -> usize {
        let mut best_score = 0;
        for row_index in 0..self.rows() {
            for column_index in 0..self.columns() {
                if self.tree_scenic_score(row_index, column_index) > best_score {
                    best_score = self.tree_scenic_score(row_index, column_index);
                }
            }
        }
        best_score
    }
}

impl TryFrom<&str> for Trees {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut result = Trees(Vec::new());
        for line in value.lines().enumerate() {
            result.0.push(Vec::new());
            for char in line.1.chars() {
                result.0[line.0].push(char.to_digit(10).unwrap() as u8);
            }
        }
        Ok(result)
    }
}

fn main() {
    let input = fs::read_to_string("data/day8.txt").unwrap();

    let trees = Trees::try_from(input.as_str()).unwrap();

    println!("total visible: {}", trees.visible_trees());
    println!("best scenic score: {}", trees.best_scenic_score());
}
