use std::time::Instant;

fn main() {
    println!("--- Day 1: Secret Entrance ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {} (finished in {:.2?})", pt1(&input), start.elapsed());
}

fn pt1(input: &str) -> i32 {
    let mut dial: i32 = 50;
    input
        .lines()
        .filter(|x| {
            let (direction, distance) = x.split_at(1);
            println!("dial: {:?}", dial);
            println!("dir: {:?}", direction);
            println!("distance: {:?}", distance);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example_pt1.txt");
        let result = pt1(&input);
        assert_eq!(result, 3);
    }
}
