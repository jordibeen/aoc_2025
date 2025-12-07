use std::time::Instant;

fn main() {
    println!("--- Day 7: Laboratories ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {}", pt1(&input));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let result = pt1(&input);
        assert_eq!(result, 21);
    }
}
