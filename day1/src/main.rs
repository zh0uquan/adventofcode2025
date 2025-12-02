const START_POINT: i32 = 50;
const MOD: i32 = 100;

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

fn part1(input: &str) -> i32 {
    let mut pos = START_POINT;
    let mut count = 0;

    for line in input.lines() {
        let (dir_str, n_str) = line.split_at(1);
        let steps: i32 = n_str.parse().unwrap_or(0);

        let step = match dir_str {
            "L" => -steps,
            "R" =>  steps,
            _ => unreachable!("invalid direction"),
        };
            
        pos = (pos + step).rem_euclid(MOD);
        if pos == 0 {
            count += 1;
        }
    }

    count
}


fn part2(input: &str) -> i16 {
    let mut pos = START_POINT;
    let mut count = 0;

    for line in input.lines() {
        let (dir_str, n_str) = line.split_at(1);
        let steps: i32 = n_str.parse().unwrap_or(0);

        let step = match dir_str {
            "L" => -1,
            "R" =>  1,
            _ => unreachable!("invalid direction"),
        };

        for _ in 0..steps {
            pos = (pos + step).rem_euclid(MOD);
            if pos == 0 {
                count += 1;
            }
        }
    }

    count
}


#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = indoc! {
            r#"
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82
            "#
        };
        assert_eq!(3, part1(input));
    }


    #[test]
    fn test_part2() { 
        let input = indoc! {
            r#"
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82
            "#
        };
        assert_eq!(6, part2(input));
    }
}