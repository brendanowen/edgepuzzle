use num_bigint::BigUint;
use num_integer::gcd;

use crate::model::PuzzleStructure;

pub struct PuzzleCombinations {
    pub middle_probablity: Vec<(BigUint, BigUint)>,
    pub border_probablity: Vec<(BigUint, BigUint)>,
    pub corner_combinations: Vec<BigUint>,
    pub edge_combinations: Vec<BigUint>,
    pub interior_combinations: Vec<BigUint>,
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

        PuzzleCombinations {
            middle_probablity,
            border_probablity,
            corner_combinations,
            edge_combinations,
            interior_combinations,
        }
    }
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
            reduce_fraction(
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
            reduce_fraction(
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

fn reduce_fraction(numerator: BigUint, denominator: BigUint) -> (BigUint, BigUint) {
    if denominator == BigUint::ZERO {
        return (numerator, denominator); // Handle division by zero
    }

    let common_divisor = gcd(numerator.clone(), denominator.clone()); // Use clone to avoid ownership issues
    let reduced_numerator = numerator / common_divisor.clone();
    let reduced_denominator = denominator / common_divisor;

    (reduced_numerator, reduced_denominator)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_border_joins() {}
}
