use common::Matrix;

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

fn part1(input: &str) -> usize {
    let matrix: Matrix<char> = Matrix::from(input, |c| c);
    let mut total = 0;
    for i in 0..matrix.height {
        for j in 0..matrix.width {
            if matrix[(i, j)] != '@' {
                continue;
            }
            let num: usize = matrix
                .get_adjacent_neighbours((i, j))
                .iter()
                .map(|coord| matrix[*coord])
                .filter(|c| *c == '@')
                .count();
            if num < 4 {
                total += 1;
            }
        }
    }
    total
}

fn transit(matrix: &Matrix<char>) -> Matrix<char> {
    let mut next_matrix = matrix.clone();
    for i in 0..matrix.height {
        for j in 0..matrix.width {
            if matrix[(i, j)] != '@' {
                continue;
            }
            let num: usize = matrix
                .get_adjacent_neighbours((i, j))
                .iter()
                .map(|coord| matrix[*coord])
                .filter(|c| *c == '@')
                .count();
            if num < 4 {
                next_matrix[(i, j)] = '.';
            }
        }
    }
    next_matrix
}

fn part2(input: &str) -> usize {
    let init_matrix = Matrix::from(input, |c| c);
    let mut matrix: Matrix<char> = init_matrix.clone();
    let mut next_matrix: Matrix<char> = transit(&matrix);
    while matrix != next_matrix {
        matrix = next_matrix;
        next_matrix = transit(&matrix);
    }
    let (mut original, mut now) = (0, 0);
    for i in 0..matrix.height {
        for j in 0..matrix.width {
            if init_matrix[(i, j)] == '@' {
                original += 1;
            }
            if next_matrix[(i, j)] == '@' {
                now += 1;
            }
        }
    }
    original - now
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = indoc! {
            r#"
            ..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@.
            "#
        };
        assert_eq!(part1(input), 13);
    }

    #[test]
    fn test_part2() {
        let input = indoc! {
            r#"
            ..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@.
            "#
        };
        assert_eq!(part2(input), 43);
    }
}
