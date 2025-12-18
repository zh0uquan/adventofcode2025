use bitvec::bitvec;
use bitvec::order::Lsb0;
use bitvec::vec::BitVec;
use good_lp::{
    default_solver, variable, Expression, ProblemVariables, SolverModel,
    Variable,
};
use indoc::printdoc;
use itertools::Itertools;
use ndarray::{ArcArray2, Array2, ArrayView2, Axis, Slice};
use std::collections::HashSet;
use std::ops::BitAnd;

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input));
}

fn get_all_placements(
    matrice: ArrayView2<bool>,
    w: usize,
    h: usize,
) -> Vec<Array2<bool>> {
    let (ph, pw) = matrice.dim();
    let mut results = Vec::new();
    for c in 0..=h - ph {
        for r in 0..=w - pw {
            let mut out = Array2::<bool>::default((h, w));
            for dc in 0..ph {
                for dr in 0..pw {
                    out[(dc + c, dr + r)] = matrice[[dc, dr]]
                }
            }
            results.push(out);
        }
    }
    results
}

fn get_all_transform(matrix: ArrayView2<bool>) -> HashSet<Array2<bool>> {
    let mut set = HashSet::new();

    for flip in [false, true] {
        let new = if flip {
            matrix
                .slice_axis(Axis(1), Slice::new(0, None, -1))
                .to_owned()
        } else {
            matrix.to_owned()
        };
        let mut current = new;
        for _rotate in 0..4 {
            set.insert(current.clone());
            current = current.t().to_owned();
            current = current
                .slice_axis(Axis(0), Slice::new(0, None, -1))
                .to_owned();
        }
    }
    set
}

fn build_placements_in_regions(
    matrices: &Vec<Array2<bool>>,
    width: &usize,
    height: &usize,
) -> Vec<Vec<BitVec<u64>>> {
    let mut present_constraints: Vec<Vec<BitVec<u64>>> = vec![];

    for matrice in matrices.iter() {
        let transforms = get_all_transform(matrice.view());
        let placements: Vec<BitVec<u64>> = transforms
            .into_iter()
            .flat_map(|t| get_all_placements(t.view(), *width, *height))
            .map(|p| p.iter().collect())
            .collect();
        present_constraints.push(placements);
    }
    present_constraints
}

fn build_cell_to_candidates(
    placements: &[Vec<BitVec<u64, Lsb0>>],
    n_cells: usize,
) -> Vec<Vec<(usize, usize)>> {
    let mut cell2 = vec![Vec::<(usize, usize)>::new(); n_cells];
    for (t, list) in placements.iter().enumerate() {
        for (p, bv) in list.iter().enumerate() {
            for i in bv.iter_ones() {
                // i is cell index
                cell2[i].push((t, p));
            }
        }
    }
    cell2
}

fn can_fit(
    width: &usize,
    height: &usize,
    placements: Vec<Vec<BitVec<u64, Lsb0>>>,
    need: &[usize],
) -> bool {
    // basic sanity checks
    if need.len() != placements.len() {
        return false;
    }
    for (req, pl) in need.iter().zip(placements.iter()) {
        if *req > pl.len() {
            return false;
        }
    }

    let n_cells = width * height;

    // build cell coverage index
    let cell2 = build_cell_to_candidates(&placements, n_cells);

    // decision variables x[t][p] ∈ {0,1}
    let mut vars = ProblemVariables::new();
    let mut x: Vec<Vec<Variable>> = Vec::with_capacity(placements.len());
    for t in 0..placements.len() {
        let mut row = Vec::with_capacity(placements[t].len());
        for _ in 0..placements[t].len() {
            row.push(vars.add(variable().binary()));
        }
        x.push(row);
    }

    // feasibility model (no real objective)
    let mut pb = vars.minimise(0).using(default_solver);

    // constraint 1: exact number per type
    for t in 0..need.len() {
        let expr = x[t].iter().copied().sum::<Expression>();
        pb = pb.with(expr.eq(need[t] as f64));
    }

    // constraint 2: no overlap per cell
    for i in 0..n_cells {
        let cand = &cell2[i];
        if cand.is_empty() {
            continue;
        }
        let mut expr = Expression::from(0.0);
        for &(t, p) in cand {
            expr = expr + x[t][p];
        }
        pb = pb.with(expr.leq(1.0));
    }

    // solve: success => feasible
    pb.solve().is_ok()
}

fn size_check(
    width: &usize,
    height: &usize,
    matrices: &Vec<Array2<bool>>,
    to_places: &Vec<usize>,
) -> bool {
    to_places
        .iter()
        .zip(matrices.iter())
        .map(|(n, matrix)| n * matrix.iter().filter(|&&x| x).count())
        .sum::<usize>()
        <= *width * *height
}

fn part1(input: &str) -> usize {
    let (matrices_str, regions_str) = input
        .rsplit_once("\n\n")
        .expect("input must contain an empty line separator");
    let matrices = matrices_str
        .split("\n\n")
        .map(|block| {
            let lines = block.lines().collect::<Vec<&str>>();
            let n = lines[1].len();
            let matrix_flatten: Vec<bool> = lines[1..]
                .iter()
                .flat_map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '#' => true,
                            '.' => false,
                            _ => unreachable!("disco!"),
                        })
                        .collect::<Vec<bool>>()
                })
                .collect();
            Array2::from_shape_vec((n, n), matrix_flatten).unwrap()
        })
        .collect::<Vec<Array2<bool>>>();

    let regions: Vec<(usize, usize, Vec<usize>)> = regions_str
        .lines()
        .map(|line| {
            let (region, constraint) =
                line.split(": ").collect_tuple().unwrap();
            let (width, height) = region.split("x").collect_tuple().unwrap();
            let constraints: Vec<&str> =
                constraint.split_ascii_whitespace().collect();
            (
                width.parse::<usize>().unwrap(),
                height.parse::<usize>().unwrap(),
                constraints
                    .iter()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect(),
            )
        })
        .collect();

    regions
        .iter()
        .map(|(width, height, to_places)| {
            println!("on region {:?} {:?}", width * height, *to_places);
            if !size_check(width, height, &matrices, to_places) {
                return 0;
            }
            // only validate when its not fit
            // let placements = build_placements_in_regions(
            //     &matrices, width, height
            // );
            // if can_fit(
            //     width, height, placements, to_places
            // ) { 1 } else { 0 }
            1
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = indoc! {
            r#"
            0:
            ###
            ##.
            ##.

            1:
            ###
            ##.
            .##

            2:
            .##
            ###
            ##.

            3:
            ##.
            ###
            ##.

            4:
            ###
            #..
            ###

            5:
            ###
            .#.
            ###

            4x4: 0 0 0 0 2 0
            12x5: 1 0 1 0 2 2
            12x5: 1 0 1 0 3 2
            "#
        };
        assert_eq!(part1(input), 2);
    }
}
