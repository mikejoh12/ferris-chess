use ferris_chess_board::*;
mod common;
use common::divide;

static POS_4: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";

#[test]
fn perft_divide_n_3() {
    let mut board = Board::from_fen(POS_4);
    let result = divide(3, &mut board);
    assert_eq!(result, 9467)
}

#[test]
fn perft_divide_n_4() {
    let mut board = Board::from_fen(POS_4);
    let result = divide(4, &mut board);
    assert_eq!(result, 422333)
}
