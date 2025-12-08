use std::collections::{HashMap, HashSet};
use std::time::Instant;

type JunctionBox = (isize, isize, isize);

fn main() {
    println!("--- Day 8: Playground ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {}", pt1(&input, 1000));
    println!("Execution time: {:.2?}", start.elapsed());
}

fn pt1(input: &str, max_connections: i32) -> i32 {
    let mut circuit: HashMap<JunctionBox, i32> = HashMap::new();
    input.lines().for_each(|line| {
        let split = line
            .split(',')
            .map(|s| s.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        circuit.insert((split[0], split[1], split[2]), 0);
    });

    let mut distances: Vec<(isize, JunctionBox, JunctionBox)> = Vec::new();
    circuit.iter().for_each(|(a, _)| {
        circuit.keys().filter(|b| a != *b).for_each(|b| {
            let euclidean_distance =
                ((b.0 - a.0).pow(2) + (b.1 - a.1).pow(2) + (b.2 - a.2).pow(2)).isqrt();

            distances.push((euclidean_distance, *a, *b));
        });
    });
    distances.sort_unstable_by_key(|x| x.0);

    let mut seen: HashSet<(JunctionBox, JunctionBox)> = HashSet::new();
    let mut i = 0;
    for (_, a, b) in distances {
        if i == max_connections {
            break;
        }
        if seen.contains(&(a, b)) || seen.contains(&(b, a)) {
            continue;
        }

        let a_conn = *circuit.get(&a).unwrap();
        let b_conn = *circuit.get(&b).unwrap();

        match (a_conn, b_conn) {
            (0, 0) => {
                *circuit.get_mut(&a).unwrap() = i;
                *circuit.get_mut(&b).unwrap() = i;
            }
            (a_id, 0) => *circuit.get_mut(&b).unwrap() = a_id,
            (0, b_id) => *circuit.get_mut(&a).unwrap() = b_id,
            (a_id, b_id) if a_id != b_id => {
                circuit
                    .values_mut()
                    .filter(|b_conn| **b_conn == b_id)
                    .for_each(|a_conn| *a_conn = a_id);
            }
            _ => {}
        }

        seen.insert((a, b));
        i += 1;
    }

    let mut counts = HashMap::new();
    circuit.values().filter(|v| *v != &0).for_each(|v| {
        *counts.entry(v).or_insert(0) += 1;
    });

    let mut sorted_counts = counts.into_values().collect::<Vec<i32>>();
    sorted_counts.sort_by(|a, b| b.cmp(a));

    sorted_counts.iter().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let result = pt1(&input, 10);
        assert_eq!(result, 40);
    }
}
