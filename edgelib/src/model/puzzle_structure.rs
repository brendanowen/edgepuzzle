#[derive(Debug)]
pub struct PuzzleStructure {
    width: usize,
    height: usize,
    border_types: usize,
    middle_types: usize,
    grid: Vec<Vec<Location>>,
    corners: usize,
    edges: usize,
    interiors: usize,
    border_joins: usize,
    middle_joins: usize,
    border_join_counts: Vec<usize>,
    middle_join_counts: Vec<usize>,
}

#[derive(Clone, Debug)]
struct Location {
    x: usize,
    y: usize,
    location_type: LocationType,
    joins: Vec<Join>,
}

#[derive(Clone, Debug)]
struct Join {
    x: usize,
    y: usize,
    join_type: JoinType,
}

#[derive(PartialEq, Clone, Debug)]
enum LocationType {
    Corner,
    Edge,
    Interior,
}

#[derive(PartialEq, Clone, Debug)]
enum JoinType {
    Border,
    Middle,
}

const COORDINATES: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

impl PuzzleStructure {
    pub fn new(width: usize, height: usize, border_types: usize, middle_types: usize) -> Self {
        if width <= 1 || height <= 0 {
            panic!("Width and height must be greater than 1");
        }

        if border_types == 0 || middle_types == 0 {
            panic!("There must be at least one border and middle join type");
        }

        let grid: Vec<Vec<Location>> = (0..width)
            .into_iter()
            .map(|x| {
                (0..height)
                    .into_iter()
                    .map(|y| {
                        let mut joins: Vec<Join> = vec![];
                        COORDINATES.iter().for_each(|coordinate| {
                            let test_x: isize = (x as isize) + coordinate.0;
                            let test_y: isize = (y as isize) + coordinate.1;
                            if test_x < 0
                                || test_x >= width as isize
                                || test_y < 0
                                || test_y >= height as isize
                            {
                                return;
                            }
                            let mut new_join: Join = Join {
                                x: test_x as usize,
                                y: test_y as usize,
                                join_type: JoinType::Middle,
                            };
                            if new_join.x == 0 || new_join.x == (width - 1) {
                                if new_join.y == 0 || new_join.y == (height - 1) || new_join.x == x
                                {
                                    new_join.join_type = JoinType::Border;
                                }
                            } else if new_join.y == 0 || new_join.y == (height - 1) {
                                if new_join.y == y {
                                    new_join.join_type = JoinType::Border;
                                }
                            }
                            joins.push(new_join);
                        });

                        let location_type: LocationType = match joins.len() {
                            2 => LocationType::Corner,
                            3 => LocationType::Edge,
                            _ => LocationType::Interior,
                        };

                        Location {
                            x,
                            y,
                            location_type,
                            joins,
                        }
                    })
                    .collect()
            })
            .collect();

        let corners = grid
            .iter()
            .flatten()
            .filter(|location| location.location_type == LocationType::Corner)
            .count();
        let edges = grid
            .iter()
            .flatten()
            .filter(|location| location.location_type == LocationType::Edge)
            .count();
        let interiors = grid
            .iter()
            .flatten()
            .filter(|location| location.location_type == LocationType::Interior)
            .count();
        let border_joins = grid
            .iter()
            .flatten()
            .map(|location| {
                location
                    .joins
                    .iter()
                    .filter(|join| join.join_type == JoinType::Border)
                    .count()
            })
            .sum::<usize>()
            / 2;
        let middle_joins = grid
            .iter()
            .flatten()
            .map(|location| {
                location
                    .joins
                    .iter()
                    .filter(|join| join.join_type == JoinType::Middle)
                    .count()
            })
            .sum::<usize>()
            / 2;

        let border_join_counts: Vec<usize> = (0..border_types)
            .map(|border_index| ((border_joins + border_types - border_index - 1) / border_types))
            .collect();
        let middle_join_counts: Vec<usize> = (0..middle_types)
            .map(|middle_index| ((middle_joins + middle_types - middle_index - 1) / middle_types))
            .collect();

        PuzzleStructure {
            width,
            height,
            border_types,
            middle_types,
            grid,
            corners,
            edges,
            interiors,
            border_joins,
            middle_joins,
            border_join_counts,
            middle_join_counts,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PuzzleStructure;

    #[test]
    fn test_build_structure() {
        let puzzle_structure = PuzzleStructure::new(16, 16, 5, 12);

        println!("{:?}", puzzle_structure);
    }
}
