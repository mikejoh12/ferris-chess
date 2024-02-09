use crate::Board;

pub fn perft(depth: u8, board: &mut Board) -> usize {
    let moves = board.get_valid_moves();
    let mut nodes = 0;

    if depth == 1 {
        return moves.len();
    };

    for m in moves {
        board.make_move(&m);
        nodes += perft(depth - 1, board);
        board.unmake_move(&m);
    }

    nodes
}

#[allow(dead_code)]
pub fn divide(depth: u8, board: &mut Board) -> usize {
    let moves = board.get_valid_moves();
    let mut nodes = 0;

    for m in moves {
        let mut sub_board = board.clone();
        sub_board.make_move(&m);
        let p_count = perft(depth - 1, &mut sub_board);
        nodes += p_count;
        println!(
            "{}{}: {}",
            sub_board.get_square_from_idx(m.start_pos),
            sub_board.get_square_from_idx(m.end_pos),
            p_count
        );
    }
    println!("Total: {}", nodes);
    nodes
}
