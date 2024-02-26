// https://www.chessprogramming.org/Perft_Results
// Position 5

use ferris_chess_board::perft::*;
use ferris_chess_board::*;

#[test]
fn perft_n_1() {
    let mut board = Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");
    let result = perft(1, &mut board);
    assert_eq!(result, 44);
}

#[test]
fn perft_n_2() {
    let mut board = Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");
    let result = perft(2, &mut board);
    assert_eq!(result, 1486);
}

#[test]
fn perft_n_3() {
    let mut board = Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");
    let result = perft(3, &mut board);
    assert_eq!(result, 62379);
}

#[test]
fn perft_n_4() {
    let mut board = Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");
    let result = perft(4, &mut board);
    assert_eq!(result, 2103487);
}

#[ignore]
#[test]
fn perft_n_5() {
    let mut board = Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");
    let result = perft(5, &mut board);
    assert_eq!(result, 89941194);
}
