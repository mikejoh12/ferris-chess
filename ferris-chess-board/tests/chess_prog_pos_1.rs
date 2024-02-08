use ferris_chess_board::*;
mod common;
use common::perft;

// Current performance
// n = 1-6 time is 7.74s with cargo test --release

#[test]
fn perft_n_1() {
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let result = perft(1, &mut board);
    assert_eq!(result, 20);
}

#[test]
fn perft_n_2() {
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let result = perft(2, &mut board);
    assert_eq!(result, 400);
}

#[test]
fn perft_n_3() {
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let result = perft(3, &mut board);
    assert_eq!(result, 8902);
}

#[test]
fn perft_n_4() {
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let result = perft(4, &mut board);
    assert_eq!(result, 197281);
}

#[test]
fn perft_n_5() {
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let result = perft(5, &mut board);
    assert_eq!(result, 4865609);
}

#[test]
fn perft_n_6() {
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let result = perft(6, &mut board);
    assert_eq!(result, 119060324);
}
