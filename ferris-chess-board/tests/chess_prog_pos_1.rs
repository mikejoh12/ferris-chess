use ferris_chess_board::perft::*;
use ferris_chess_board::*;

#[test]
fn perft_n_1() {
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let zobrist_init = board.zobrist.hash;
    let result = perft(1, &mut board);
    assert_eq!(result, 20);
    assert_eq!(board.zobrist.hash, zobrist_init);
}

#[test]
fn perft_n_2() {
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let zobrist_init = board.zobrist.hash;
    let result = perft(2, &mut board);
    assert_eq!(result, 400);
    assert_eq!(board.zobrist.hash, zobrist_init);
}

#[test]
fn perft_n_3() {
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let zobrist_init = board.zobrist.hash;
    let result = perft(3, &mut board);
    assert_eq!(result, 8902);
    assert_eq!(board.zobrist.hash, zobrist_init);
}

#[test]
fn perft_n_4() {
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let zobrist_init = board.zobrist.hash;
    let result = perft(4, &mut board);
    assert_eq!(result, 197281);
    assert_eq!(board.zobrist.hash, zobrist_init);
}

#[test]
fn perft_n_5() {
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let zobrist_init = board.zobrist.hash;
    let result = perft(5, &mut board);
    assert_eq!(result, 4865609);
    assert_eq!(board.zobrist.hash, zobrist_init);
}

#[ignore]
#[test]
fn perft_n_6() {
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let result = perft(6, &mut board);
    assert_eq!(result, 119060324);
}
