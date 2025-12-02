use core::num;
use std::time::Instant;

fn main() {
    println!("--- Day 1: Secret Entrance ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {} (finished in {:.2?})", pt1(&input), start.elapsed());
}

fn pt1(input: &str) -> i64 {
    let mut sum = 0;
    input.split(",").for_each(|range| {
        let (start, end) = range
            .trim()
            .split_once("-")
            .map(|s| (s.0.parse::<i64>().unwrap(), s.1.parse::<i64>().unwrap()))
            .unwrap();

        (start..end + 1).for_each(|num| {
            let id = num.to_string();
            let (l, r) = id.split_at(id.len() / 2);
            if l == r {
                sum += id.parse::<i64>().unwrap();
            }
        });
    });
    sum as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let result = pt1(&input);
        assert_eq!(result, 1227775554);
    }
}
