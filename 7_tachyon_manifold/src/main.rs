#[derive(Clone, Copy, PartialEq, Eq)]
enum GridEntry {
    Source,
    Space,
    Splitter,
}

struct SplitterDiagram {
    grid: Vec<GridEntry>,
    rows: usize,
    columns: usize,
}

impl SplitterDiagram {
    fn new(grid: &str) -> Self {
        let lines = grid.trim().split('\n');

        let columns = lines.clone().next().expect("Empty grid").chars().count();
        let rows = lines.clone().into_iter().count();

        let mut grid = Vec::with_capacity(rows * columns);

        for line in lines {
            for ch in line.chars() {
                use GridEntry::{Source, Space, Splitter};

                let entry = match ch {
                    'S' => Source,
                    '.' => Space,
                    '^' => Splitter,
                    _ => panic!("Unsupported grid entry"),
                };

                grid.push(entry);
            }
        }

        Self {
            grid,
            rows,
            columns,
        }
    }

    fn count_splits(&self) -> usize {
        let mut beams = vec![false; self.rows * self.columns];

        for (entry, space) in self.grid.iter().zip(beams.iter_mut()) {
            if matches!(entry, GridEntry::Source) {
                *space = true;
            }
        }

        for row in 0..self.rows - 1 {
            for col in 0..self.columns {
                let index = self.columns * row + col;

                if beams[index] && matches!(self.grid[index], GridEntry::Splitter) {
                    if index % self.columns > 0 {
                        beams[index - 1] = true;
                    }
                    if index % self.columns < self.columns - 1 {
                        beams[index + 1] = true;
                    }
                }
            }

            for col in 0..self.columns {
                let index = self.columns * row + col;

                if beams[index] && !matches!(self.grid[index], GridEntry::Splitter) {
                    beams[index + self.columns] = true;
                }
            }
        }

        /*

        for row in 0..self.rows {
            for col in 0..self.columns {
                let index = row * self.columns + col;
                use GridEntry::*;
                match self.grid[index] {
                    Source => print!("S"),
                    Splitter => print!("^"),
                    Space => print!("."),
                }
            }
            println!();
        }

        println!();

        for row in 0..self.rows {
            for col in 0..self.columns {
                let index = row * self.columns + col;
                use GridEntry::*;
                match (self.grid[index], beams[index]) {
                    (Source, _) => print!("S"),
                    (Splitter, true) => print!("x"),
                    (Splitter, false) => print!("^"),
                    (Space, true) => print!("|"),
                    (Space, false) => print!("."),
                }
            }
            println!();
        }

        */

        self.grid
            .iter()
            .zip(beams.into_iter())
            .filter(|pair| matches!(pair, (GridEntry::Splitter, true)))
            .count()
    }
}

fn main() {
    /*
        let input = "
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";
    */

    let diagram = SplitterDiagram::new(
        &std::fs::read_to_string("diagram").expect("Cannot read the tachyon diagram"),
    );

    println!(
        "The tachyon beams make {} splits in this diagram",
        diagram.count_splits()
    );
}
