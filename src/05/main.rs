use std::time::Instant;

fn main() {
    println!("--- Day 5: Cafeteria ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {} (finished in {:.2?})", pt1(&input), start.elapsed());
}

fn pt1(input: &str) -> i32 {
    let (fresh_ranges, ids): (Vec<(i64, i64)>, Vec<i64>) = input
        .split_once("\n\n")
        .map(|(s1, s2)| {
            (
                s1.trim()
                    .lines()
                    .map(|line| {
                        line.split_once("-")
                            .map(|(l, r)| (l.parse::<i64>().unwrap(), r.parse::<i64>().unwrap()))
                            .unwrap()
                    })
                    .collect::<Vec<(i64, i64)>>(),
                s2.trim()
                    .lines()
                    .map(|line| line.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>(),
            )
        })
        .unwrap();

    ids.iter()
        .filter(|id| {
            fresh_ranges
                .iter()
                .filter(|(l, r)| id >= &l && id <= &r)
                .count()
                > 0
        })
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let result = pt1(&input);
        assert_eq!(result, 3);
    }
}
