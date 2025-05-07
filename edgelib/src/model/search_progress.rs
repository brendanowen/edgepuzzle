use super::{PuzzleStructure, SearchOrder};

#[derive(Clone, Debug)]
pub struct SearchProgress {
    width: usize,
    height: usize,
    size: usize,
    initial: Used,
    progress: Vec<Used>,
}

#[derive(Clone, Debug)]
pub struct Used {
    corners: usize,
    edges: usize,
    interiors: usize,
    borders: usize,
    middles: usize,
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

        let initial: Used = Used {
            corners: puzzle_structure.corners,
            edges: puzzle_structure.edges,
            interiors: puzzle_structure.interiors,
            borders: puzzle_structure.border_join_counts.iter().sum(),
            middles: puzzle_structure.middle_join_counts.iter().sum(),
        };

        SearchProgress {
            width: puzzle_structure.width,
            height: puzzle_structure.height,
            size: search_order.size,
            initial: initial.clone(),
            progress: vec![initial.clone()],
        }
    }
}
