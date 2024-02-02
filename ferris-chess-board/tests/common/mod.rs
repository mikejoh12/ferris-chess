use ferris_chess_board::*;

pub fn perft(depth: u8, board: &mut Board) -> usize {

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