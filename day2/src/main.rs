use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}




fn part1(input: &str) -> usize {
    input
        .trim()
        .split(',')
        .map(|range| range.split('-').collect_tuple().unwrap())
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        .map(|(start, end)| 
            (start..=end).map(|n| {
                let n_str = n.to_string();
                let (left, right) = n_str.split_at(n_str.len() / 2);
                if left == right {
                    return n;
                }
                0
            }).sum::<usize>()
        )
        .sum()
}

fn is_invalid(n: usize) -> bool {
    let n_str = n.to_string();
    for i in 1..=n_str.len() / 2 {
        if (n_str.len()) % i != 0 {
            continue;
        }
        let same = n_str
            .chars()
            .collect::<Vec<char>>()
            .chunks(i)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>()
            .windows(2)
            .all(|window| window[0] == window[1]);
        if same {
            return true
        }
    }
    false
}


fn part2(input: &str) -> usize {
        input
        .trim()
        .split(',')
        .map(|range| range.split('-').collect_tuple().unwrap())
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        .map(|(start, end)| 
            (start..=end).map(|n| {
                if is_invalid(n) {
                    return n;
                }
                0
            }).sum::<usize>()
        )
        .sum()
    
}


#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = indoc! {
            r#"
            11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
            "#
        };
        assert_eq!(part1(input), 1227775554);
    }


    #[test]
    fn test_part2() {
        let input = indoc! {
            r#"
            11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
            "#
        };
        assert_eq!(part2(input), 4174379265);
    }
}