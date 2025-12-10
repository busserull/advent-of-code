use std::fs;

enum Op {
    Add,
    Mul,
}

impl Op {
    fn apply(&self, l: u64, r: u64) -> u64 {
        match self {
            Op::Add => l + r,
            Op::Mul => l * r,
        }
    }
}

struct Challenges {
    rows: Vec<String>,
    ops: Vec<Op>,
}

impl Challenges {
    fn new(input: &str) -> Self {
        let lines: Vec<_> = input.trim().split('\n').collect();

        let ops = lines
            .last()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|chs| match chs.chars().next().unwrap() {
                '+' => Op::Add,
                '*' => Op::Mul,
                _ => unreachable!(),
            })
            .collect();

        let rows = lines[..lines.len() - 1]
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        Challenges { rows, ops }
    }

    fn solve_columns(&self) -> u64 {
        self.ops
            .iter()
            .zip(CephalopodIter::new(&self.rows).into_iter())
            .map(|(op, column)| {
                column
                    .into_iter()
                    .reduce(|acc, x| op.apply(acc, x))
                    .unwrap()
            })
            .sum()
    }
}

struct CephalopodIter {
    start: usize,
    separators: Vec<usize>,
    end_index: usize,
    rows: Vec<String>,
}

impl CephalopodIter {
    fn new(rows: &[String]) -> Self {
        let mut separators: Vec<usize> = (0..rows[0].len())
            .into_iter()
            .filter(|i| rows.iter().all(|row| row.chars().nth(*i) == Some(' ')))
            .collect();

        separators.push(rows[0].len());

        let rows = rows.into_iter().map(|s| s.to_string()).collect();

        Self {
            start: 0,
            separators,
            end_index: 0,
            rows,
        }
    }
}

impl Iterator for CephalopodIter {
    type Item = Vec<u64>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end_index >= self.separators.len() {
            return None;
        }

        let start = self.start;
        let end = self.separators[self.end_index];

        self.start = end + 1;
        self.end_index += 1;

        Some(
            (start..end)
                .into_iter()
                .rev()
                .map(|col| {
                    (0..self.rows.len())
                        .into_iter()
                        .map(|i| self.rows[i].chars().nth(col).unwrap())
                        .fold(0, |acc, digit| {
                            if let Some(digit) = digit.to_digit(10) {
                                acc * 10 + digit as u64
                            } else {
                                acc
                            }
                        })
                })
                .collect(),
        )
    }
}

fn main() {
    let challenges = Challenges::new(
        &fs::read_to_string("math_challenges").expect("Could not read challenge sheet"),
    );

    println!("The check is {}", challenges.solve_columns());
}
