use std::fs;
use std::path::Path;

enum Direction {
    Left,
    Right,
}

struct SafeDial(i32);

impl SafeDial {
    fn new() -> Self {
        Self(50)
    }

    fn rotate(&mut self, direction: Direction, clicks: i32) {
        let change = match direction {
            Direction::Left => -clicks,
            Direction::Right => clicks,
        };

        self.0 = (self.0 + change) % 100;
    }

    fn read(&self) -> i32 {
        self.0
    }
}

pub fn get_entrance_password<P>(input: P) -> usize
where
    P: AsRef<Path>,
{
    let instructions = fs::read_to_string(input).expect("Could not read the safe instructions");

    let mut dial = SafeDial::new();
    let mut password = 0;

    for line in instructions.trim().split('\n') {
        let direction = match line.chars().next() {
            Some('L') => Direction::Left,
            Some('R') => Direction::Right,
            _ => panic!("Can't understand rotation direction '{}'", line),
        };

        let clicks = line[1..]
            .parse::<i32>()
            .expect("Can't read rotation clicks");

        dial.rotate(direction, clicks);

        if dial.read() == 0 {
            password += 1;
        }
    }

    password
}
