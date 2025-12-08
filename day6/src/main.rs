use itertools::{enumerate, Itertools};

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut input: Vec<&str> = input.lines().collect();
    let signs: Vec<&str> =
        input.pop().unwrap().split_ascii_whitespace().collect();

    let inits: Vec<usize> = signs
        .iter()
        .map(|sig| match *sig {
            "*" => 1,
            "+" => 0,
            _ => panic!("disco!"),
        })
        .collect();

    input
        .iter()
        .map(|line| line.split_ascii_whitespace().collect())
        .map(|nums_str: Vec<&str>| {
            nums_str
                .iter()
                .map(|num_str| num_str.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .fold(inits, |acc, nums| {
            acc.into_iter()
                .zip(nums.into_iter())
                .enumerate()
                .map(|(idx, (a, b))| match signs[idx] {
                    "*" => a * b,
                    "+" => a + b,
                    _ => panic!("disco!"),
                })
                .collect()
        })
        .iter()
        .sum()
}

fn part2(input: &str) -> usize {
    let mut input: Vec<&str> = input.lines().collect();
    let signs: Vec<&str> =
        input.pop().unwrap().split_ascii_whitespace().collect();
    let inits: Vec<usize> = signs
        .iter()
        .map(|sig| match *sig {
            "*" => 1,
            "+" => 0,
            _ => panic!("disco!"),
        })
        .collect();

    let filled_signs: Vec<char> = input
        .iter()
        .map(|line| line.split_ascii_whitespace().collect())
        .fold(vec![0; inits.len()], |mut acc, nums_str: Vec<&str>| {
            for (i, num_str) in nums_str.iter().enumerate() {
                if num_str.len() > acc[i] {
                    acc[i] = num_str.len();
                }
            }
            acc
        })
        .iter()
        .enumerate()
        .fold(String::new(), |mut acc, (index, gap)| {
            acc.push_str(&*String::from(signs[index].repeat(*gap)));
            acc.push_str(" ");
            acc
        })
        .chars()
        .collect();

    let input = input
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut total = 0;
    let mut acc: Option<usize> = None;
    for (j, &op) in filled_signs.iter().enumerate() {
        if op == ' ' {
            if let Some(value) = acc.take() {
                total += value;
            }
            continue;
        }

        if acc.is_none() {
            acc = Some(match op {
                '+' => 0,
                '*' => 1,
                _ => panic!("unexpected operator"),
            });
        }

        let n = (0..input.len())
            .fold(String::new(), |mut acc, i| {
                acc.push(*input[i].get(j).unwrap_or(&' '));
                acc
            })
            .trim()
            .parse::<usize>()
            .unwrap();

        acc = Some(match op {
            '+' => acc.unwrap() + n,
            '*' => acc.unwrap() * n,
            _ => unreachable!(),
        });
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = indoc! {
            r#"
            123 328  51 64 
             45 64  387 23 
              6 98  215 314
            *   +   *   +  
            "#
        };
        assert_eq!(part1(input), 4277556);
    }

    #[test]
    fn test_part2() {
        let input = indoc! {
            r#"
            123 328  51 64
             45 64  387 23
              6 98  215 314
            *   +   *   +   
            "#
        };
        assert_eq!(part2(input), 3263827);
    }
}
