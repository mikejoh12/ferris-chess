// https://www.chessprogramming.org/Perft_Results
// Passes: Position 3 Perft 1-6
// Fails at n 7 (white) with too many positions

use ferris_chess_board::*;
mod common;
use common::perft;

#[test]
fn perft_n_1() {
    let mut board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    let result = perft(1, &mut board);
    assert_eq!(result, 14);
}

#[test]
fn perft_n_2() {
    let mut board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    let result = perft(2, &mut board);
    assert_eq!(result, 191);
}

#[test]
fn perft_n_3() {
    let mut board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    let result = perft(3, &mut board);
    assert_eq!(result, 	2812);
}

#[test]
fn perft_n_4() {
    let mut board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    let result = perft(4, &mut board);
    assert_eq!(result, 43238);
}

#[test]
fn perft_n_5() {
    let mut board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    let result = perft(5, &mut board);
    assert_eq!(result, 674624);
}

#[test]
fn perft_n_6() {
    let mut board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    let result = perft(6, &mut board);
    assert_eq!(result, 11030083);
}

#[ignore]
#[test]
fn perft_n_7() {
    let mut board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    let result = perft(7, &mut board);
    assert_eq!(result, 178633661);
}