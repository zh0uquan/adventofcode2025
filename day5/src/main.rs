use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", solution(input));
}

fn solution(input: &str) -> (usize, usize) {
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();
    let freshness = ranges
        .lines()
        .map(|line| line.split('-').collect_tuple().unwrap())
        .map(|(start, end)| {
            (
                start.parse::<usize>().unwrap(),
                end.parse::<usize>().unwrap(),
            )
        })
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .collect::<Vec<(usize, usize)>>();

    let mut merged = vec![freshness[0]];

    for (current_start, current_end) in freshness[1..].iter() {
        let (last_start, last_end) = merged.last().unwrap();

        if current_start <= last_end {
            let len = merged.len();
            merged[len - 1] = (*last_start, *current_end.max(last_end));
        } else {
            merged.push((*current_start, *current_end))
        }
    }

    let mut total = 0;
    for ingredient_str in ingredients.lines() {
        let ingredient = ingredient_str.parse::<usize>().unwrap();
        for (start, end) in merged.iter() {
            if ingredient >= *start && ingredient <= *end {
                total += 1;
                break;
            }
        }
    }
    let fresh_ids = merged
        .iter()
        .fold(0, |acc, range| acc + range.1 - range.0 + 1);
    (total, fresh_ids)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_solution() {
        let input = indoc! {
            r#"
            3-5
            10-14
            16-20
            12-18

            1
            5
            8
            11
            17
            32
            "#
        };
        assert_eq!(solution(input), (3, 14));
    }
}
