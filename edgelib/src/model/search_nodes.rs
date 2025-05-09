use super::{FastCombinations, PuzzleCombinations, SearchProgress, Used};

#[derive(Clone, Debug)]
pub struct SearchNodes {
    pub nodes: Vec<f64>,
}

impl SearchNodes {
    pub fn new(puzzle_combinations: &FastCombinations, search_progress: &SearchProgress) -> Self {
        if search_progress.width <= 1 || search_progress.height <= 1 {
            panic!("Width and height must be greater than 1");
        }
        let nodes: Vec<f64> = search_progress
            .progress
            .iter()
            .map(|item: &Used| {
                puzzle_combinations.log10_border[item.borders]
                    + puzzle_combinations.log10_middle[item.middles]
                    + puzzle_combinations.log10_corner[item.corners]
                    + puzzle_combinations.log10_edge[item.edges]
                    + puzzle_combinations.log10_interior[item.interiors]
            })
            .collect();

        SearchNodes { nodes }
    }
}
