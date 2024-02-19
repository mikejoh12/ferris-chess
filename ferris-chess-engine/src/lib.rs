use ferris_chess_board::{Board, MoveData, Piece};

pub struct Engine {}

impl Engine {
    pub fn root_negamax(&mut self, board: &mut Board, depth: usize) -> Option<MoveData> {
        let mut max = i32::MIN;
        let mut best_move: Option<MoveData> = None;

        for m in board.get_valid_moves()  {
            board.make_move(&m);
            let score = -self.negamax(board,  depth - 1 );
            board.unmake_move(&m);

            if score > max {
                max = score;
                best_move = Some(m.clone());
            }
        }
        best_move
    }

    pub fn negamax(&mut self, board: &mut Board, depth: usize) -> i32 {
        if depth == 0 {
            return self.static_eval(board);
        }
        let mut max = i32::MIN;

        let moves = board.get_valid_moves();
        for m in &moves  {
            
            board.make_move(&m);
            let score = -self.negamax(board,  depth - 1 );
            board.unmake_move(&m);

            if score > max {
                max = score;
            }
        }
        
        // Handle mate and stalemate
        if moves.len() == 0 {
            if board.is_player_mated() {
                max = -100000 - depth as i32;
            } else {
                max = 0;
            }
        }

        max
    }

    fn static_eval(&self, board: &Board) -> i32 {
        let mut score = 0;

        let who_to_move = match board.is_white_to_move {
            true => 1,
            false => -1,
        };

        for i in &board.pieces_w {
            let p = board.data[*i].unwrap().1;
            score += self.get_piece_weight(p) * who_to_move;
        }

        for i in &board.pieces_b {
            let p = board.data[*i].unwrap().1;
            score -= self.get_piece_weight(p) * who_to_move;
        }
        score
    }

    fn get_piece_weight(&self, piece: Piece) -> i32 {
        match piece {
            Piece::Pawn => 100,
            Piece::Rook => 500,
            Piece::Knight => 310,
            Piece::Bishop => 320,
            Piece::Queen => 900,
            Piece::King => 300
        }
    }

}

