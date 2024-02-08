use std::time::Instant;

use ferris_chess_board::{self, perft::perft};
use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
enum CliMode {
    Solo,
    Perft,
    Uci,
    Debug,
}

/// Ferris-Chess: A chess engine written in Rust
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Mode that the chess engine will start in
    #[arg(short, long)]
    mode: CliMode,

    /// FEN position for the chess board (default is the regular starting position)
    #[arg(short, long, default_value = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")]
    fen: String,
}

fn main() {
    let args = Args::parse();
    let mut board = ferris_chess_board::Board::from_fen(&args.fen);

    match args.mode {
        CliMode::Solo => {
            // Engine will play against itself
            todo!();
        },
        CliMode::Perft => {
            // Counts nr of leafnodes to a certain depth from current position
            let depth = 6;
            let timing = Instant::now();
            let result = perft(depth, &mut board);
            println!("Perft result for n = {}: {} Time: {:?}", depth, result, timing.elapsed());
        },
        CliMode::Uci => {
            // Implement the UCI chess protocol to communicate with GUI
            todo!();
        },
        CliMode::Debug => {
            // Use this for testing purposes for now
            board.print();
            let valid_moves = board.get_valid_moves();
            board.print_moves(&valid_moves);
            println!("Nr of moves {}", valid_moves.len());
        }

    }
}
