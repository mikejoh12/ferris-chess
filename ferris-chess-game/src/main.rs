use ferris_chess_board;

fn main() {
    let mut board = ferris_chess_board::Board::from_fen(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    );
    board.print();

    let mut valid_moves = board.get_valid_moves();

    board.print_moves(&valid_moves);

    println!("Game status: {:?}\n", board.game_status);

    /*

    // Opening move f2f3 (long "uci" notation) - f pawn forward 1 square
    board.make_move(&ferris_chess_board::MoveData{
        start_pos: 13,
        end_pos: 21,
    });

    board.print();
    valid_moves = board.get_valid_moves();
    println!("Is white to move: {}", board.is_white_to_move);
    board.print_moves(&valid_moves);
    println!("Game status: {:?}\n", board.game_status);

    // Black moves e7e5
    board.make_move(&ferris_chess_board::MoveData { start_pos: 52, end_pos: 36});
    board.print();
    valid_moves = board.get_valid_moves();
    println!("Is white to move: {}", board.is_white_to_move);
    board.print_moves(&valid_moves);
    println!("Game status: {:?}\n", board.game_status);

    // White moves g2g4
    board.make_move(&ferris_chess_board::MoveData { start_pos: 14, end_pos: 30});
    board.print();
    valid_moves = board.get_valid_moves();
    println!("Is white to move: {}", board.is_white_to_move);
    board.print_moves(&valid_moves);
    println!("Game status: {:?}\n", board.game_status);

    // Black moves d8h4 - Fool's mate
    board.make_move(&ferris_chess_board::MoveData { start_pos: 59, end_pos: 31});
    board.print();
    valid_moves = board.get_valid_moves();
    println!("Is white to move: {}", board.is_white_to_move);
    board.print_moves(&valid_moves);
    println!("Game status: {:?}\n", board.game_status);

    */
}
