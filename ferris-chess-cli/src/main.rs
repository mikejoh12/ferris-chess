use std::time::Instant;

use ferris_chess_board::{self, perft::perft, Board, GameStatus};
use clap::{Parser, ValueEnum};
use rand::Rng;

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
        CliMode::Solo => play_against_self(&mut board),
        CliMode::Perft => perft_results(&mut board),
        CliMode::Uci => start_uci(&board),
        CliMode::Debug => debug_board(&mut board)}
}

fn play_against_self(board: &mut Board) {
            while board.game_status == GameStatus::Ongoing {
                let moves = board.get_valid_moves();
                if moves.len() > 0 {
                    let num = rand::thread_rng().gen_range(0..moves.len());
                    let m = &moves[num];
                    let uci_move = format!("{}{}", board.get_square_from_idx(m.start_pos), board.get_square_from_idx(m.end_pos));
                    println!("Move: {} {}", board.full_moves, uci_move);
                    board.make_move(m);

                }
            };
            board.print();
}

fn perft_results(board: &mut Board) {
    todo!()
}

fn start_uci(board: &Board) {
    todo!();
}

fn debug_board(board: &mut Board) {
    let depth = 6;
    let timing = Instant::now();
    let result = perft(depth, board);
    println!("Perft result for n = {}: {} Time: {:?}", depth, result, timing.elapsed());
}