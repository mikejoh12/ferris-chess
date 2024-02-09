// Test cases from: https://gist.github.com/peterellisjones/8c46c28141c162d1d8a0f0badbc9cff9

use ferris_chess_board::perft::*;
use ferris_chess_board::*;

#[test]
fn perft_pos_1() {
    let mut board = Board::from_fen("r6r/1b2k1bq/8/8/7B/8/8/R3K2R b KQ - 3 2");
    let result = perft(1, &mut board);
    assert_eq!(result, 8);
}

#[test]
fn perft_pos_2() {
    let mut board = Board::from_fen("8/8/8/2k5/2pP4/8/B7/4K3 b - d3 0 3");
    let result = perft(1, &mut board);
    assert_eq!(result, 8);
}

#[test]
fn perft_pos_3() {
    let mut board = Board::from_fen("r1bqkbnr/pppppppp/n7/8/8/P7/1PPPPPPP/RNBQKBNR w KQkq - 2 2");
    let result = perft(1, &mut board);
    assert_eq!(result, 19);
}

#[test]
fn perft_pos_4() {
    let mut board =
        Board::from_fen("r3k2r/p1pp1pb1/bn2Qnp1/2qPN3/1p2P3/2N5/PPPBBPPP/R3K2R b KQkq - 3 2");
    let result = perft(1, &mut board);
    assert_eq!(result, 5);
}

#[test]
fn perft_pos_5() {
    let mut board =
        Board::from_fen("2kr3r/p1ppqpb1/bn2Qnp1/3PN3/1p2P3/2N5/PPPBBPPP/R3K2R b KQ - 3 2");
    let result = perft(1, &mut board);
    assert_eq!(result, 44);
}

#[test]
fn perft_pos_6() {
    let mut board = Board::from_fen("rnb2k1r/pp1Pbppp/2p5/q7/2B5/8/PPPQNnPP/RNB1K2R w KQ - 3 9");
    let result = perft(1, &mut board);
    assert_eq!(result, 39);
}

#[test]
fn perft_pos_7() {
    let mut board = Board::from_fen("2r5/3pk3/8/2P5/8/2K5/8/8 w - - 5 4");
    let result = perft(1, &mut board);
    assert_eq!(result, 9);
}

#[test]
fn perft_pos_8() {
    let mut board = Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");
    let result = perft(3, &mut board);
    assert_eq!(result, 62379);
}

#[test]
fn perft_pos_9() {
    let mut board =
        Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10");
    let result = perft(3, &mut board);
    assert_eq!(result, 89890);
}

#[test]
fn perft_pos_10() {
    let mut board = Board::from_fen("3k4/3p4/8/K1P4r/8/8/8/8 b - - 0 1");
    let result = perft(6, &mut board);
    assert_eq!(result, 1134888);
}

#[test]
fn perft_pos_11() {
    let mut board = Board::from_fen("8/8/4k3/8/2p5/8/B2P2K1/8 w - - 0 1");
    let result = perft(6, &mut board);
    assert_eq!(result, 1015133);
}

#[test]
fn perft_pos_12() {
    let mut board = Board::from_fen("8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1");
    let result = perft(6, &mut board);
    assert_eq!(result, 1440467);
}

#[test]
fn perft_pos_13() {
    let mut board = Board::from_fen("5k2/8/8/8/8/8/8/4K2R w K - 0 1");
    let result = perft(6, &mut board);
    assert_eq!(result, 661072);
}

#[test]
fn perft_pos_14() {
    let mut board = Board::from_fen("3k4/8/8/8/8/8/8/R3K3 w Q - 0 1");
    let result = perft(6, &mut board);
    assert_eq!(result, 803711);
}

#[test]
fn perft_pos_15() {
    let mut board = Board::from_fen("r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1");
    let result = perft(4, &mut board);
    assert_eq!(result, 1274206);
}

#[test]
fn perft_pos_16() {
    let mut board = Board::from_fen("r3k2r/8/3Q4/8/8/5q2/8/R3K2R b KQkq - 0 1");
    let result = perft(4, &mut board);
    assert_eq!(result, 1720476);
}

#[test]
fn perft_pos_17() {
    let mut board = Board::from_fen("2K2r2/4P3/8/8/8/8/8/3k4 w - - 0 1");
    let result = perft(6, &mut board);
    assert_eq!(result, 3821001);
}

#[test]
fn perft_pos_18() {
    let mut board = Board::from_fen("8/8/1P2K3/8/2n5/1q6/8/5k2 b - - 0 1");
    let result = perft(5, &mut board);
    assert_eq!(result, 1004658);
}

#[test]
fn perft_pos_19() {
    let mut board = Board::from_fen("4k3/1P6/8/8/8/8/K7/8 w - - 0 1");
    let result = perft(6, &mut board);
    assert_eq!(result, 217342);
}

#[test]
fn perft_pos_20() {
    let mut board = Board::from_fen("8/P1k5/K7/8/8/8/8/8 w - - 0 1");
    let result = perft(6, &mut board);
    assert_eq!(result, 92683);
}

#[test]
fn perft_pos_21() {
    let mut board = Board::from_fen("K1k5/8/P7/8/8/8/8/8 w - - 0 1");
    let result = perft(6, &mut board);
    assert_eq!(result, 2217);
}

#[test]
fn perft_pos_22() {
    let mut board = Board::from_fen("8/k1P5/8/1K6/8/8/8/8 w - - 0 1");
    let result = perft(7, &mut board);
    assert_eq!(result, 567584);
}

#[test]
fn perft_pos_23() {
    let mut board = Board::from_fen("8/8/2k5/5q2/5n2/8/5K2/8 b - - 0 1");
    let result = perft(4, &mut board);
    assert_eq!(result, 23527);
}
