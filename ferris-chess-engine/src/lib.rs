use ferris_chess_board::{Board, Color, MoveData, Piece};
use std::time::{Duration, Instant};

#[allow(dead_code)]
pub struct Engine {
    mg_pawn_table_b: [i32; 64],
    eg_pawn_table_b: [i32; 64],
    mg_knight_table_b: [i32; 64],
    eg_knight_table_b: [i32; 64],
    mg_bishop_table_b: [i32; 64],
    eg_bishop_table_b: [i32; 64],
    mg_rook_table_b: [i32; 64],
    eg_rook_table_b: [i32; 64],
    mg_queen_table_b: [i32; 64],
    eg_queen_table_b: [i32; 64],
    mg_king_table_b: [i32; 64],
    eg_king_table_b: [i32; 64],

    mg_pawn_table_w: [i32; 64],
    eg_pawn_table_w: [i32; 64],
    mg_knight_table_w: [i32; 64],
    eg_knight_table_w: [i32; 64],
    mg_bishop_table_w: [i32; 64],
    eg_bishop_table_w: [i32; 64],
    mg_rook_table_w: [i32; 64],
    eg_rook_table_w: [i32; 64],
    mg_queen_table_w: [i32; 64],
    eg_queen_table_w: [i32; 64],
    mg_king_table_w: [i32; 64],
    eg_king_table_w: [i32; 64],

    best_move: Option<MoveData>,
}

fn mg_piece_weight(piece: Piece) -> i32 {
    match piece {
        Piece::Pawn => 82,
        Piece::Knight => 337,
        Piece::Bishop => 365,
        Piece::Rook => 477,
        Piece::Queen => 1025,
        Piece::King => 0,
    }
}

fn eg_piece_weight(piece: Piece) -> i32 {
    match piece {
        Piece::Pawn => 94,
        Piece::Knight => 281,
        Piece::Bishop => 297,
        Piece::Rook => 512,
        Piece::Queen => 936,
        Piece::King => 0,
    }
}

fn mirror_table(t: &[i32; 64]) -> [i32; 64] {
    let mut mirrored: [i32; 64] = [0; 64];
    for i in 0..64 {
        mirrored[i ^ 56] = t[i];
    }
    mirrored
}

fn get_game_phase_table(t: &[i32; 64], corr: i32) -> [i32; 64] {
    let mut corrected: [i32; 64] = [0; 64];
    for i in 0..64 {
        corrected[i] = t[i] + corr;
    }
    corrected
}

impl Engine {
    pub fn new() -> Engine {
        #[rustfmt::skip]
        let mg_pawn_table: [i32; 64] = [
            0,   0,   0,   0,   0,   0,  0,   0,
           98, 134,  61,  95,  68, 126, 34, -11,
           -6,   7,  26,  31,  65,  56, 25, -20,
          -14,  13,   6,  21,  23,  12, 17, -23,
          -27,  -2,  -5,  12,  17,   6, 10, -25,
          -26,  -4,  -4, -10,   3,   3, 33, -12,
          -35,  -1, -20, -23, -15,  24, 38, -22,
            0,   0,   0,   0,   0,   0,  0,   0,
        ];

        #[rustfmt::skip]
        let eg_pawn_table: [i32; 64] = [
            0,   0,   0,   0,   0,   0,   0,   0,
          178, 173, 158, 134, 147, 132, 165, 187,
           94, 100,  85,  67,  56,  53,  82,  84,
           32,  24,  13,   5,  -2,   4,  17,  17,
           13,   9,  -3,  -7,  -7,  -8,   3,  -1,
            4,   7,  -6,   1,   0,  -5,  -1,  -8,
           13,   8,   8,  10,  13,   0,   2,  -7,
            0,   0,   0,   0,   0,   0,   0,   0,
        ];

        #[rustfmt::skip]
        let mg_knight_table: [i32; 64] = [
            -167, -89, -34, -49,  61, -97, -15, -107,
             -73, -41,  72,  36,  23,  62,   7,  -17,
             -47,  60,  37,  65,  84, 129,  73,   44,
              -9,  17,  19,  53,  37,  69,  18,   22,
             -13,   4,  16,  13,  28,  19,  21,   -8,
             -23,  -9,  12,  10,  19,  17,  25,  -16,
             -29, -53, -12,  -3,  -1,  18, -14,  -19,
            -105, -21, -58, -33, -17, -28, -19,  -23,
        ];

        #[rustfmt::skip]
        let eg_knight_table: [i32; 64] = [
            -58, -38, -13, -28, -31, -27, -63, -99,
            -25,  -8, -25,  -2,  -9, -25, -24, -52,
            -24, -20,  10,   9,  -1,  -9, -19, -41,
            -17,   3,  22,  22,  22,  11,   8, -18,
            -18,  -6,  16,  25,  16,  17,   4, -18,
            -23,  -3,  -1,  15,  10,  -3, -20, -22,
            -42, -20, -10,  -5,  -2, -20, -23, -44,
            -29, -51, -23, -15, -22, -18, -50, -64,
        ];

        #[rustfmt::skip]
        let mg_bishop_table: [i32; 64] = [
            -29,   4, -82, -37, -25, -42,   7,  -8,
            -26,  16, -18, -13,  30,  59,  18, -47,
            -16,  37,  43,  40,  35,  50,  37,  -2,
             -4,   5,  19,  50,  37,  37,   7,  -2,
             -6,  13,  13,  26,  34,  12,  10,   4,
              0,  15,  15,  15,  14,  27,  18,  10,
              4,  15,  16,   0,   7,  21,  33,   1,
            -33,  -3, -14, -21, -13, -12, -39, -21,
        ];

        #[rustfmt::skip]
        let eg_bishop_table: [i32; 64] = [
            -14, -21, -11,  -8, -7,  -9, -17, -24,
             -8,  -4,   7, -12, -3, -13,  -4, -14,
              2,  -8,   0,  -1, -2,   6,   0,   4,
             -3,   9,  12,   9, 14,  10,   3,   2,
             -6,   3,  13,  19,  7,  10,  -3,  -9,
            -12,  -3,   8,  10, 13,   3,  -7, -15,
            -14, -18,  -7,  -1,  4,  -9, -15, -27,
            -23,  -9, -23,  -5, -9, -16,  -5, -17,
        ];

        #[rustfmt::skip]
        let mg_rook_table: [i32; 64] = [
            32,  42,  32,  51, 63,  9,  31,  43,
            27,  32,  58,  62, 80, 67,  26,  44,
            -5,  19,  26,  36, 17, 45,  61,  16,
           -24, -11,   7,  26, 24, 35,  -8, -20,
           -36, -26, -12,  -1,  9, -7,   6, -23,
           -45, -25, -16, -17,  3,  0,  -5, -33,
           -44, -16, -20,  -9, -1, 11,  -6, -71,
           -19, -13,   1,  17, 16,  7, -37, -26,
        ];

        #[rustfmt::skip]
        let eg_rook_table: [i32; 64] = [
            13, 10, 18, 15, 12,  12,   8,   5,
            11, 13, 13, 11, -3,   3,   8,   3,
             7,  7,  7,  5,  4,  -3,  -5,  -3,
             4,  3, 13,  1,  2,   1,  -1,   2,
             3,  5,  8,  4, -5,  -6,  -8, -11,
            -4,  0, -5, -1, -7, -12,  -8, -16,
            -6, -6,  0,  2, -9,  -9, -11,  -3,
            -9,  2,  3, -1, -5, -13,   4, -20,
        ];

        #[rustfmt::skip]
        let mg_queen_table: [i32; 64] = [
            -28,   0,  29,  12,  59,  44,  43,  45,
            -24, -39,  -5,   1, -16,  57,  28,  54,
            -13, -17,   7,   8,  29,  56,  47,  57,
            -27, -27, -16, -16,  -1,  17,  -2,   1,
             -9, -26,  -9, -10,  -2,  -4,   3,  -3,
            -14,   2, -11,  -2,  -5,   2,  14,   5,
            -35,  -8,  11,   2,   8,  15,  -3,   1,
             -1, -18,  -9,  10, -15, -25, -31, -50,
        ];

        #[rustfmt::skip]
        let eg_queen_table: [i32; 64] = [
            -9,  22,  22,  27,  27,  19,  10,  20,
           -17,  20,  32,  41,  58,  25,  30,   0,
           -20,   6,   9,  49,  47,  35,  19,   9,
             3,  22,  24,  45,  57,  40,  57,  36,
           -18,  28,  19,  47,  31,  34,  39,  23,
           -16, -27,  15,   6,   9,  17,  10,   5,
           -22, -23, -30, -16, -16, -23, -36, -32,
           -33, -28, -22, -43,  -5, -32, -20, -41,
        ];

        #[rustfmt::skip]
        let mg_king_table: [i32; 64] = [
            -65,  23,  16, -15, -56, -34,   2,  13,
             29,  -1, -20,  -7,  -8,  -4, -38, -29,
             -9,  24,   2, -16, -20,   6,  22, -22,
            -17, -20, -12, -27, -30, -25, -14, -36,
            -49,  -1, -27, -39, -46, -44, -33, -51,
            -14, -14, -22, -46, -44, -30, -15, -27,
              1,   7,  -8, -64, -43, -16,   9,   8,
            -15,  36,  12, -54,   8, -28,  24,  14,
        ];

        #[rustfmt::skip]
        let eg_king_table: [i32; 64] = [
            -74, -35, -18, -18, -11,  15,   4, -17,
            -12,  17,  14,  17,  17,  38,  23,  11,
             10,  17,  23,  15,  20,  45,  44,  13,
             -8,  22,  24,  27,  26,  33,  26,   3,
            -18,  -4,  21,  24,  27,  23,   9, -11,
            -19,  -3,  11,  21,  23,  16,   7,  -9,
            -27, -11,   4,  13,  14,   4,  -5, -17,
            -53, -34, -21, -11, -28, -14, -24, -43
        ];

        let mg_pawn_table_b = get_game_phase_table(&mg_pawn_table, mg_piece_weight(Piece::Pawn));
        let eg_pawn_table_b = get_game_phase_table(&eg_pawn_table, eg_piece_weight(Piece::Pawn));
        let mg_knight_table_b =
            get_game_phase_table(&mg_knight_table, mg_piece_weight(Piece::Knight));
        let eg_knight_table_b =
            get_game_phase_table(&eg_knight_table, eg_piece_weight(Piece::Knight));
        let mg_bishop_table_b =
            get_game_phase_table(&mg_bishop_table, mg_piece_weight(Piece::Bishop));
        let eg_bishop_table_b =
            get_game_phase_table(&eg_bishop_table, eg_piece_weight(Piece::Bishop));
        let mg_rook_table_b = get_game_phase_table(&mg_rook_table, mg_piece_weight(Piece::Rook));
        let eg_rook_table_b = get_game_phase_table(&eg_rook_table, eg_piece_weight(Piece::Rook));
        let mg_queen_table_b = get_game_phase_table(&mg_queen_table, mg_piece_weight(Piece::Queen));
        let eg_queen_table_b = get_game_phase_table(&eg_queen_table, eg_piece_weight(Piece::Queen));
        let mg_king_table_b = get_game_phase_table(&mg_king_table, mg_piece_weight(Piece::King));
        let eg_king_table_b = get_game_phase_table(&eg_king_table, eg_piece_weight(Piece::King));

        Engine {
            mg_pawn_table_w: mirror_table(&mg_pawn_table_b),
            eg_pawn_table_w: mirror_table(&eg_pawn_table_b),
            mg_knight_table_w: mirror_table(&mg_king_table_b),
            eg_knight_table_w: mirror_table(&eg_knight_table_b),
            mg_bishop_table_w: mirror_table(&mg_bishop_table_b),
            eg_bishop_table_w: mirror_table(&eg_bishop_table_b),
            mg_rook_table_w: mirror_table(&mg_rook_table_b),
            eg_rook_table_w: mirror_table(&eg_rook_table_b),
            mg_queen_table_w: mirror_table(&mg_queen_table_b),
            eg_queen_table_w: mirror_table(&eg_queen_table_b),
            mg_king_table_w: mirror_table(&mg_king_table_b),
            eg_king_table_w: mirror_table(&eg_king_table_b),
            mg_pawn_table_b,
            eg_pawn_table_b,
            mg_knight_table_b,
            eg_knight_table_b,
            mg_bishop_table_b,
            eg_bishop_table_b,
            mg_rook_table_b,
            eg_rook_table_b,
            mg_queen_table_b,
            eg_queen_table_b,
            mg_king_table_b,
            eg_king_table_b,
            best_move: None,
        }
    }

    pub fn iter_deepening(&mut self, board: &mut Board, max_depth: usize) -> Option<MoveData> {
        let start = Instant::now();

        for depth in 1..=max_depth {
            let m = self.root_alpha_beta(board, depth);
            self.best_move = m.clone();

            println!("info depth {}", depth);

            if start.elapsed() >= Duration::from_secs(60) {
                return self.best_move.clone();
            }
        }

        self.best_move.clone()
    }

    pub fn root_alpha_beta(&mut self, board: &mut Board, depth: usize) -> Option<MoveData> {
        let mut alpha = i32::MIN + 1;
        let beta = i32::MAX - 1;
        let mut best_move: Option<MoveData> = None;

        let mut moves = board.get_pseudo_legal_moves();

        
        if let Some(m) = &self.best_move {
            moves.sort_unstable_by_key(|x| {
                if x == m {
                    return -10000;
                }
                if let Some(cap) = x.capture {
                    return x.piece as i32 - cap as i32;
                }
                10000
            })
        }

        for m in &moves {
            board.make_move(&m);
            if !board.is_king_left_in_check() {
                let score = -self.alpha_beta(board, depth - 1, -beta, -alpha);

                if score > alpha {
                    alpha = score;
                    best_move = Some(m.clone());
                }
            }
            board.unmake_move(&m);
        }
        best_move
    }

    pub fn alpha_beta(
        &mut self,
        board: &mut Board,
        depth: usize,
        mut alpha: i32,
        beta: i32,
    ) -> i32 {
        if depth == 0 {
            //return self.quiesce(board, alpha, beta);
            return self.static_eval(board);
        }

        let mut moves = board.get_pseudo_legal_moves();
        let mut legal_moves = 0;

        let mut max = i32::MIN;
        for m in moves.drain(..) {
            board.make_move(&m);
            if board.is_king_left_in_check() {
                board.unmake_move(&m)
            } else {
                legal_moves += 1;
                let score = -self.alpha_beta(board, depth - 1, -beta, -alpha);
                board.unmake_move(&m);

                if score >= beta {
                    return beta;
                }

                if score > max {
                    max = score;
                }

                alpha = alpha.max(score);
            }
        }

        if legal_moves == 0 {
            if board.is_player_mated() {
                return -100000 - depth as i32;
            }
            return 0;
        }

        max
    }

    #[allow(dead_code)]
    fn quiesce(&self, board: &mut Board, mut alpha: i32, beta: i32) -> i32 {
        let stand_pat = self.static_eval(board);

        if stand_pat >= beta {
            return beta;
        }
        if alpha < stand_pat {
            alpha = stand_pat;
        }

        let moves = board.get_pseudo_legal_moves();

        for m in moves {
            match m.capture {
                Some(_) => {
                    board.make_move(&m);
                    let score = -self.quiesce(board, -beta, -alpha);
                    board.unmake_move(&m);
                    if score >= beta {
                        return beta;
                    }
                    if score > alpha {
                        alpha = score;
                    }
                }
                None => (),
            }
        }
        alpha
    }

    fn static_eval(&self, board: &Board) -> i32 {
        let mut mg_score = 0;
        let mut eg_score = 0;

        let who_to_move = match board.is_white_to_move {
            true => 1,
            false => -1,
        };

        let mut game_phase_score = 0;

        for i in &board.pieces_w {
            let p = board.data[*i].unwrap();

            game_phase_score += self.score_game_phase_pieces(p.1);

            mg_score += self.get_mg_score(p, *i) * who_to_move;
            eg_score += self.get_eg_score(p, *i) * who_to_move;
        }

        for i in &board.pieces_b {
            let p = board.data[*i].unwrap();

            game_phase_score += self.score_game_phase_pieces(p.1);

            mg_score -= self.get_mg_score(p, *i) * who_to_move;
            eg_score -= self.get_eg_score(p, *i) * who_to_move;
        }

        if game_phase_score > 24 {
            game_phase_score = 24;
        }

        ((game_phase_score * mg_score) + ((24 - game_phase_score) * eg_score)) / 24
    }

    fn score_game_phase_pieces(&self, piece: Piece) -> i32 {
        match piece {
            Piece::Pawn => 0,
            Piece::Knight => 1,
            Piece::Bishop => 1,
            Piece::Rook => 2,
            Piece::Queen => 4,
            Piece::King => 0,
        }
    }

    fn get_mg_score(&self, piece: (Color, Piece), square: usize) -> i32 {
        match piece {
            (Color::Black, Piece::Pawn) => self.mg_pawn_table_b[square],
            (Color::Black, Piece::Knight) => self.mg_knight_table_b[square],
            (Color::Black, Piece::Bishop) => self.mg_bishop_table_b[square],
            (Color::Black, Piece::Rook) => self.mg_rook_table_b[square],
            (Color::Black, Piece::Queen) => self.mg_queen_table_b[square],
            (Color::Black, Piece::King) => self.mg_king_table_b[square],
            (Color::White, Piece::Pawn) => self.mg_pawn_table_w[square],
            (Color::White, Piece::Knight) => self.mg_knight_table_w[square],
            (Color::White, Piece::Bishop) => self.mg_bishop_table_w[square],
            (Color::White, Piece::Rook) => self.mg_rook_table_w[square],
            (Color::White, Piece::Queen) => self.mg_queen_table_w[square],
            (Color::White, Piece::King) => self.mg_king_table_w[square],
        }
    }

    fn get_eg_score(&self, piece: (Color, Piece), square: usize) -> i32 {
        match piece {
            (Color::Black, Piece::Pawn) => self.eg_pawn_table_b[square],
            (Color::Black, Piece::Knight) => self.eg_knight_table_b[square],
            (Color::Black, Piece::Bishop) => self.eg_bishop_table_b[square],
            (Color::Black, Piece::Rook) => self.eg_rook_table_b[square],
            (Color::Black, Piece::Queen) => self.eg_queen_table_b[square],
            (Color::Black, Piece::King) => self.eg_king_table_b[square],
            (Color::White, Piece::Pawn) => self.eg_pawn_table_w[square],
            (Color::White, Piece::Knight) => self.eg_knight_table_w[square],
            (Color::White, Piece::Bishop) => self.eg_bishop_table_w[square],
            (Color::White, Piece::Rook) => self.eg_rook_table_w[square],
            (Color::White, Piece::Queen) => self.eg_queen_table_w[square],
            (Color::White, Piece::King) => self.eg_king_table_w[square],
        }
    }
}
