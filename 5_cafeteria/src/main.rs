use std::collections::HashSet;
use std::fs;
use std::ops::RangeInclusive;

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

    fn all_fresh_ids(&self) -> HashSet<u64> {
        let mut set = HashSet::new();

        for range in self.0.iter().cloned() {
            println!("Range {:?}", range);
            for id in range {
                set.insert(id);
            }
        }

        set
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

    println!(
        "In total, there are {} fresh ingredient IDs",
        fresh_ids.len()
    );
}
