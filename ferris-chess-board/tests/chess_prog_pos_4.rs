// https://www.chessprogramming.org/Perft_Results
// Position 4 Perft. Tests successful for n 1-5

use ferris_chess_board::*;
mod common;
use common::perft;

static POS_4: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";

#[test]
fn perft_n_1() {
    let mut board = Board::from_fen(POS_4);
    let result = perft(1, &mut board);
    assert_eq!(result, 6);
}

#[test]
fn perft_n_2() {
    let mut board = Board::from_fen(POS_4);
    let result = perft(2, &mut board);
    assert_eq!(result, 264);
}

#[test]
fn perft_n_3() {
    let mut board = Board::from_fen(POS_4);
    let result = perft(3, &mut board);
    assert_eq!(result, 9467);
}

#[test]
fn perft_n_4() {
    let mut board = Board::from_fen(POS_4);
    let result = perft(4, &mut board);
    assert_eq!(result, 422333);
}

#[test]
fn perft_n_5() {
    let mut board = Board::from_fen(POS_4);
    let result = perft(5, &mut board);
    assert_eq!(result, 15833292);
}