use std::collections::HashMap;
use std::time::Instant;

fn main() {
    println!("--- Day 7: Laboratories ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {}", pt1(&input));
    println!("pt2: {}", pt2(&input));
    println!("Execution time: {:.2?}", start.elapsed());
}

fn pt1(input: &str) -> i32 {
    let mut beams = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .position(|c| c == 'S')
        .into_iter()
        .collect::<Vec<usize>>();

    let mut splits = 0;
    input.lines().for_each(|row| {
        row.chars().enumerate().for_each(|(x, c)| {
            if c == '^' && beams.contains(&x) {
                beams.retain(|v| v != &x);
                beams.push(x - 1);
                beams.push(x + 1);
                splits += 1;
            }
        })
    });

    splits
}

fn pt2(input: &str) -> i64 {
    let mut starting_pos = (0, 0);
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        starting_pos = (y, x);
                    }
                    c
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    ways(&mut HashMap::new(), &grid, starting_pos)
}

fn ways(
    cache: &mut HashMap<(usize, usize), i64>,
    grid: &Vec<Vec<char>>,
    pos: (usize, usize),
) -> i64 {
    if let Some(cached_val) = cache.get(&pos) {
        return *cached_val;
    }

    let mut timelines = 0;
    if pos.0 + 1 == grid.len() - 1 {
        timelines = 1;
    } else {
        if grid[pos.0 + 1][pos.1] == '^' {
            timelines += ways(cache, grid, (pos.0 + 1, pos.1 - 1));
            timelines += ways(cache, grid, (pos.0 + 1, pos.1 + 1));
        } else {
            timelines += ways(cache, grid, (pos.0 + 1, pos.1));
        }
    }

    cache.insert(pos, timelines);

    timelines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let result = pt1(&input);
        assert_eq!(result, 21);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let result = pt2(&input);
        assert_eq!(result, 40);
    }
}
