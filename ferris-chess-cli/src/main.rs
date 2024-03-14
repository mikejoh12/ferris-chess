pub mod uci;
use crate::uci::Uci;
use clap::{Parser, Subcommand};
use ferris_chess_board::{self, perft::perft, Board};
use std::cell::RefCell;
use std::io::Write;
use std::process;
use std::sync::{Arc, Mutex};
use std::{fs::File, io, panic, time::Instant};

#[derive(Subcommand, Debug)]
enum Command {
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

/// Redirects error messages from stderr to stdout and writes them to a file.
/// This allows them to be read when engine runs as a process under a GUI such as Cutechess.
fn init_logging() {
    let file = Arc::new(Mutex::new(
        File::create("error.log").expect("Failed to create error.log"),
    ));
    let stderr = io::stderr();
    let stderr_lock = Arc::new(Mutex::new(RefCell::new(stderr)));

    // Redirect stderr to file and stdout
    let _ = panic::take_hook();
    let file_clone = Arc::clone(&file);
    panic::set_hook(Box::new(move |info| {
        let stderr_lock = stderr_lock.lock().unwrap();
        let _ = writeln!(stderr_lock.borrow_mut(), "{}", info);
        let mut file_lock = file_clone.lock().unwrap();
        let _ = writeln!(&mut *file_lock, "{}", info); // Redirect to file
        let _ = writeln!(&mut io::stdout(), "{}", info); // Redirect to stdout
    }));
}

fn main() {
    init_logging();

    let args = Args::parse();
    let mut board = ferris_chess_board::Board::from_fen(&args.fen);

    match args.command {
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
    let moves = board.get_pseudo_legal_moves();
    println!("Moves: {:?}", moves);
}
