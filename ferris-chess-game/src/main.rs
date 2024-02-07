use ferris_chess_board;

fn main() {
    let mut board = ferris_chess_board::Board::from_fen(
        "n1n5/PPPk4/8/8/8/8/4Kppp/5N1N b - - 0 1",
    );
    board.print();

    let valid_moves = board.get_valid_moves();
    board.print_moves(&valid_moves);
    println!("Nr of moves {}", valid_moves.len());
}
