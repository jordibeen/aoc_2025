use std::collections::{HashMap, HashSet};
use std::time::Instant;

type JunctionBox = (isize, isize, isize);

fn main() {
    println!("--- Day 8: Playground ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    let (circuit_map, distances) = parse(&input);
    println!("pt1: {}", pt1(circuit_map.clone(), &distances, 1000));
    println!("pt2: {}", pt2(circuit_map, &distances));
    println!("Execution time: {:.2?}", start.elapsed());
}
fn parse(
    input: &str,
) -> (
    HashMap<JunctionBox, i32>,
    Vec<(isize, JunctionBox, JunctionBox)>,
) {
    let mut circuit_map: HashMap<JunctionBox, i32> = HashMap::new();
    input.lines().for_each(|line| {
        let split = line
            .split(',')
            .map(|s| s.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        circuit_map.insert((split[0], split[1], split[2]), 0);
    });

    let mut distances: Vec<(isize, JunctionBox, JunctionBox)> = Vec::new();
    circuit_map.iter().for_each(|(a, _)| {
        circuit_map.keys().filter(|b| a != *b).for_each(|b| {
            let euclidean_distance =
                ((b.0 - a.0).pow(2) + (b.1 - a.1).pow(2) + (b.2 - a.2).pow(2)).isqrt();

            distances.push((euclidean_distance, *a, *b));
        });
    });
    distances.sort_unstable_by_key(|x| x.0);

    (circuit_map, distances)
}

fn pt1(
    circuit_map: HashMap<JunctionBox, i32>,
    distances: &Vec<(isize, JunctionBox, JunctionBox)>,
    limit: i32,
) -> i32 {
    let (resulting_circuit_map, _) = make_connections(circuit_map.clone(), distances, Some(limit));

    let mut counts = HashMap::new();
    resulting_circuit_map
        .values()
        .filter(|v| *v != &0)
        .for_each(|v| {
            *counts.entry(v).or_insert(0) += 1;
        });

    let mut sorted_counts = counts.into_values().collect::<Vec<i32>>();
    sorted_counts.sort_by(|a, b| b.cmp(a));

    sorted_counts.iter().take(3).product()
}

fn pt2(
    circuit_map: HashMap<JunctionBox, i32>,
    distances: &Vec<(isize, JunctionBox, JunctionBox)>,
) -> i32 {
    let (_, last_connection) = make_connections(circuit_map.clone(), distances, None);

    (last_connection.0.0 * last_connection.1.0) as i32
}

fn make_connections(
    mut circuit_map: HashMap<JunctionBox, i32>,
    distances: &Vec<(isize, JunctionBox, JunctionBox)>,
    limit: Option<i32>,
) -> (HashMap<JunctionBox, i32>, (JunctionBox, JunctionBox)) {
    let mut last_connection: (JunctionBox, JunctionBox) = ((0, 0, 0), (0, 0, 0));

    let mut seen: HashSet<(JunctionBox, JunctionBox)> = HashSet::new();
    let mut i = 0;
    for (_, a, b) in distances {
        // pt1
        if let Some(max_connections) = limit {
            if i == max_connections {
                break;
            }
        // pt2
        } else {
            let circuit_id = circuit_map.values().next().unwrap();
            if circuit_map.values().all(|v| v != &0 && v == circuit_id) {
                break;
            }
        }

        if seen.contains(&(*a, *b)) || seen.contains(&(*b, *a)) {
            continue;
        }

        let a_circuit = *circuit_map.get(&a).unwrap();
        let b_circuit = *circuit_map.get(&b).unwrap();

        match (a_circuit, b_circuit) {
            (0, 0) => {
                *circuit_map.get_mut(&a).unwrap() = i;
                *circuit_map.get_mut(&b).unwrap() = i;
            }
            (a_id, 0) => *circuit_map.get_mut(&b).unwrap() = a_id,
            (0, b_id) => *circuit_map.get_mut(&a).unwrap() = b_id,
            (a_id, b_id) if a_id != b_id => {
                circuit_map
                    .values_mut()
                    .filter(|b_circuit| **b_circuit == b_id)
                    .for_each(|a_circuit| *a_circuit = a_id);
            }
            _ => {}
        }

        seen.insert((*a, *b));
        last_connection = (*a, *b);
        i += 1;
    }

    (circuit_map, last_connection)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let (circuit_map, distances) = parse(&input);
        let result = pt1(circuit_map, &distances, 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let (circuit_map, distances) = parse(&input);
        let result = pt2(circuit_map, &distances);
        assert_eq!(result, 25272);
    }
}
