use std::collections::HashMap;

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

    fn count_quantum_splits(&self) -> usize {
        let start_index = self
            .grid
            .iter()
            .position(|entry| matches!(entry, GridEntry::Source))
            .expect("There is no tachyon beam source");

        let mut memo = HashMap::new();

        self.count_quantum_splits_from(start_index, &mut memo)
    }

    fn count_quantum_splits_from(
        &self,
        mut index: usize,
        memo: &mut HashMap<usize, usize>,
    ) -> usize {
        if let Some(paths) = memo.get(&index) {
            return *paths;
        }

        let memo_index = index;

        while index < self.grid.len() && self.grid[index] != GridEntry::Splitter {
            index += self.columns;
        }

        if index > self.grid.len() {
            memo.insert(memo_index, 1);
            return 1;
        }

        let left = (index % self.columns > 0)
            .then(|| self.count_quantum_splits_from(index - 1, memo))
            .unwrap_or(0);

        let right = (index % self.columns < self.columns - 1)
            .then(|| self.count_quantum_splits_from(index + 1, memo))
            .unwrap_or(0);

        memo.insert(memo_index, left + right);

        left + right
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

        self.grid
            .iter()
            .zip(beams.into_iter())
            .filter(|pair| matches!(pair, (GridEntry::Splitter, true)))
            .count()
    }
}

fn main() {
    let diagram = SplitterDiagram::new(
        &std::fs::read_to_string("diagram").expect("Cannot read the tachyon diagram"),
    );

    println!(
        "There are {} splitters being hit in this diagram",
        diagram.count_splits()
    );

    println!(
        "There are {} possible tachyon splits in this diagram",
        diagram.count_quantum_splits()
    );
}
