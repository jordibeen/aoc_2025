use std::time::Instant;

fn main() {
    println!("--- Day 6: Trash Compactor ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {}", pt1(&input));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let result = pt1(&input);
        assert_eq!(result, 4277556);
    }
}
