use good_lp::{
    default_solver, variable, variables, Expression, Solution, SolverModel,
};
use nom::{
    character::complete::{char as nom_char, digit1, one_of},
    combinator::map_res,
    multi::{many0, separated_list0},
    sequence::{delimited, preceded},
    IResult,
};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Parsed {
    mask: Vec<char>,
    groups: Vec<Vec<usize>>,
    values: Vec<usize>,
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

/* [.###.#] */
fn parse_mask(input: &str) -> IResult<&str, Vec<char>> {
    delimited(nom_char('['), many0(one_of(".#")), nom_char(']'))(input)
}

/* (0,1,2,3) */
fn parse_group(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(
        nom_char('('),
        separated_list0(nom_char(','), parse_usize),
        nom_char(')'),
    )(input)
}

/* {10,11,11,5,10,5} */
fn parse_values(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(
        nom_char('{'),
        separated_list0(nom_char(','), parse_usize),
        nom_char('}'),
    )(input)
}

/* Full line */
fn parse_line(input: &str) -> IResult<&str, Parsed> {
    let (input, mask) = parse_mask(input)?;
    let (input, groups) = many0(preceded(nom_char(' '), parse_group))(input)?;
    let (input, _) = nom_char(' ')(input)?;
    let (input, values) = parse_values(input)?;

    Ok((
        input,
        Parsed {
            mask,
            groups,
            values,
        },
    ))
}

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

fn dfs(dest: &usize, current: &usize, masks: &[usize]) -> Option<usize> {
    if current == dest {
        return Some(0);
    }
    if masks.is_empty() {
        return None;
    }
    let mut best: Option<usize> = None;
    for i in 0..masks.len() {
        let next = current ^ masks[i];
        if let Some(steps) = dfs(dest, &next, &masks[i + 1..]) {
            let candidate = steps + 1;
            best = Some(match best {
                Some(b) => b.min(candidate),
                None => candidate,
            });
        }
    }
    best
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (_, parsed) = parse_line(line).unwrap();
            let mask = parsed.mask;
            let groups = parsed.groups;
            let bits: usize =
                mask.iter().enumerate().fold(0, |acc, (i, c)| {
                    acc | match c {
                        '.' => 0,
                        '#' => 1 << i,
                        _ => unreachable!("disco!"),
                    }
                });
            let mask_groups = groups
                .iter()
                .map(|group| {
                    group.iter().fold(0, |acc, i| acc | (1 << i) as usize)
                })
                .collect::<Vec<usize>>();

            dfs(&bits, &0, &mask_groups).unwrap()
        })
        .sum()
}

pub fn solve_buttons(a: &[Vec<i32>], b: &[i32]) -> (i32, Vec<i32>) {
    let m = a.len();
    let n = a[0].len();

    let mut vars = variables!();
    let x = (0..m)
        .map(|_| vars.add(variable().integer().min(0)))
        .collect::<Vec<_>>();

    let obj: Expression = x.iter().copied().sum();
    let mut pb = vars.minimise(obj).using(default_solver);

    // A^T x = b
    for j in 0..n {
        let expr: Expression = (0..m).map(|i| (a[i][j] as f64) * x[i]).sum();
        pb = pb.with(expr.eq(b[j] as f64));
    }

    let sol = pb.solve().unwrap();
    let x_val = x.iter().map(|&v| sol.value(v) as i32).collect::<Vec<_>>();
    let presses = x_val.iter().sum();
    (presses, x_val)
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (_, parsed) = parse_line(line).unwrap();
            let mask = parsed.mask;
            let masks: Vec<Vec<i32>> = parsed
                .groups
                .iter()
                .map(|vec| {
                    (0..mask.len())
                        .map(|i| if vec.contains(&i) { 1 } else { 0 })
                        .collect()
                })
                .collect::<Vec<_>>();
            let values =
                parsed.values.iter().map(|&i| i as i32).collect::<Vec<_>>();
            let (sum, _) = solve_buttons(&masks, &values);
            sum as usize
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_solution() {
        let input = indoc! {
            r#"
            [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
            [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
            [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
            "#
        };
        assert_eq!(part1(input), 7);
        assert_eq!(part2(input), 33);
    }
}
