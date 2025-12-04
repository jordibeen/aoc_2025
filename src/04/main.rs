use std::time::Instant;

fn main() {
    println!("--- Day 4: Printing Department ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {}", pt1(&input));
    println!("Execution time: {:.2?}", start.elapsed());
}

fn pt1(input: &str) -> i32 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let n = grid.len();
    let m = grid[0].len();

    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(x, c)| {
                    if **c == '@' {
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
                            let xx = *x as isize + dir.1;
                            (yy >= 0 && yy < n as isize)
                                && (xx >= 0 && xx < m as isize)
                                && grid[yy as usize][xx as usize] == '@'
                        })
                        .count();

                        adjacent_roll_count < 4
                    } else {
                        false
                    }
                })
                .count() as i32
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
        assert_eq!(result, 13);
    }
}
