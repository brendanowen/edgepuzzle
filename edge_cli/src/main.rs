use edgelib::model::SearchNodes;

use clap::{Parser, Subcommand, ValueEnum};
use edgelib::model::PuzzleCombinations;
use edgelib::model::PuzzleStructure;
use edgelib::model::SearchOption;
use edgelib::model::SearchOrder;
use edgelib::model::SearchProgress;

#[derive(Parser, Debug)]
#[command(name = "Edge Puzzle CLI")]
#[command(version = "1.0")]
#[command(about = "Generates information about edge puzzles", long_about = None)]
#[command(author = "Brendan Owen")]
struct CliArgs {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Outputs known search profile
    Profile {
        /// Width of puzzle
        #[arg(short, long, value_name = "INTEGER")]
        x: usize,

        /// Height of puzzle
        #[arg(short, long, value_name = "INTEGER")]
        y: usize,

        /// Border edge types
        #[arg(short, long, value_name = "INTEGER")]
        border: usize,

        /// Middle edge types
        #[arg(short, long, value_name = "INTEGER")]
        middle: usize,

        /// Comma-delimited list of search orders to calculate.
        #[arg(short, long, value_enum, num_args = 1.., value_delimiter = ',')]
        searches: Vec<SearchType>,

        /// Comma-delimited list of border join counts.
        #[arg(long, value_name = "INTEGERS", num_args = 1.., value_delimiter = ',')]
        border_joins: Option<Vec<usize>>,

        /// Comma-delimited list of middle join counts.
        #[arg(long, value_name = "INTEGERS", num_args = 1.., value_delimiter = ',')]
        middle_joins: Option<Vec<usize>>,

        /// Output search profile. "-" writes to stdout.
        #[arg(short, long, value_name = "CSV FILE")]
        output: String,
    },
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum SearchType {
    ScanRows,
    ScanColumns,
    ScanLargest,
    BuildSquare,
    SpiralIn,
    SpiralInSquare,
    SpiralOut,
    SpiralOutSquare,
    FrameFirstRows,
    FrameFirstColumns,
    FrameLastRows,
    FrameLastColumns,
}

macro_rules! impl_from_enum {
    ($from_enum:ident, $to_enum:ident) => {
        impl From<$from_enum> for $to_enum {
            fn from(e: $from_enum) -> Self {
                match e {
                    $from_enum::ScanRows => $to_enum::ScanRows,
                    $from_enum::ScanColumns => $to_enum::ScanColumns,
                    $from_enum::ScanLargest => $to_enum::ScanLargest,
                    $from_enum::BuildSquare => $to_enum::BuildSquare,
                    $from_enum::SpiralIn => $to_enum::SpiralIn,
                    $from_enum::SpiralInSquare => $to_enum::SpiralInSquare,
                    $from_enum::SpiralOut => $to_enum::SpiralOut,
                    $from_enum::SpiralOutSquare => $to_enum::SpiralOutSquare,
                    $from_enum::FrameFirstRows => $to_enum::FrameFirstRows,
                    $from_enum::FrameFirstColumns => $to_enum::FrameFirstColumns,
                    $from_enum::FrameLastRows => $to_enum::FrameLastRows,
                    $from_enum::FrameLastColumns => $to_enum::FrameLastColumns,
                }
            }
        }
    };
}

impl_from_enum!(SearchType, SearchOption);
impl_from_enum!(SearchOption, SearchType);

fn main() {
    // Parse the command-line arguments
    let args = CliArgs::parse();

    match &args.command {
        Commands::Profile {
            x,
            y,
            border,
            middle,
            searches,
            output,
            border_joins,
            middle_joins,
        } => {
            let mut puzzle_structure: PuzzleStructure =
                PuzzleStructure::new(*x, *y, *border, *middle);
            if let Some(middles) = middle_joins {
                puzzle_structure.middle_join_counts = middles.clone();
            }
            if let Some(borders) = border_joins {
                puzzle_structure.border_join_counts = borders.clone();
            }

            let puzzle_combinations: PuzzleCombinations =
                PuzzleCombinations::new(&puzzle_structure);

            let search_orders: Vec<SearchOrder> = searches
                .iter()
                .map(|search_type| SearchOrder::new(*x, *y, SearchOption::from(*search_type)))
                .collect();
            let search_progress: Vec<SearchProgress> = search_orders
                .iter()
                .map(|search_order| SearchProgress::new(&puzzle_structure, search_order))
                .collect();

            let search_nodes: Vec<SearchNodes> = search_progress
                .iter()
                .map(|search_progress| SearchNodes::new(&puzzle_combinations, search_progress))
                .collect();

            let mut result_string = String::new();
            result_string.push_str("Depth");
            for search in searches {
                result_string.push_str(&format!(",{:?}", search));
            }
            result_string.push_str("\n");

            let size = x * y;

            for depth in 0..=size {
                result_string.push_str(&format!("{}", depth));
                for search_node in search_nodes.iter() {
                    result_string.push_str(&format!(",{}", search_node.nodes[depth]));
                }
                result_string.push_str("\n");
            }

            output_default_stdout(output, result_string);
        }
    }
}

/// Outputs the result to a file or stdout
/// If the output is "-", it writes to stdout.
/// Otherwise, it writes to the specified file.
/// If the file already exists, it will be overwritten.
/// If the file cannot be written, it will panic.
fn output_default_stdout(output: &str, string: String) {
    if output != "-" {
        std::fs::write(output, string).expect("Unable to write to file");
        eprintln!("Data written to file: {}", output);
    } else {
        println!("{}", string);
    }
}
