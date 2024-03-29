// https://www.chessprogramming.org/Perft_Results
// Position 6
// Tested n 1-5. Passes

use ferris_chess_board::perft::*;
use ferris_chess_board::*;

#[test]
fn perft_n_1() {
    let mut board =
        Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10");
    let zobrist_init = board.zobrist.hash;
    let result = perft(1, &mut board);
    assert_eq!(result, 46);
    assert_eq!(board.zobrist.hash, zobrist_init);
}

#[test]
fn perft_n_2() {
    let mut board =
        Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10");
    let zobrist_init = board.zobrist.hash;
    let result = perft(2, &mut board);
    assert_eq!(result, 2079);
    assert_eq!(board.zobrist.hash, zobrist_init);
}

#[test]
fn perft_n_3() {
    let mut board =
        Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10");
    let zobrist_init = board.zobrist.hash;
    let result = perft(3, &mut board);
    assert_eq!(result, 89890);
    assert_eq!(board.zobrist.hash, zobrist_init);
}

#[test]
fn perft_n_4() {
    let mut board =
        Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10");
    let zobrist_init = board.zobrist.hash;
    let result = perft(4, &mut board);
    assert_eq!(result, 3894594);
    assert_eq!(board.zobrist.hash, zobrist_init);
}

#[ignore]
#[test]
fn perft_n_5() {
    let mut board =
        Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10");
    let result = perft(5, &mut board);
    assert_eq!(result, 164075551);
}
