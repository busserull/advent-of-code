use std::fmt;
use std::fs;

struct AroundIndices {
    at: usize,
    end: usize,
    indices: [usize; 8],
}

impl AroundIndices {
    fn new(row: usize, col: usize, rows: usize, cols: usize) -> Self {
        let mut end = 0;
        let mut indices = [0; 8];

        let row_start = if row == 0 { 0 } else { row - 1 };
        let row_end = if row == rows - 1 { rows - 1 } else { row + 1 };

        let col_start = if col == 0 { 0 } else { col - 1 };
        let col_end = if col == cols - 1 { cols - 1 } else { col + 1 };

        for r in row_start..=row_end {
            for c in col_start..=col_end {
                if r == row && c == col {
                    continue;
                }

                indices[end] = c + r * cols;
                end += 1;
            }
        }

        Self {
            at: 0,
            end,
            indices,
        }
    }
}

impl Iterator for AroundIndices {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.at == self.end {
            return None;
        }

        let index = self.indices[self.at];
        self.at += 1;

        Some(index)
    }
}

struct PaperGrid {
    rolls: Vec<bool>,
    rows: usize,
    cols: usize,
}

impl PaperGrid {
    fn new(grid: &str) -> Self {
        let mut rolls = Vec::new();
        let mut cols = 0;

        for row in grid.trim().split('\n') {
            cols = row.len();
            rolls.extend(row.chars().map(|ch| ch == '@'));
        }

        let rows = rolls.len() / cols;

        Self { rolls, rows, cols }
    }

    fn is_occupied(&self, row: usize, col: usize) -> bool {
        self.rolls[col + row * self.cols]
    }

    fn adjacent_to(&self, row: usize, col: usize) -> usize {
        AroundIndices::new(row, col, self.rows, self.cols)
            .into_iter()
            .filter(|&index| self.rolls[index])
            .count()
    }

    fn remove_accessible(&mut self, inaccessible_limit: usize) -> Option<usize> {
        let mut removed = 0;

        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.adjacent_to(row, col) < inaccessible_limit {
                    removed += self.is_occupied(row, col) as usize;
                    self.rolls[col + row * self.cols] = false;
                }
            }
        }

        if removed == 0 {
            return None;
        }

        Some(removed)
    }
}

impl fmt::Display for PaperGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut start = 0;

        while start < self.rolls.len() {
            for position in &self.rolls[start..start + self.cols] {
                write!(f, "{}", if *position { '@' } else { '.' })?;
            }

            writeln!(f)?;

            start += self.cols;
        }

        Ok(())
    }
}

fn main() {
    let mut grid =
        PaperGrid::new(&fs::read_to_string("paper_rolls").expect("Cannot read the paper rolls"));

    let mut removed = 0;

    while let Some(rolls) = grid.remove_accessible(4) {
        removed += rolls;
    }

    println!("{} rolls can be removed using forklifts", removed);
}
