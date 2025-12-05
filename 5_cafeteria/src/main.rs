use std::fs;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy)]
struct Delimiter {
    at: u64,
    opening: bool,
}

struct FreshnessTracker(Vec<RangeInclusive<u64>>);

impl FreshnessTracker {
    fn new(ranges: &str) -> Self {
        Self(
            ranges
                .trim()
                .split('\n')
                .map(|range| {
                    let mut parts = range.split('-');

                    let start = parts
                        .next()
                        .expect("Not a range")
                        .parse()
                        .expect("Not a number");

                    let end = parts
                        .next()
                        .expect("Not a range")
                        .parse()
                        .expect("Not a number");

                    start..=end
                })
                .collect(),
        )
    }

    fn all_fresh_ids(&self) -> u64 {
        let mut ranges = self.0.iter().fold(Vec::new(), |mut acc, x| {
            acc.push(Delimiter {
                at: *x.start(),
                opening: true,
            });

            acc.push(Delimiter {
                at: *x.end(),
                opening: false,
            });

            acc
        });

        ranges.sort_by(|l, r| r.opening.cmp(&l.opening));
        ranges.sort_by(|l, r| l.at.cmp(&r.at));

        let simplified = ranges
            .into_iter()
            .fold((Vec::new(), 0), |(mut acc, mut count), x| {
                match (count, x.opening) {
                    (0, true) => {
                        count += 1;
                        acc.push(x.at);
                    }

                    (_, true) => {
                        count += 1;
                    }

                    (1, false) => {
                        count -= 1;
                        acc.push(x.at);
                    }

                    (_, false) => {
                        count -= 1;
                    }
                }

                (acc, count)
            })
            .0;

        println!("{:#?}", simplified);

        simplified
            .iter()
            .step_by(2)
            .zip(simplified.iter().skip(1).step_by(2))
            .map(|(s, e)| e - s + 1)
            .sum()
    }
}

fn get_tracker_and_ingredients_list(input: &str) -> (FreshnessTracker, Vec<u64>) {
    let mut parts = input.trim().split("\n\n");

    let tracker = FreshnessTracker::new(parts.next().expect("No freshness list"));

    let ingredients = parts
        .next()
        .expect("No ingredients list")
        .split('\n')
        .map(|id| id.parse().expect("Not an ID"))
        .collect();

    (tracker, ingredients)
}

fn main() {
    let database =
        fs::read_to_string("ingredient_database").expect("Cannot read the ingredient database");

    let (tracker, _list) = get_tracker_and_ingredients_list(&database);

    let fresh_ids = tracker.all_fresh_ids();

    println!("In total, there are {} fresh ingredient IDs", fresh_ids);
}
