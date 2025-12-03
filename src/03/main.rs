use std::time::Instant;

fn main() {
    println!("--- Day 3: Lobby ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {} (finished in {:.2?})", pt1(&input), start.elapsed());
    println!("pt2: {} (finished in {:.2?})", pt2(&input), start.elapsed());
}

fn pt1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let n = line.len();
            let (battery_1, battery_2) =
                line.chars()
                    .enumerate()
                    .fold((0, 0), |(battery_1, battery_2), (i, c)| {
                        let voltage = c.to_digit(10).unwrap() as i8;
                        if voltage > battery_1 && i != n - 1 {
                            (voltage, 0)
                        } else if voltage > battery_2 {
                            (battery_1, voltage)
                        } else {
                            (battery_1, battery_2)
                        }
                    });
            format!("{}{}", battery_1, battery_2)
                .parse::<i32>()
                .unwrap()
        })
        .sum()
}

fn pt2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let n = line.len();
            let batteries =
                line.chars()
                    .enumerate()
                    .fold(vec![0_i8; 12], |mut batteries, (i, c)| {
                        let voltage = c.to_digit(10).unwrap() as i8;
                        for (o, battery) in batteries.clone().iter().enumerate() {
                            if voltage > *battery && 11 - o <= n - i - 1 {
                                batteries[o] = voltage;
                                if o < 11 {
                                    batteries[o + 1..12].iter_mut().for_each(|x| *x = 0);
                                }
                                break;
                            }
                        }
                        batteries
                    });

            batteries
                .iter()
                .map(|battery| battery.to_string())
                .collect::<String>()
                .parse::<i64>()
                .unwrap()
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
        assert_eq!(result, 357);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let result = pt2(&input);
        assert_eq!(result, 3121910778619);
    }
}
