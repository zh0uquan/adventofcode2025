use cached::proc_macro::cached;

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}


fn part1(input: &str) -> usize {
    input.lines()
        .map(|line| {
            let digits = line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<usize>>();
            let mut max = usize::MIN;
            for left in 0..digits.len() {
                for right in left + 1..digits.len() {
                    if digits[left] * 10 + digits[right] > max {
                        max = digits[left] * 10 + digits[right];
                    }
                }
            }
            max
        })
        .sum()
}

#[cached(
    key = "String",
    convert = r#"{ format!("{:?}-{}", digits, length) }"#
)]
fn search(digits: &[usize], length: usize) -> usize {
    if length == 1 {
        return *digits.iter().max().unwrap();
    }
    let right = digits.len() - length;
    let mut max = usize::MIN;
    for left in 0..=right {
        max = max.max(digits[left] * 10usize.pow((length - 1) as u32) + search(&digits[left+1..], length - 1));
    }
    max
}

fn part2(input: &str) -> usize {
    input.lines()
        .map(|line| {
            let digits = line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<usize>>();
            let max = search(&digits, 12);
            max
        })
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
            987654321111111
            811111111111119
            234234234234278
            818181911112111
            "#
        };
        assert_eq!(part1(input), 357);
    }


    #[test]
    fn test_part2() {
        let input = indoc! {
            r#"
            987654321111111
            811111111111119
            234234234234278
            818181911112111
            "#
        };
        assert_eq!(part2(input), 3121910778619);    }
}