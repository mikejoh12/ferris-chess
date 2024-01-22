use ferris_chess_board;

fn main() {
    let mut board = ferris_chess_board::Board::new();
    board.print();

    let mut valid_moves = board.get_valid_moves();
    println!("Length of valid moves (King not implemented): {}\n", valid_moves.len());
    println!("Valid moves: {:?}\n", valid_moves);

    // Opening move e2e4 (long "uci" notation) - king pawn forward 2 squares
    board.make_move(&ferris_chess_board::MoveData{
        start_pos: 12,
        end_pos: 28,
    });

    board.print();
    valid_moves = board.get_valid_moves();
    println!("Length of valid moves (King not implemented): {}\n", valid_moves.len());
    println!("Valid moves: {:?}\n", valid_moves);

    // Black follows with e7e5 - king pawn forward 2 squares
    board.make_move(&ferris_chess_board::MoveData { start_pos: 52, end_pos: 36});
    board.print();
    valid_moves = board.get_valid_moves();
    println!("Length of valid moves (King not implemented): {}\n", valid_moves.len());
    println!("Valid moves: {:?}\n", valid_moves);

}
