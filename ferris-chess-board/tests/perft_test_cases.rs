use ferris_chess_board::*;
mod common;

#[test]
fn perft_pos_1() {
    let mut board = Board::from_fen("r6r/1b2k1bq/8/8/7B/8/8/R3K2R b KQ - 3 2");
    let result = common::perft(1, &mut board);
    assert_eq!(result, 8);
}

// Black is in check and can remove checking pawn with en passant.
#[test]
fn perft_pos_2() {
    let mut board = Board::from_fen("8/8/8/2k5/2pP4/8/B7/4K3 b - d3 0 3");
    let result = common::perft(1, &mut board);
    assert_eq!(result, 8);
}

#[test]
fn perft_pos_3() {
    let mut board = Board::from_fen("r1bqkbnr/pppppppp/n7/8/8/P7/1PPPPPPP/RNBQKBNR w KQkq - 2 2");
    let result = common::perft(1, &mut board);
    assert_eq!(result, 19);
}

#[test]
fn perft_pos_4() {
    let mut board =
        Board::from_fen("r3k2r/p1pp1pb1/bn2Qnp1/2qPN3/1p2P3/2N5/PPPBBPPP/R3K2R b KQkq - 3 2");
    let result = common::perft(1, &mut board);
    assert_eq!(result, 5);
}

#[test]
fn perft_pos_5() {
    let mut board =
        Board::from_fen("2kr3r/p1ppqpb1/bn2Qnp1/3PN3/1p2P3/2N5/PPPBBPPP/R3K2R b KQ - 3 2");
    let result = common::perft(1, &mut board);
    assert_eq!(result, 44);
}

#[test]
fn perft_pos_6() {
    let mut board = Board::from_fen("rnb2k1r/pp1Pbppp/2p5/q7/2B5/8/PPPQNnPP/RNB1K2R w KQ - 3 9");
    let result = common::perft(1, &mut board);
    assert_eq!(result, 39);
}

#[test]
fn perft_pos_7() {
    let mut board = Board::from_fen("2r5/3pk3/8/2P5/8/2K5/8/8 w - - 5 4");
    let result = common::perft(1, &mut board);
    assert_eq!(result, 9);
}

#[test]
fn perft_pos_8() {
    let mut board = Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");
    let result = common::perft(3, &mut board);
    assert_eq!(result, 62379);
}

#[test]
fn perft_pos_9() {
    let mut board =
        Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10");
    let result = common::perft(3, &mut board);
    assert_eq!(result, 89890);
}

#[test]
fn perft_pos_10() {
    let mut board = Board::from_fen("3k4/3p4/8/K1P4r/8/8/8/8 b - - 0 1");
    let result = common::perft(6, &mut board);
    assert_eq!(result, 1134888);
}

#[test]
fn perft_pos_11() {
    let mut board = Board::from_fen("8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1");
    let result = common::perft(6, &mut board);
    assert_eq!(result, 1440467);
}
/*
[
   {
      "depth":1,
      "nodes":8,
      "fen":"r6r/1b2k1bq/8/8/7B/8/8/R3K2R b KQ - 3 2"
   },
   {
      "depth":1,
      "nodes":8,
      "fen":"8/8/8/2k5/2pP4/8/B7/4K3 b - d3 0 3"
   },
   {
      "depth":1,
      "nodes":19,
      "fen":"r1bqkbnr/pppppppp/n7/8/8/P7/1PPPPPPP/RNBQKBNR w KQkq - 2 2"
   },
   {
      "depth":1,
      "nodes":5,
      "fen":"r3k2r/p1pp1pb1/bn2Qnp1/2qPN3/1p2P3/2N5/PPPBBPPP/R3K2R b KQkq - 3 2"
   },
   {
      "depth":1,
      "nodes":44,
      "fen":"2kr3r/p1ppqpb1/bn2Qnp1/3PN3/1p2P3/2N5/PPPBBPPP/R3K2R b KQ - 3 2"
   },
   {
      "depth":1,
      "nodes":39,
      "fen":"rnb2k1r/pp1Pbppp/2p5/q7/2B5/8/PPPQNnPP/RNB1K2R w KQ - 3 9"
   },
   {
      "depth":1,
      "nodes":9,
      "fen":"2r5/3pk3/8/2P5/8/2K5/8/8 w - - 5 4"
   },
   {
      "depth":3,
      "nodes":62379,
      "fen":"rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8"
   },
   {
      "depth":3,
      "nodes":89890,
      "fen":"r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10"
   },
   {
      "depth":6,
      "nodes":1134888,
      "fen":"3k4/3p4/8/K1P4r/8/8/8/8 b - - 0 1"
   },
   {
      "depth":6,
      "nodes":1015133,
      "fen":"8/8/4k3/8/2p5/8/B2P2K1/8 w - - 0 1"
   },
   {
      "depth":6,
      "nodes":1440467,
      "fen":"8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1"
   },
   {
      "depth":6,
      "nodes":661072,
      "fen":"5k2/8/8/8/8/8/8/4K2R w K - 0 1"
   },
   {
      "depth":6,
      "nodes":803711,
      "fen":"3k4/8/8/8/8/8/8/R3K3 w Q - 0 1"
   },
   {
      "depth":4,
      "nodes":1274206,
      "fen":"r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1"
   },
   {
      "depth":4,
      "nodes":1720476,
      "fen":"r3k2r/8/3Q4/8/8/5q2/8/R3K2R b KQkq - 0 1"
   },
   {
      "depth":6,
      "nodes":3821001,
      "fen":"2K2r2/4P3/8/8/8/8/8/3k4 w - - 0 1"
   },
   {
      "depth":5,
      "nodes":1004658,
      "fen":"8/8/1P2K3/8/2n5/1q6/8/5k2 b - - 0 1"
   },
   {
      "depth":6,
      "nodes":217342,
      "fen":"4k3/1P6/8/8/8/8/K7/8 w - - 0 1"
   },
   {
      "depth":6,
      "nodes":92683,
      "fen":"8/P1k5/K7/8/8/8/8/8 w - - 0 1"
   },
   {
      "depth":6,
      "nodes":2217,
      "fen":"K1k5/8/P7/8/8/8/8/8 w - - 0 1"
   },
   {
      "depth":7,
      "nodes":567584,
      "fen":"8/k1P5/8/1K6/8/8/8/8 w - - 0 1"
   },
   {
      "depth":4,
      "nodes":23527,
      "fen":"8/8/2k5/5q2/5n2/8/5K2/8 b - - 0 1"
   }
]
*/
