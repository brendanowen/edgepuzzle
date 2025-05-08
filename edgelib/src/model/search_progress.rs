use super::{JoinType, LocationType, PuzzleStructure, SearchOrder, search_order::Location};

#[derive(Clone, Debug)]
pub struct SearchProgress {
    pub width: usize,
    pub height: usize,
    pub size: usize,
    pub end_point: Used,
    pub progress: Vec<Used>,
}

#[derive(Clone, Debug)]
pub struct Used {
    pub corners: usize,
    pub edges: usize,
    pub interiors: usize,
    pub borders: usize,
    pub middles: usize,
}

impl SearchProgress {
    pub fn new(puzzle_structure: &PuzzleStructure, search_order: &SearchOrder) -> Self {
        if puzzle_structure.width <= 1 || puzzle_structure.height <= 1 {
            panic!("Width and height must be greater than 1");
        }
        if search_order.width <= 1 || search_order.height <= 1 {
            panic!("Width and height must be greater than 1");
        }
        if search_order.width != puzzle_structure.width {
            panic!("Search order and Puzzle structure width needs to match");
        }
        if search_order.height != puzzle_structure.height {
            panic!("Search order and Puzzle structure height needs to match");
        }

        let end_point: Used = Used {
            corners: puzzle_structure.corners,
            edges: puzzle_structure.edges,
            interiors: puzzle_structure.interiors,
            borders: puzzle_structure.border_join_counts.iter().sum(),
            middles: puzzle_structure.middle_join_counts.iter().sum(),
        };

        let mut current_used = Used {
            corners: 0,
            edges: 0,
            interiors: 0,
            borders: 0,
            middles: 0,
        };

        let mut progress: Vec<Used> = vec![current_used.clone()];

        let mut filled: Vec<Vec<bool>> = vec![vec![false; search_order.height]; search_order.width];
        search_order.order.iter().for_each(|location: &Location| {
            let mut next_used: Used = current_used.clone();
            let grid_location = &puzzle_structure.grid[location.x][location.y];
            grid_location.joins.iter().for_each(|join| {
                if filled[join.x][join.y] {
                    match join.join_type {
                        JoinType::Border => {
                            next_used.borders += 1;
                        }
                        JoinType::Middle => {
                            next_used.middles += 1;
                        }
                    }
                }
            });
            match grid_location.location_type {
                LocationType::Corner => {
                    next_used.corners += 1;
                }
                LocationType::Edge => {
                    next_used.edges += 1;
                }
                LocationType::Interior => {
                    next_used.interiors += 1;
                }
            }
            filled[location.x][location.y] = true;
            progress.push(next_used.clone());
            current_used = next_used;
        });

        SearchProgress {
            width: puzzle_structure.width,
            height: puzzle_structure.height,
            size: search_order.size,
            end_point: end_point.clone(),
            progress,
        }
    }
}
