use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

fn dfs(current: &str, end: &str, paths: &HashMap<&str, Vec<&str>>) -> usize {
    if current == end {
        return 1;
    }
    let mut total = 0;
    for next in paths.get(current).unwrap() {
        total += dfs(next, end, paths)
    }
    total
}

fn get_or_insert(
    node_ids: &mut HashMap<String, usize>,
    adjs: &mut Vec<Vec<usize>>,
    name: &str,
) -> usize {
    if let Some(&i) = node_ids.get(name) {
        return i;
    }
    let id = adjs.len();
    node_ids.insert(name.to_string(), id);
    adjs.push(Vec::new());
    id
}

fn get_paths(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|line| {
            let (from, gotos) = line.split(": ").collect_tuple().unwrap();
            (from, gotos.split_ascii_whitespace().collect())
        })
        .collect()
}

fn count_paths_dp_mask(
    adj: &Vec<Vec<usize>>,
    svr: usize,
    out: usize,
    dac: usize,
    fft: usize,
) -> usize {
    let n = adj.len();
    let mut memo: Vec<[Option<usize>; 4]> = vec![[None; 4]; n];

    fn bit(u: usize, dac: usize, fft: usize) -> u8 {
        let mut m = 0u8;
        if u == dac {
            m |= 1;
        }
        if u == fft {
            m |= 2;
        }
        m
    }

    fn dfs(
        u: usize,
        out: usize,
        dac: usize,
        fft: usize,
        mask: u8,
        adj: &Vec<Vec<usize>>,
        memo: &mut Vec<[Option<usize>; 4]>,
    ) -> usize {
        let mi = mask as usize;
        if let Some(ans) = memo[u][mi] {
            return ans;
        }

        let ans = if u == out {
            if mask == 3 {
                1
            } else {
                0
            }
        } else {
            let mut total = 0;
            for &v in &adj[u] {
                total +=
                    dfs(v, out, dac, fft, mask | bit(v, dac, fft), adj, memo);
            }
            total
        };

        memo[u][mi] = Some(ans);
        ans
    }

    dfs(svr, out, dac, fft, bit(svr, dac, fft), adj, &mut memo)
}

fn part1(input: &str) -> usize {
    let paths = get_paths(input);
    dfs("you", "out", &paths)
}

fn part2(input: &str) -> usize {
    let paths = get_paths(input);
    let mut node_ids: HashMap<String, usize> = HashMap::new();
    let mut adjs: Vec<Vec<usize>> = Vec::new();

    for (name, neighbours) in paths {
        let id = get_or_insert(&mut node_ids, &mut adjs, name);
        for neighbour in neighbours {
            let neighbour_id =
                get_or_insert(&mut node_ids, &mut adjs, neighbour);
            adjs[id].push(neighbour_id);
        }
    }
    let svr = node_ids.get("svr").unwrap();
    let out = node_ids.get("out").unwrap();
    let fft = node_ids.get("fft").unwrap();
    let dac = node_ids.get("dac").unwrap();

    count_paths_dp_mask(&adjs, *svr, *out, *fft, *dac)
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
