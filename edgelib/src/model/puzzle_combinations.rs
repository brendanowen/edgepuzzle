use num_bigint::BigUint;
use num_traits::ToPrimitive;

use crate::model::PuzzleStructure;

pub struct PuzzleCombinations {
    pub middle_probablity: Vec<(BigUint, BigUint)>,
    pub border_probablity: Vec<(BigUint, BigUint)>,
    pub corner_combinations: Vec<BigUint>,
    pub edge_combinations: Vec<BigUint>,
    pub interior_combinations: Vec<BigUint>,
    pub log10_middle: Vec<f64>,
    pub log10_border: Vec<f64>,
    pub log10_corner: Vec<f64>,
    pub log10_edge: Vec<f64>,
    pub log10_interior: Vec<f64>,
}

impl PuzzleCombinations {
    pub fn new(puzzle_structure: &PuzzleStructure) -> Self {
        let size1: usize = puzzle_structure.border_join_counts.iter().sum();
        let size2: usize = puzzle_structure.middle_join_counts.iter().sum::<usize>() * 2;
        let size3: usize = puzzle_structure.corners;
        let size4: usize = puzzle_structure.edges;
        let size5: usize = puzzle_structure.interiors;
        let max_size: usize = *(vec![size1, size2, size3, size4, size5]
            .iter()
            .max()
            .unwrap());
        let f: Vec<BigUint> = generate_factorials(max_size);
        let p: Vec<Vec<BigUint>> = generate_permutions(&f);
        let c: Vec<Vec<BigUint>> = generate_combinations(&f);

        let middle_probablity: Vec<(BigUint, BigUint)> =
            middle_joins(&puzzle_structure.middle_join_counts, &p, &c);
        let border_probablity: Vec<(BigUint, BigUint)> =
            border_joins(&puzzle_structure.border_join_counts, &p, &c);
        let corner_combinations: Vec<BigUint> = p[puzzle_structure.corners].clone();
        let edge_combinations: Vec<BigUint> = p[puzzle_structure.edges].clone();
        let interior_combinations: Vec<BigUint> = p[puzzle_structure.interiors]
            .iter()
            .enumerate()
            .map(|(index, combinations)| {
                combinations.clone() * BigUint::from(4usize).pow(index as u32)
            })
            .collect();

        let log10_middle: Vec<f64> = convert_ratio(&middle_probablity);
        let log10_border: Vec<f64> = convert_ratio(&border_probablity);
        let log10_corner: Vec<f64> = convert_value(&corner_combinations);
        let log10_edge: Vec<f64> = convert_value(&edge_combinations);
        let log10_interior: Vec<f64> = convert_value(&interior_combinations);

        PuzzleCombinations {
            middle_probablity,
            border_probablity,
            corner_combinations,
            edge_combinations,
            interior_combinations,
            log10_middle,
            log10_border,
            log10_corner,
            log10_edge,
            log10_interior,
        }
    }
}

fn approximate_log10(n: &BigUint) -> f64 {
    // 1. Find the number of digits.
    let digit_count = n.to_string().len();

    // 2. Reduce the number of digits if it's too large.  We want to work
    // with a smaller number that can be accurately represented by f64.
    let reduced_n = if digit_count > 18 {
        // Reduce to a number with at most 18 digits (f64 can precisely
        // represent integers up to around 2^53, which is about 16 decimal digits,
        // but we give a bit of extra room).  This reduction is done by
        // dividing by 10^(number of digits - 18).
        let reduction_factor = BigUint::from(10_u32).pow((digit_count - 18) as u32);
        (n / reduction_factor).to_f64()
    } else {
        //if the digit count is already small enough, just convert it to f64
        n.to_f64()
    }
    .unwrap();

    // 3. Calculate the approximate log10 of the reduced number.
    let approx_log10 = reduced_n.log10();

    // 4. Add the number of digits back to the result, adjusting for the reduction.
    if digit_count > 18 {
        approx_log10 + (digit_count - 18) as f64
    } else {
        approx_log10
    }
}

fn convert_value(list: &[BigUint]) -> Vec<f64> {
    list.iter().map(|item| approximate_log10(item)).collect()
}

fn convert_ratio(list_duals: &[(BigUint, BigUint)]) -> Vec<f64> {
    list_duals
        .iter()
        .map(|item| approximate_log10(&item.0) - approximate_log10(&item.1))
        .collect()
}

fn middle_joins(
    join_counts: &Vec<usize>,
    p: &Vec<Vec<BigUint>>,
    c: &Vec<Vec<BigUint>>,
) -> Vec<(BigUint, BigUint)> {
    let join_types: usize = join_counts.len();
    let total_joints: usize = join_counts.iter().sum();

    let mut valid_combinations: Vec<Vec<BigUint>> =
        vec![vec![BigUint::ZERO; total_joints + 1]; join_types + 1];
    valid_combinations[0][0] = BigUint::from(1usize);

    (0..=total_joints).for_each(|m| {
        (1..=join_types).for_each(|i| {
            valid_combinations[i][m] = (0..=join_counts[i - 1])
                .map(|j| {
                    if j <= m {
                        valid_combinations[i - 1][m - j].clone()
                            * p[2 * join_counts[i - 1]][2 * j].clone()
                            * c[m][j].clone()
                    } else {
                        BigUint::ZERO
                    }
                })
                .sum::<BigUint>();
        });
    });

    (0..=total_joints)
        .map(|m| {
            (
                valid_combinations[join_types][m].clone(),
                p[2 * total_joints][2 * m].clone(),
            )
        })
        .collect()
}

fn border_joins(
    join_counts: &Vec<usize>,
    p: &Vec<Vec<BigUint>>,
    c: &Vec<Vec<BigUint>>,
) -> Vec<(BigUint, BigUint)> {
    let join_types: usize = join_counts.len();
    let total_joints: usize = join_counts.iter().sum();

    let mut valid_combinations: Vec<Vec<BigUint>> =
        vec![vec![BigUint::ZERO; total_joints + 1]; join_types + 1];
    valid_combinations[0][0] = BigUint::from(1usize);

    (0..=total_joints).for_each(|b| {
        (1..=join_types).for_each(|i| {
            valid_combinations[i][b] = (0..=join_counts[i - 1])
                .map(|j| {
                    if j <= b {
                        valid_combinations[i - 1][b - j].clone()
                            * p[join_counts[i - 1]][j].clone()
                            * p[join_counts[i - 1]][j].clone()
                            * c[b][j].clone()
                    } else {
                        BigUint::ZERO
                    }
                })
                .sum::<BigUint>();
        });
    });

    (0..=total_joints)
        .map(|b| {
            (
                valid_combinations[join_types][b].clone(),
                p[total_joints][b].clone() * p[total_joints][b].clone(),
            )
        })
        .collect()
}

fn generate_factorials(max_value: usize) -> Vec<BigUint> {
    let mut factorial: Vec<BigUint> = vec![BigUint::ZERO; max_value + 1];
    factorial[0] = BigUint::from(1usize);
    (1..=max_value).for_each(|index| {
        factorial[index] = factorial[index - 1].clone() * BigUint::from(index);
    });

    factorial
}

fn generate_permutions(factorial: &Vec<BigUint>) -> Vec<Vec<BigUint>> {
    let max_value = factorial.len() - 1;
    (0..=max_value)
        .map(|n| {
            (0..=n)
                .map(|k| factorial[n].clone() / factorial[n - k].clone())
                .collect()
        })
        .collect()
}

fn generate_combinations(factorial: &Vec<BigUint>) -> Vec<Vec<BigUint>> {
    let max_value = factorial.len() - 1;
    (0..=max_value)
        .map(|n| {
            (0..=n)
                .map(|k| factorial[n].clone() / factorial[n - k].clone() / factorial[k].clone())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_border_joins() {}
}
