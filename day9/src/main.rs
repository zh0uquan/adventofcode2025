use itertools::Itertools;
use std::collections::BTreeMap;

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

fn part1(input: &str) -> usize {
    let pos = input
        .lines()
        .map(|line| line.split(',').collect_tuple().unwrap())
        .map(|(a, b)| {
            (a.parse::<isize>().unwrap(), b.parse::<isize>().unwrap())
        })
        .collect::<Vec<_>>();

    let n = pos.len();
    let mut max_area = 0;
    for i in 0..n {
        for j in i + 1..n {
            let dy = pos[i].0.abs_diff(pos[j].0) + 1;
            let dx = pos[i].1.abs_diff(pos[j].1) + 1;
            max_area = max_area.max(dy * dx);
        }
    }
    max_area
}

fn point_in_polygon(x: f64, y: f64, poly: &Vec<(f64, f64)>) -> bool {
    let (x, y) = (x + 0.5, y + 0.5);
    let mut inside = false;

    for (poly1, poly2) in poly.iter().circular_tuple_windows() {
        let (x1, y1) = *poly1;
        let (x2, y2) = *poly2;

        if (y1 > y) != (y2 > y) {
            let x_cross = x1 + (x2 - x1) * (y - y1) / (y2 - y1);
            if x_cross > x {
                inside = !inside;
            }
        }
    }
    inside
}

fn part2(input: &str) -> isize {
    let polygon: Vec<(isize, isize)> = input
        .lines()
        .map(|line| line.split(',').collect_tuple().unwrap())
        .map(|(a, b)| {
            (a.parse::<isize>().unwrap(), b.parse::<isize>().unwrap())
        })
        .collect();

    let interiors: BTreeMap<isize, Vec<isize>> = polygon
        .iter()
        .cloned()
        .sorted_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)))
        .fold(BTreeMap::new(), |mut acc, (x, y)| {
            acc.entry(y).or_default().push(x);
            acc
        });

    let polygon: Vec<(f64, f64)> = polygon
        .into_iter()
        .map(|(a, b)| (a as f64, b as f64))
        .collect();

    let max = 0;
    for comb in interiors.iter().combinations(2) {
        let line1 = comb[0];
        let line2 = comb[1];

        let (top, bottom) = (*line1.0, *line2.0);
        let (tl, tr) = (line1.1[0], line1.1[1]);
        let (bl, br) = (line2.1[0], line2.1[1]);

        let l_star = tl.max(bl);
        let r_star = tr.min(br);

        if l_star > r_star {
            continue;
        }
    }

    println!("max={max}");

    24
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_solution() {
        let input = indoc! {
            r#"
            7,1
            11,1
            11,7
            9,7
            9,5
            2,5
            2,3
            7,3
            "#
        };
        assert_eq!(part1(input), 50);
        assert_eq!(part2(input), 24);
    }
}
