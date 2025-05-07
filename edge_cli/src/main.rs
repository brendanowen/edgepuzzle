use clap::{Parser, Subcommand, ValueEnum};
use edgelib::model::PuzzleCombinations;
use edgelib::model::PuzzleStructure;
use edgelib::model::SearchOption;
use edgelib::model::SearchOrder;

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
        searches: Vec<SearchType>, // Made optional to avoid requiring it

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
        } => {
            let puzzle_structure: PuzzleStructure = PuzzleStructure::new(*x, *y, *border, *middle);
            let puzzle: PuzzleCombinations = PuzzleCombinations::new(&puzzle_structure);
            let search_orders: Vec<SearchOrder> = searches
                .iter()
                .map(|search_type| SearchOrder::new(*x, *y, SearchOption::from(*search_type)))
                .collect();
        }
    }
}
