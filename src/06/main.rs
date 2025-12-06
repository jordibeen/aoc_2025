use std::time::Instant;

fn main() {
    println!("--- Day 6: Trash Compactor ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {}", pt1(&input));
    println!("pt2: {}", pt2(&input));
    println!("Execution time: {:.2?}", start.elapsed());
}

fn pt1(input: &str) -> i64 {
    let mut grid = input
        .lines()
        .map(|row| {
            row.split_whitespace()
                .map(|col| col.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();
    let operators = grid.pop().unwrap();

    (0..grid[0].len())
        .map(|x| match operators[x].as_ref() {
            "+" => (0..grid.len())
                .map(|y| grid[y][x].parse::<i64>().unwrap())
                .sum::<i64>(),
            "*" => (0..grid.len())
                .map(|y| grid[y][x].parse::<i64>().unwrap())
                .product::<i64>(),
            _ => unreachable!(),
        })
        .sum()
}

fn pt2(input: &str) -> i64 {
    let mut grid = input
        .lines()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let operators = grid
        .pop()
        .unwrap()
        .into_iter()
        .filter(|x| x != &' ')
        .collect::<Vec<char>>();

    let nums = (0..grid[0].len())
        .map(|x| {
            let s = (0..grid.len())
                .filter_map(|y| (grid[y][x] != ' ').then_some(grid[y][x]))
                .collect::<String>();

            if !s.is_empty() {
                Some(s.parse::<i64>().unwrap())
            } else {
                None
            }
        })
        .collect::<Vec<Option<i64>>>();

    let groups: Vec<Vec<i64>> = nums
        .split(|x| x.is_none())
        .map(|group| group.iter().filter_map(|&x| x).collect())
        .collect();

    groups
        .iter()
        .enumerate()
        .map(|(i, column)| match operators[i] {
            '+' => column.iter().sum::<i64>(),
            '*' => column.iter().product::<i64>(),
            _ => unreachable!(),
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
        assert_eq!(result, 4277556);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let result = pt2(&input);
        assert_eq!(result, 3263827);
    }
}
