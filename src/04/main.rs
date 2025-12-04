use std::i32;
use std::time::Instant;

fn main() {
    println!("--- Day 4: Printing Department ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    let grid = parse(&input);
    println!("pt1: {}", pt1(grid.clone()));
    println!("pt2: {}", pt2(grid));
    println!("Execution time: {:.2?}", start.elapsed());
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

fn pt1(grid: Vec<Vec<char>>) -> i32 {
    let (_, removal_count) = remove_accessible_rolls(grid);
    removal_count
}

fn pt2(mut grid: Vec<Vec<char>>) -> i32 {
    let mut ans = 0;

    let mut removal_count = i32::MAX;
    while removal_count != 0 {
        (grid, removal_count) = remove_accessible_rolls(grid);
        ans += removal_count
    }

    ans
}

fn remove_accessible_rolls(mut grid: Vec<Vec<char>>) -> (Vec<Vec<char>>, i32) {
    let n = grid.len();
    let m = grid[0].len();

    let removals: Vec<(usize, usize)> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, c)| {
                    if *c == '@' {
                        let adjacent_roll_count = [
                            (-1, -1),
                            (-1, 0),
                            (-1, 1),
                            (0, -1),
                            (0, 1),
                            (1, -1),
                            (1, 0),
                            (1, 1),
                        ]
                        .iter()
                        .filter(|dir| {
                            let yy = y as isize + dir.0;
                            let xx = x as isize + dir.1;
                            (yy >= 0 && yy < n as isize)
                                && (xx >= 0 && xx < m as isize)
                                && grid[yy as usize][xx as usize] == '@'
                        })
                        .count();

                        if adjacent_roll_count < 4 {
                            return Some((y, x));
                        };
                    };
                    None
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect();

    removals.iter().for_each(|(y, x)| {
        grid[*y as usize][*x as usize] = '.';
    });

    (grid, removals.len() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let grid = parse(&input);
        let result = pt1(grid);
        assert_eq!(result, 13);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let grid = parse(&input);
        let result = pt2(grid);
        assert_eq!(result, 43);
    }
}
