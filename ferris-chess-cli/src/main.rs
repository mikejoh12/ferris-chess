pub mod uci;
use crate::uci::Uci;

use std::{process, time::Instant};

use clap::{Parser, Subcommand};
use ferris_chess_board::{self, perft::perft, Board, GameStatus};
use rand::Rng;

#[derive(Subcommand, Debug)]
enum Command {
    /// Engine will play against itself
    Solo,
    /// Runs perft performance test to a given depth
    Perft {
        /// The depth for perft
        depth: u8,
    },
    /// Start the engine in UCI mode (default)
    Uci,
    /// Used during development for debugging
    Debug,
}

/// Ferris-Chess: A chess engine written in Rust
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,

    /// FEN position for the chess board (default is the regular starting position)
    #[arg(
        short,
        long,
        default_value = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    )]
    fen: String,
}

fn main() {
    let args = Args::parse();
    let mut board = ferris_chess_board::Board::from_fen(&args.fen);

    match args.command {
        Some(Command::Solo) => play_against_self(&mut board),
        Some(Command::Perft { depth }) => {
            if depth == 0 || depth > 10 {
                eprintln!("Error: perft depth needs to be between 1-10");
                process::exit(1);
            }
            perft_results(&mut board, depth);
        }
        Some(Command::Uci) => handle_uci(&mut board),
        Some(Command::Debug) => debug_board(&mut board),
        None => handle_uci(&mut board),
    }
}

fn handle_uci(board: &mut Board) {
    let mut uci = Uci::new();
    uci.start_read_stdin_loop(board);
}

fn play_against_self(board: &mut Board) {
    while board.game_status == GameStatus::Ongoing {
        let moves = board.get_valid_moves();
        if moves.len() > 0 {
            let num = rand::thread_rng().gen_range(0..moves.len());
            let m = &moves[num];
            let uci_move = format!(
                "{}{}",
                board.get_square_from_idx(m.start_pos),
                board.get_square_from_idx(m.end_pos)
            );
            println!("Move: {} {}", board.full_moves, uci_move);
            board.make_move(m);
        }
    }
    board.print();
}

fn perft_results(board: &mut Board, depth: u8) {
    println!("Checking perft for n = {}", depth);
    let timing = Instant::now();
    let result = perft(depth, board);
    let elapsed = timing.elapsed();
    println!(
        "Result: {} Time: {:?} Nodes / second: {}",
        result,
        elapsed,
        (result as f64 / elapsed.as_secs_f64()) as usize
    );
}

fn debug_board(board: &mut Board) {
    board.print();
    let moves = board.get_valid_moves();
    println!("Moves: {:?}", moves);
}
