use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}


fn dfs(
    current: &str,
    end: &str,
    paths: &HashMap<&str, Vec<&str>>,
) -> usize{
    if current == end {
        return 1;
    }
    let mut total = 0;
    for next in paths.get(current).unwrap() {
        total += dfs(next, end, paths)
    }
    total
}


fn dfs_with_memo(
    current: &str,
    end: &str,
    graph: &HashMap<&str, Vec<&str>>,
    visited: &mut HashSet<String>,
    has_fft: bool,
    has_dac: bool,
    memo: &mut HashMap<(String, bool, bool, Vec<String>), usize>,
) -> usize {
    if current == end {
        return if has_fft && has_dac { 1 } else { 0 };
    }

    let mut visited_vec: Vec<_> = visited.iter().map(|s| s.to_string()).collect();
    visited_vec.sort_unstable();
    let key = (current.to_string(), has_fft, has_dac, visited_vec);

    if let Some(&result) = memo.get(&key) {
        return result;
    }

    let Some(neighbors) = graph.get(current) else {
        return 0;
    };

    let mut path_count = 0;
    for &neighbor in neighbors {
        if visited.contains(neighbor) {
            continue;
        }
        visited.insert(neighbor.into());
        let new_has_fft = has_fft || neighbor == "fft";
        let new_has_dac = has_dac || neighbor == "dac";
        path_count += dfs_with_memo(neighbor, end, graph, visited, new_has_fft, new_has_dac, memo);
        visited.remove(neighbor);
    }

    memo.insert(key, path_count);
    path_count
}
fn get_paths(input: &str) -> HashMap<&str, Vec<&str>> {
    input.lines().map(
        |line| {
            let (from, gotos) = line.split(": ").collect_tuple().unwrap();
            (from, gotos.split_ascii_whitespace().collect())
        }
    ).collect()
}


fn part1(input: &str) -> usize {
    let paths = get_paths(input);
    dfs("you", "out", &paths)
}

fn part2(input: &str) -> usize {
    let paths = get_paths(input);
    let mut visited = HashSet::new();
    let mut memo = HashMap::new();
    dfs_with_memo(
        "svr", 
        "out", 
        &paths, 
        &mut visited,
        false,
        false,
        &mut memo,
    )
}


#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = indoc! {
            r#"
            aaa: you hhh
            you: bbb ccc
            bbb: ddd eee
            ccc: ddd eee fff
            ddd: ggg
            eee: out
            fff: out
            ggg: out
            hhh: ccc fff iii
            iii: out
            "#
        };
        assert_eq!(part1(input), 5);
    }
    
    #[test]
    fn test_part2() {
        let input = indoc! {
            r#"
            svr: aaa bbb
            aaa: fft
            fft: ccc
            bbb: tty
            tty: ccc
            ccc: ddd eee
            ddd: hub
            hub: fff
            eee: dac
            dac: fff
            fff: ggg hhh
            ggg: out
            hhh: out
            "#
        };
        assert_eq!(part2(input), 2);
    }
}