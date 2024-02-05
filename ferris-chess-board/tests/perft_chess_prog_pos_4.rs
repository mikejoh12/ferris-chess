// https://www.chessprogramming.org/Perft_Results
// Position 4 Perft
// left: 422321    Failure with w to move on n 4
// right: 422333
// Works for mirrored code where b starts

use ferris_chess_board::*;
mod common;
use common::perft;

#[test]
fn perft_n_1() {
    let mut board = Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1");
    let result = perft(1, &mut board);
    assert_eq!(result, 6);
}

#[test]
fn perft_n_2() {
    let mut board = Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1");
    let result = perft(2, &mut board);
    assert_eq!(result, 264);
}

#[test]
fn perft_n_3() {
    let mut board = Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1");
    let result = perft(3, &mut board);
    assert_eq!(result, 9467);
}

#[ignore]
#[test]
fn perft_n_4() {
    let mut board = Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1");
    let result = perft(4, &mut board);
    assert_eq!(result, 422333);
}

#[ignore]
#[test]
fn perft_n_5() {
    let mut board = Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1");
    let result = perft(5, &mut board);
    assert_eq!(result, 15833292);
}