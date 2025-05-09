use crate::model::PuzzleStructure;

pub struct FastCombinations {
    pub log10_middle: Vec<f64>,
    pub log10_border: Vec<f64>,
    pub log10_corner: Vec<f64>,
    pub log10_edge: Vec<f64>,
    pub log10_interior: Vec<f64>,
}

impl FastCombinations {
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
        let f: Vec<f64> = generate_factorials(max_size);
        let p: Vec<Vec<f64>> = generate_permutions(&f);
        let c: Vec<Vec<f64>> = generate_combinations(&f);

        let log10_middle: Vec<f64> = middle_joins(&puzzle_structure.middle_join_counts, &p, &c);
        let log10_border: Vec<f64> = border_joins(&puzzle_structure.border_join_counts, &p, &c);
        let log10_corner: Vec<f64> = p[puzzle_structure.corners].clone();
        let log10_edge: Vec<f64> = p[puzzle_structure.edges].clone();
        let log10_interior: Vec<f64> = p[puzzle_structure.interiors]
            .iter()
            .enumerate()
            .map(|(index, combinations)| combinations + 4f64.log10() * (index as f64))
            .collect();

        FastCombinations {
            log10_middle,
            log10_border,
            log10_corner,
            log10_edge,
            log10_interior,
        }
    }
}

fn middle_joins(join_counts: &Vec<usize>, p: &Vec<Vec<f64>>, c: &Vec<Vec<f64>>) -> Vec<f64> {
    let join_types: usize = join_counts.len();
    let total_joints: usize = join_counts.iter().sum();

    let mut valid_combinations: Vec<Vec<f64>> =
        vec![vec![f64::NEG_INFINITY; total_joints + 1]; join_types + 1];
    valid_combinations[0][0] = 0.0;

    (1..=join_types).for_each(|i| {
        (0..=total_joints).for_each(|m| {
            let values: Vec<f64> = (0..=join_counts[i - 1])
                .filter_map(|j| {
                    if j <= m && valid_combinations[i - 1][m - j] != f64::NEG_INFINITY {
                        Some(
                            valid_combinations[i - 1][m - j]
                                + p[2 * join_counts[i - 1]][2 * j]
                                + c[m][j],
                        )
                    } else {
                        None
                    }
                })
                .collect();
            valid_combinations[i][m] = add_log10s(&values);
        });
    });

    (0..=total_joints)
        .map(|m| valid_combinations[join_types][m] - p[2 * total_joints][2 * m])
        .collect()
}

fn border_joins(join_counts: &Vec<usize>, p: &Vec<Vec<f64>>, c: &Vec<Vec<f64>>) -> Vec<f64> {
    let join_types: usize = join_counts.len();
    let total_joints: usize = join_counts.iter().sum();

    let mut valid_combinations: Vec<Vec<f64>> =
        vec![vec![f64::NEG_INFINITY; total_joints + 1]; join_types + 1];
    valid_combinations[0][0] = 0.0;

    (1..=join_types).for_each(|i| {
        (0..=total_joints).for_each(|b| {
            let values: Vec<f64> = (0..=join_counts[i - 1])
                .filter_map(|j| {
                    if j <= b && valid_combinations[i - 1][b - j] != f64::NEG_INFINITY {
                        Some(
                            valid_combinations[i - 1][b - j]
                                + 2.0 * p[join_counts[i - 1]][j]
                                + c[b][j],
                        )
                    } else {
                        None
                    }
                })
                .collect();
            valid_combinations[i][b] = add_log10s(&values);
        });
    });

    (0..=total_joints)
        .map(|b| (valid_combinations[join_types][b] - 2.0 * p[total_joints][b]))
        .collect()
}

fn add_log10s(values: &[f64]) -> f64 {
    if values.is_empty() {
        return f64::NEG_INFINITY;
    }

    let mut max_value = values[0]; // Initialize with the first element

    values.iter().skip(1).for_each(|value| {
        // Iterate from the second element
        if *value > max_value {
            max_value = *value;
        }
    });

    let reduce = max_value.floor();
    let mut sum: f64 = 0.0;
    values.iter().for_each(|value| {
        sum += 10f64.powf(*value - reduce);
    });

    sum.log10() + reduce
}
fn generate_factorials(max_value: usize) -> Vec<f64> {
    let mut factorial: Vec<f64> = vec![0.0; max_value + 1];
    (1..=max_value).for_each(|index| {
        factorial[index] = factorial[index - 1] + (index as f64).log10();
    });
    factorial
}

fn generate_permutions(factorial: &Vec<f64>) -> Vec<Vec<f64>> {
    let max_value = factorial.len() - 1;
    (0..=max_value)
        .map(|n| (0..=n).map(|k| factorial[n] - factorial[n - k]).collect())
        .collect()
}

fn generate_combinations(factorial: &Vec<f64>) -> Vec<Vec<f64>> {
    let max_value = factorial.len() - 1;
    (0..=max_value)
        .map(|n| {
            (0..=n)
                .map(|k| factorial[n] - factorial[n - k] - factorial[k])
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_border_joins() {}
}
