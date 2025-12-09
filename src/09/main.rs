use std::time::Instant;

fn main() {
    println!("--- Day 9: Movie Theater ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {}", pt1(&input));
    println!("Execution time: {:.2?}", start.elapsed());
}

fn pt1(input: &str) -> i64 {
    let tiles = input
        .lines()
        .map(|line| {
            let s = line.split_once(',').unwrap();
            (s.0.parse::<isize>().unwrap(), s.1.parse::<isize>().unwrap())
        })
        .collect::<Vec<(isize, isize)>>();

    let mut max_rectangle: i64 = 0;
    tiles.iter().for_each(|a| {
        tiles.iter().for_each(|b| {
            let rectangle = a.0.abs_diff(b.0 + 1) * a.1.abs_diff(b.1 + 1);
            max_rectangle = max_rectangle.max(rectangle as i64);
        });
    });

    max_rectangle
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let result = pt1(&input);
        assert_eq!(result, 50);
    }
}
