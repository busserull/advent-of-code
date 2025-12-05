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

    fn highest_joltage(&self) -> (u32, u32) {
        let (index, first) = self.0[..self.0.len() - 1]
            .into_iter()
            .enumerate()
            .map(|(index, &jolt)| (index, jolt))
            .reduce(|left, right| highest_digit(left, right))
            .unwrap();

        let second = self.0[index + 1..].into_iter().max().cloned().unwrap();

        (first, second)
    }
}

fn highest_digit(left: (usize, u32), right: (usize, u32)) -> (usize, u32) {
    if right.1 > left.1 { right } else { left }
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
    let max_jolts: u32 = read_battery_banks("battery_banks")
        .into_iter()
        .map(|bank| bank.highest_joltage())
        .map(|(f, s)| f * 10 + s)
        .sum();

    println!("Max jolts: {}", max_jolts);
}
