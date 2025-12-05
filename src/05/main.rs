use std::time::Instant;

fn main() {
    println!("--- Day 5: Cafeteria ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    let (fresh_ranges, ids) = parse(&input);
    println!("pt1: {}", pt1(&fresh_ranges, &ids));
    println!("pt2: {}", pt2(&fresh_ranges));
    println!("Execution time: {:.2?}", start.elapsed());
}

fn parse(input: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    input
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
        .unwrap()
}

fn pt1(fresh_ranges: &Vec<(i64, i64)>, ids: &Vec<i64>) -> i32 {
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

fn pt2(fresh_ranges: &Vec<(i64, i64)>) -> i64 {
    let mut saved_ranges: Vec<(i64, i64)> = vec![];

    fresh_ranges.iter().for_each(|range| {
        let mut current_ranges = vec![*range];

        saved_ranges.iter().for_each(|saved| {
            let mut new_ranges = vec![];

            current_ranges.iter().for_each(|curr| {
                if curr.1 < saved.0 || curr.0 > saved.1 {
                    new_ranges.push(*curr);
                } else if curr.0 < saved.0 && curr.1 >= saved.0 {
                    new_ranges.push((curr.0, saved.0 - 1));
                } else if curr.0 <= saved.1 && curr.1 > saved.1 {
                    new_ranges.push((saved.1 + 1, curr.1));
                } else if curr.0 < saved.0 && curr.1 > saved.1 {
                    new_ranges.push((curr.0, saved.0 - 1));
                    new_ranges.push((saved.1 + 1, curr.1));
                }
            });

            current_ranges = new_ranges;
        });

        saved_ranges.extend(current_ranges);
    });

    saved_ranges.iter().map(|saved| saved.1 - saved.0 + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let (fresh_ranges, ids) = parse(&input);
        let result = pt1(&fresh_ranges, &ids);
        assert_eq!(result, 3);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let (fresh_ranges, _) = parse(&input);
        let result = pt2(&fresh_ranges);
        assert_eq!(result, 14);
    }
}
