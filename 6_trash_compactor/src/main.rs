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
    rows: Vec<Vec<u64>>,
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
            .map(|line| {
                line.trim()
                    .split_whitespace()
                    .map(|digits| digits.parse::<u64>().unwrap())
                    .collect()
            })
            .collect();

        Challenges { rows, ops }
    }

    fn solve_columns(&self) -> u64 {
        self.ops
            .iter()
            .enumerate()
            .map(|(i, op)| {
                self.rows
                    .iter()
                    .map(|row| row[i])
                    .reduce(|l, r| op.apply(l, r))
                    .unwrap()
            })
            .sum()
    }
}

fn main() {
    let challenges = Challenges::new(
        &fs::read_to_string("math_challenges").expect("Could not read challenge sheet"),
    );

    println!("The check is {}", challenges.solve_columns());
}
