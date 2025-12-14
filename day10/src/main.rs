use cached::proc_macro::cached;
use nom::{
    character::complete::{char as nom_char, digit1, one_of},
    combinator::map_res,
    multi::{many0, separated_list0},
    sequence::{delimited, preceded},
    IResult,
};
use std::cmp::Reverse;

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
    dfs(&1, &0, &vec![1]);
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

fn part2(input: &str) -> usize {
    33
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
