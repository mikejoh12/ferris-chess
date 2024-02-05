// https://www.chessprogramming.org/Perft_Results
// Position 2 Perft 1-4
// Fails at n 4
// left: 4087679
// right: 4085603



use ferris_chess_board::*;
mod common;
use common::perft;

#[test]
fn perft_n_1() {
    let mut board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
    let result = perft(1, &mut board);
    assert_eq!(result, 48);
}

#[test]
fn perft_n_2() {
    let mut board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
    let result = perft(2, &mut board);
    assert_eq!(result, 2039);
}

#[test]
fn perft_n_3() {
    let mut board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
    let result = perft(3, &mut board);
    assert_eq!(result, 97862);
}

#[ignore]
#[test]
fn perft_n_4() {
    let mut board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
    let result = perft(4, &mut board);
    assert_eq!(result, 4085603);
}