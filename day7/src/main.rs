use cached::proc_macro::cached;
use common::{Coord, Inbound, Matrix};
use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", solution(input));
}

fn dfs(
    pos: Coord,
    visited: &mut HashSet<Coord>,
    matrix: &Matrix<char>,
) -> usize {
    let next_pos = (pos.0 + 1, pos.1);
    if visited.contains(&next_pos) || !next_pos.inbound(&matrix) {
        return 0;
    }
    match matrix[next_pos] {
        '.' => {
            visited.insert(next_pos);
            dfs(next_pos, visited, matrix)
        }
        '^' => {
            let mut total = 1;
            for pos in [(pos.0 + 1, pos.1 - 1), (pos.0 + 1, pos.1 + 1)] {
                visited.insert(pos);
                total += dfs(pos, visited, matrix);
            }
            total
        }
        _ => panic!("disco!"),
    }
}

#[cached(key = "String", convert = r#"{ format!("{:?}", pos) }"#)]
fn dfsv2(pos: Coord, matrix: &Matrix<char>) -> usize {
    let next_pos = (pos.0 + 1, pos.1);
    let mut total = 0;
    if next_pos.0 == matrix.height && next_pos.1 < matrix.width {
        return 1;
    }
    match matrix[next_pos] {
        '.' => dfsv2(next_pos, matrix),
        '^' => {
            for pos in [(pos.0 + 1, pos.1 - 1), (pos.0 + 1, pos.1 + 1)] {
                total += dfsv2(pos, matrix);
            }
            total
        }
        _ => panic!("disco!"),
    }
}

fn solution(input: &str) -> (usize, usize) {
    let matrix = Matrix::from(input, |c| c);
    let start = matrix.find(&'S').unwrap();
    let mut visited: HashSet<Coord> = HashSet::new();
    let total = dfs(start, &mut visited, &matrix);
    let total_v2 = dfsv2(start, &matrix);
    (total, total_v2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_solution() {
        let input = indoc! {
            r#"
            .......S.......
            ...............
            .......^.......
            ...............
            ......^.^......
            ...............
            .....^.^.^.....
            ...............
            ....^.^...^....
            ...............
            ...^.^...^.^...
            ...............
            ..^...^.....^..
            ...............
            .^.^.^.^.^...^.
            ...............
            "#
        };
        assert_eq!(solution(input), (21, 40));
    }
}
