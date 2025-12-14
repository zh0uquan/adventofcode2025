use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Coord = (usize, usize, usize);

trait Distance {
    fn distance(&self, other: &Self) -> f64;
}

impl Distance for Coord {
    fn distance(&self, other: &Self) -> f64 {
        let dx = self.0 as f64 - other.0 as f64;
        let dy = self.1 as f64 - other.1 as f64;
        let dz = self.2 as f64 - other.2 as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input, 1000));
    println!("{:?}", part2(input));
}

fn get_distances(input: &str) -> Vec<(Coord, Coord, f64)> {
    let input: Vec<Coord> = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|coord| coord.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|coords| (coords[0], coords[1], coords[2]))
        .collect();
    let distances: Vec<(Coord, Coord, f64)> = input
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (*a, *b, a.distance(b)))
        .sorted_by(|i1, i2| i1.2.total_cmp(&i2.2))
        .collect();
    distances
}

fn part1(input: &str, n: usize) -> usize {
    let distances = &get_distances(input)[..n];

    let mut nodes: HashMap<Coord, HashSet<Coord>> = HashMap::new();
    for (a, b, _) in distances {
        let set_a = nodes.entry(*a).or_default().clone();
        let set_b = nodes.entry(*b).or_default().clone();

        let mut union: HashSet<Coord> = set_a.union(&set_b).cloned().collect();
        union.insert(*a);
        union.insert(*b);

        for node in union.iter() {
            nodes.insert(*node, union.clone());
        }
    }

    let unique_sizes = nodes
        .values()
        .map(|set| {
            let mut v: Vec<_> = set.iter().cloned().collect();
            v.sort();
            (v, set.len())
        })
        .unique_by(|(v, _)| v.clone())
        .map(|(_, size)| size)
        .sorted_by(|a, b| b.cmp(a))
        .collect::<Vec<_>>();

    unique_sizes.iter().take(3).product()
}

fn part2(input: &str) -> usize {
    let distances = get_distances(input);

    let mut nodes: HashMap<Coord, HashSet<Coord>> = HashMap::new();
    let mut multiply = 0;
    for (a, b, _) in distances.iter() {
        let set_a = nodes.entry(*a).or_default().clone();
        let set_b = nodes.entry(*b).or_default().clone();
        if set_a.contains(b) {
            continue;
        }
        multiply = a.0 * b.0;

        let mut union: HashSet<Coord> = set_a.union(&set_b).cloned().collect();
        union.insert(*a);
        union.insert(*b);

        for node in union.iter() {
            nodes.insert(*node, union.clone());
        }
    }

    multiply
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_solution() {
        let input = indoc! {
            r#"
            162,817,812
            57,618,57
            906,360,560
            592,479,940
            352,342,300
            466,668,158
            542,29,236
            431,825,988
            739,650,466
            52,470,668
            216,146,977
            819,987,18
            117,168,530
            805,96,715
            346,949,466
            970,615,88
            941,993,340
            862,61,35
            984,92,344
            425,690,689
            "#
        };
        assert_eq!(part1(input, 10), 40);
        assert_eq!(part2(input), 25272);
    }
}
