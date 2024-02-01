use ferris_chess_board::*;

fn perft(depth: u8, board: &mut Board) -> usize {

    let moves = board.get_valid_moves();
    let mut nodes = 0;
    
    if depth == 1 {
        return moves.len()
    };

    for m in moves {
        board.make_move(&m);
        nodes += perft(depth - 1, board);
        board.unmake_move(&m);
    }

    nodes
}

#[test]
fn perft_n_1() {
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let result = perft(1, &mut board);
    assert_eq!(result, 20);
}

#[test]
fn perft_n_2() {
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let result = perft(2, &mut board);
    assert_eq!(result, 400);
}

#[test]
fn perft_n_3() {
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let result = perft(3, &mut board);
    assert_eq!(result, 8902);
}

#[test]
fn perft_n_4() {
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let result = perft(4, &mut board);
    assert_eq!(result, 197281);
}

#[test]
fn perft_n_5() {
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let result = perft(5, &mut board);
    assert_eq!(result, 4865609);
}

// Pre optimization perft n 6 = 89.83s cargo test --release
#[ignore]
#[test]
fn perft_n_6() {
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let result = perft(6, &mut board);
    assert_eq!(result, 119060324);
}