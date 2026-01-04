use itertools::Itertools;

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

fn get_size(x1: isize, y1: isize, x2: isize, y2: isize) -> isize {
    ((x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1)) as isize
}

fn part2(input: &str) -> isize {
    let corners: Vec<(isize, isize)> = input
        .lines()
        .map(|line| line.split(',').collect_tuple().unwrap())
        .map(|(a, b)| {
            (a.parse::<isize>().unwrap(), b.parse::<isize>().unwrap())
        })
        .collect();

    let mut edges = vec![];

    let n = corners.len();
    let mut sizes: Vec<(isize, (isize, isize), (isize, isize))> =
        Vec::with_capacity(n * (n - 1) / 2);

    for i in 0..n {
        // edge i: corners[i] -> corners[i-1]
        let a = corners[i];
        let b = corners[(i + n - 1) % n];
        let e = if a <= b { (a, b) } else { (b, a) };
        edges.push(e);

        // all pairs i<j
        for j in (i + 1)..n {
            let a = corners[i];
            let b = corners[j];
            let (c1, c2) = if a <= b { (a, b) } else { (b, a) };
            let area = get_size(c1.0, c1.1, c2.0, c2.1);
            sizes.push((area, c1, c2));
        }
    }

    edges.sort_by(|(a1, a2), (b1, b2)| {
        let sa = get_size(a1.0, a1.1, a2.0, a2.1);
        let sb = get_size(b1.0, b1.1, b2.0, b2.1);
        sb.cmp(&sa) // descending
    });

    sizes.sort_by(|(sa, a1, a2), (sb, b1, b2)| {
        sb.cmp(sa).then_with(|| b1.cmp(a1)).then_with(|| b2.cmp(a2))
    });

    for (area, (x1, y1), (x2, y2)) in sizes {
        let mut y_lo = y1;
        let mut y_hi = y2;
        if y_lo > y_hi {
            std::mem::swap(&mut y_lo, &mut y_hi);
        }

        let mut blocked = false;
        for &((x3, y3), (x4, y4)) in &edges {
            // same as python:
            // (x4 > x1 and x3 < x2 and y4 > y1 and y3 < y2)
            if x4 > x1 && x3 < x2 && y4 > y_lo && y3 < y_hi {
                blocked = true;
                break;
            }
        }

        if !blocked {
            return area;
        }
    }

    0
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
