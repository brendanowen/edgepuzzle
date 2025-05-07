use num_bigint::BigUint;

pub fn middle_joins(join_counts: Vec<usize>) -> Vec<BigUint> {
    let join_types: usize = join_counts.len();
    let total_joints: usize = join_counts.iter().sum();
    let total_edges: usize = total_joints * 2;
    let p: Vec<Vec<BigUint>> = generate_permutions(total_edges);
    let c: Vec<Vec<BigUint>> = generate_combinations(total_edges);

    let mut valid_combinations: Vec<Vec<BigUint>> =
        vec![vec![BigUint::ZERO; join_types]; total_joints + 1];
    valid_combinations[0][0] = BigUint::from(1usize);

    (1..=total_joints).for_each(|m| {
        (0..join_types).for_each(|i| {
            valid_combinations[i][m] = (0..join_counts[i])
                .map(|j| {
                    if j <= m {
                        valid_combinations[i - 1][m - j].clone()
                            * p[2 * join_counts[i]][2 * j].clone()
                            * c[m][j].clone()
                    } else {
                        BigUint::ZERO
                    }
                })
                .sum::<BigUint>();
        });
    });

    valid_combinations
        .iter()
        .map(|valids| valids.last().unwrap().clone())
        .collect::<Vec<BigUint>>()
}

fn generate_factorials(max_value: usize) -> Vec<BigUint> {
    let mut factorial: Vec<BigUint> = vec![BigUint::ZERO; max_value + 1];
    factorial[0] = BigUint::from(1usize);
    (1..=max_value).for_each(|index| {
        factorial[index] = factorial[index - 1].clone() * BigUint::from(index);
    });

    factorial
}

fn generate_permutions(max_value: usize) -> Vec<Vec<BigUint>> {
    let factorial: Vec<BigUint> = generate_factorials(max_value);
    (0..=max_value)
        .map(|n| {
            (0..=n)
                .map(|k| factorial[n].clone() / factorial[n - k].clone())
                .collect()
        })
        .collect()
}

fn generate_combinations(max_value: usize) -> Vec<Vec<BigUint>> {
    let factorial: Vec<BigUint> = generate_factorials(max_value);
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
    use num_bigint::BigUint;

    use crate::combinations::joins::generate_combinations;
    use crate::combinations::joins::generate_permutions;
    use crate::combinations::middle_joins;

    #[test]
    fn test_middle_joins() {
        let middle_joins = middle_joins(vec![35; 12]);
        println!("{:?}", middle_joins);
    }

    #[test]
    fn test_combinations() {
        let combinations = generate_combinations(10);
        assert_eq!(
            combinations[5][3],
            BigUint::from(5 * 4 * 3 * 2 * 1 / (3 * 2 * 1) / (2 * 1) as usize)
        )
    }

    #[test]
    fn test_perimutions() {
        let perimutions = generate_permutions(10);
        assert_eq!(
            perimutions[5][3],
            BigUint::from(5 * 4 * 3 * 2 * 1 / (2 * 1) as usize)
        )
    }
}
