use std::fs;
use std::ops::RangeInclusive;
use std::path::Path;

fn is_invalid(mut id: usize) -> bool {
    let digit_count = (id as f64).log10() as usize + 1;

    if digit_count % 2 != 0 {
        return false;
    }

    let mut digits = Vec::with_capacity(digit_count);

    while id != 0 {
        digits.push(id % 10);
        id /= 10;
    }

    let mid = digit_count / 2;

    &digits[..mid] == &digits[mid..]
}

fn create_range(range_text: &str) -> RangeInclusive<usize> {
    let mut pair = range_text.split('-');

    let start = pair
        .next()
        .expect("No range start")
        .parse()
        .expect("Cannot interpret range start");

    let end = pair
        .next()
        .expect("No range end")
        .parse()
        .expect("Cannot interpret range end");

    start..=end
}

pub fn count_invalid_ids<P>(ranges_file: P) -> usize
where
    P: AsRef<Path>,
{
    fs::read_to_string(ranges_file)
        .expect("Cannot read the ranges file")
        .trim()
        .split(',')
        .map(create_range)
        .fold(0, |acc, range| {
            acc + range
                .into_iter()
                .filter(|&id| is_invalid(id))
                .sum::<usize>()
        })
}
