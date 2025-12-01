use std::time::Instant;

fn main() {
    println!("--- Day 1: Secret Entrance ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {} (finished in {:.2?})", pt1(&input), start.elapsed());
    println!("pt2: {} (finished in {:.2?})", pt2(&input), start.elapsed());
}

fn pt1(input: &str) -> i32 {
    let mut dial: i32 = 50;
    input
        .lines()
        .filter(|x| {
            let (direction, distance) = x.split_at(1);
            match direction {
                "L" => {
                    dial -= distance.parse::<i32>().unwrap();
                }
                "R" => {
                    dial += distance.parse::<i32>().unwrap();
                }
                _ => unreachable!(),
            }
            dial % 100 == 0
        })
        .count() as i32
}

fn pt2(input: &str) -> i32 {
    let mut dial: i32 = 50;
    input
        .lines()
        .map(|x| {
            let (direction, distance) = x.split_at(1);
            (0..distance.parse::<i32>().unwrap())
                .filter(|_| {
                    match direction {
                        "L" => dial -= 1,
                        "R" => dial += 1,
                        _ => unreachable!(),
                    };
                    dial % 100 == 0
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
        assert_eq!(result, 3);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let result = pt2(&input);
        assert_eq!(result, 6);
    }
}
