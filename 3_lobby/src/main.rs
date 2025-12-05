use std::fs;
use std::path::Path;

struct BatteryBank(Vec<u32>);

impl BatteryBank {
    fn new(line: &str) -> Self {
        Self(
            line.chars()
                .map(|letter| letter.to_digit(10).expect("Not a digit"))
                .collect(),
        )
    }

    fn highest_joltage(&self, cells_to_activate: usize) -> u64 {
        let mut start = 0;
        let mut end = self.0.len() - cells_to_activate + 1;

        let mut joltage = 0;

        for _ in 0..cells_to_activate {
            let (offset, digit) = highest_digit_in_sequence(&self.0[start..end]);
            joltage = 10 * joltage + digit as u64;

            start += offset + 1;
            end += 1;
        }

        joltage
    }
}

fn highest_digit_in_sequence(sequence: &[u32]) -> (usize, u32) {
    sequence
        .iter()
        .cloned()
        .enumerate()
        .rev()
        .max_by(|left, right| left.1.cmp(&right.1))
        .unwrap()
}

fn read_battery_banks<P>(input: P) -> Vec<BatteryBank>
where
    P: AsRef<Path>,
{
    fs::read_to_string(input)
        .expect("Cannot read battery bank file")
        .trim()
        .split('\n')
        .map(BatteryBank::new)
        .collect()
}

fn main() {
    let max_joltage: u64 = read_battery_banks("battery_banks")
        .into_iter()
        .map(|bank| bank.highest_joltage(12))
        .sum();

    println!("Maximum joltage: {}", max_joltage);
}
