// https://www.chessprogramming.org/Perft_Results
// Passes: Position 3 Perft 1-7

use ferris_chess_board::perft::*;
use ferris_chess_board::*;

#[test]
fn perft_n_1() {
    let mut board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    let zobrist_init = board.zobrist.hash;
    let result = perft(1, &mut board);
    assert_eq!(result, 14);
    assert_eq!(board.zobrist.hash, zobrist_init);
}

#[test]
fn perft_n_2() {
    let mut board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    let zobrist_init = board.zobrist.hash;
    let result = perft(2, &mut board);
    assert_eq!(result, 191);
    assert_eq!(board.zobrist.hash, zobrist_init);
}

#[test]
fn perft_n_3() {
    let mut board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    let zobrist_init = board.zobrist.hash;
    let result = perft(3, &mut board);
    assert_eq!(result, 2812);
    assert_eq!(board.zobrist.hash, zobrist_init);
}

#[test]
fn perft_n_4() {
    let mut board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    let zobrist_init = board.zobrist.hash;
    let result = perft(4, &mut board);
    assert_eq!(result, 43238);
    assert_eq!(board.zobrist.hash, zobrist_init);
}

#[test]
fn perft_n_5() {
    let mut board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    let zobrist_init = board.zobrist.hash;
    let result = perft(5, &mut board);
    assert_eq!(result, 674624);
    assert_eq!(board.zobrist.hash, zobrist_init);
}

#[test]
fn perft_n_6() {
    let mut board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    let zobrist_init = board.zobrist.hash;
    let result = perft(6, &mut board);
    assert_eq!(result, 11030083);
    assert_eq!(board.zobrist.hash, zobrist_init);
}

#[ignore]
#[test]
fn perft_n_7() {
    let mut board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    let zobrist_init = board.zobrist.hash;
    let result = perft(7, &mut board);
    assert_eq!(result, 178633661);
    assert_eq!(board.zobrist.hash, zobrist_init);
}
