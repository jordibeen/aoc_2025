use std::collections::VecDeque;
use std::time::Instant;

use regex::Regex;

type ManualEntry = (Vec<bool>, Vec<Vec<usize>>, Vec<usize>);

fn main() {
    println!("--- Day 10: Factory ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {}", pt1(&input));
    println!("Execution time: {:.2?}", start.elapsed());
}

fn pt1(input: &str) -> i32 {
    let manual: Vec<ManualEntry> = input
        .lines()
        .map(|line| {
            let caps = Regex::new(r"\[(?<a>.*)\] (?<b>.*) \{(?<c>.*)\}")
                .unwrap()
                .captures(line)
                .unwrap();

            let diagram = caps["a"]
                .chars()
                .filter_map(|c| match c {
                    '.' => Some(false),
                    '#' => Some(true),
                    _ => None,
                })
                .collect::<Vec<bool>>();

            let buttons = caps["b"]
                .split(" ")
                .map(|s| {
                    s.chars()
                        .filter_map(|c| match c {
                            '(' => None,
                            ')' => None,
                            ',' => None,
                            _ => Some(c.to_digit(10).unwrap() as usize),
                        })
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>();

            let joltage = caps["c"]
                .split(",")
                .map(|c| usize::from_str_radix(c, 16).unwrap())
                .collect::<Vec<usize>>();

            (diagram, buttons, joltage)
        })
        .collect();

    manual
        .iter()
        .map(|(diagram, buttons, _)| {
            let mut fewest = 0;

            let mut queue = VecDeque::from([(vec![false; diagram.len()], 0)]);
            while let Some((curr, presses)) = queue.pop_front() {
                if diagram == &curr {
                    fewest = presses;
                    break;
                }

                buttons.iter().for_each(|b| {
                    let lights = curr
                        .iter()
                        .enumerate()
                        .map(|(i, v)| if b.contains(&i) { !v } else { *v })
                        .collect::<Vec<bool>>();
                    queue.push_back((lights, presses + 1));
                });
            }

            fewest
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let result = pt1(&input);
        assert_eq!(result, 7);
    }
}
