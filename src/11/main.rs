use std::collections::{HashMap, VecDeque};
use std::time::Instant;

fn main() {
    println!("--- Day 11: Reactor ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {}", pt1(&input));
    println!("Execution time: {:.2?}", start.elapsed());
}

fn pt1(input: &str) -> i32 {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    input.lines().for_each(|line| {
        let (from, to) = line
            .split_once(": ")
            .map(|(from, to)| (from, to.split_whitespace().collect()))
            .unwrap();

        map.insert(from, to);
    });

    let mut ans = 0;
    let mut queue = VecDeque::from(["you"]);
    while let Some(curr) = queue.pop_front() {
        if curr == "out" {
            ans += 1;
        }

        if let Some(to) = map.get(curr) {
            to.iter().for_each(|next| {
                queue.push_back(next);
            });
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let result = pt1(&input);
        assert_eq!(result, 5);
    }
}
