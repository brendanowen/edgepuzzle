use clap::{Parser, Subcommand, ValueEnum};
use edgelib::model::PuzzleStructure;

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
        searches: Vec<SearchOrder>, // Made optional to avoid requiring it

        /// Output search profile. "-" writes to stdout.
        #[arg(short, long, value_name = "CSV FILE")]
        output: String,
    },
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum SearchOrder {
    ScanRows,
    ScanColumns,
    ScanAlternate,
    SpiralIn,
    SpiralOut,
    FrameFirst,
}

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
            let puzzle_structure = PuzzleStructure::new(*x, *y, *border, *middle);
        }
    }
}
