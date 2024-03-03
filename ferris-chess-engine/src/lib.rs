use ferris_chess_board::{Board, Color, MoveData, MoveType, Piece};
use regex::Regex;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

pub struct GoCommand {
    wtime: usize,
    btime: usize,
    movestogo: usize,
    max_depth: usize,
}

impl GoCommand {
    pub fn new(go_input: &String) -> Self {
        // Set some default values in case go command doesn't include them
        let mut wtime: usize = 10000;
        let mut btime: usize = 10000;
        let mut movestogo = 40;

        let wtime_re = Regex::new(r"wtime \d*").unwrap();
        let btime_re = Regex::new(r"btime \d*").unwrap();
        let movestogo_re = Regex::new(r"movestogo \d*").unwrap();

        println!("Parsing regex for: {}", go_input);

        if let Some(wtime_match) = wtime_re.find(&go_input) {
            wtime = wtime_match.as_str().split_ascii_whitespace().nth(1).unwrap().parse().unwrap();
        }

        if let Some(btime_match) = btime_re.find(&go_input) {
            btime = btime_match.as_str().split_ascii_whitespace().nth(1).unwrap().parse().unwrap();
        }

        if let Some(movestogo_match) = movestogo_re.find(&go_input) {
            movestogo = movestogo_match.as_str().split_ascii_whitespace().nth(1).unwrap().parse().unwrap();
        }

        println!("Got go command: wtime {} btime {} movestogo {}", wtime, btime, movestogo);

        GoCommand {
            wtime,
            btime,
            movestogo,
            max_depth: 10,
        }
    }
}

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

    mvv_lva_table: HashMap<(Piece, Piece), i32>,
    is_stopped: bool,
    stop_time: Instant,
}

pub struct SearchInfo {
    depth: usize,
    nodes: usize,
    time: usize,
    move_data: MoveData,
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

        let mvv_lva_table = HashMap::from([
            ((Piece::King, Piece::Pawn), -6),
            ((Piece::King, Piece::Knight), -5),
            ((Piece::King, Piece::Bishop), -4),
            ((Piece::King, Piece::Rook), -3),
            ((Piece::King, Piece::Queen), -2),
            ((Piece::King, Piece::King), -1),
            ((Piece::Queen, Piece::Pawn), 0),
            ((Piece::Queen, Piece::Knight), 1),
            ((Piece::Queen, Piece::Bishop), 2),
            ((Piece::Queen, Piece::Rook), 3),
            ((Piece::Queen, Piece::Queen), 4),
            ((Piece::Queen, Piece::King), 5),
            ((Piece::Rook, Piece::Pawn), 6),
            ((Piece::Rook, Piece::Knight), 7),
            ((Piece::Rook, Piece::Bishop), 8),
            ((Piece::Rook, Piece::Rook), 9),
            ((Piece::Rook, Piece::Queen), 10),
            ((Piece::Rook, Piece::King), 11),
            ((Piece::Bishop, Piece::Pawn), 12),
            ((Piece::Bishop, Piece::Knight), 13),
            ((Piece::Bishop, Piece::Bishop), 14),
            ((Piece::Bishop, Piece::Rook), 15),
            ((Piece::Bishop, Piece::Queen), 16),
            ((Piece::Bishop, Piece::King), 17),
            ((Piece::Knight, Piece::Pawn), 18),
            ((Piece::Knight, Piece::Knight), 19),
            ((Piece::Knight, Piece::Bishop), 20),
            ((Piece::Knight, Piece::Rook), 21),
            ((Piece::Knight, Piece::Queen), 22),
            ((Piece::Knight, Piece::King), 23),
            ((Piece::Pawn, Piece::Pawn), 24),
            ((Piece::Pawn, Piece::Knight), 25),
            ((Piece::Pawn, Piece::Bishop), 26),
            ((Piece::Pawn, Piece::Rook), 27),
            ((Piece::Pawn, Piece::Queen), 28),
            ((Piece::Pawn, Piece::King), 29),
        ]);

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
            mvv_lva_table,
            is_stopped: false,
            stop_time: Instant::now(),
        }
    }

    pub fn stop(&mut self) {
        self.is_stopped = true;
    }

    pub fn new_game(&mut self) {
        self.is_stopped = false;
    }

    fn init_time(&mut self, board: &Board, go_cmd: &GoCommand) {
        self.stop_time = Instant::now()
            + Duration::from_millis(match board.is_white_to_move {
                true => go_cmd.wtime - 1000,
                false => go_cmd.btime - 1000,
            } as u64 / go_cmd.movestogo as u64)
    }

    pub fn iter_deepening(&mut self, board: &mut Board, go_cmd: &GoCommand) -> MoveData {
        self.init_time(board, go_cmd);

        let mut info: Option<SearchInfo> = None;

        for depth in 1..=go_cmd.max_depth {
            if let Some(search_info) = self.root_alpha_beta(board, depth)
            {
                println!(
                    "info depth {} nodes {} time {}",
                    search_info.depth, search_info.nodes, search_info.time
                );
                info = Some(search_info);
            }

            if self.is_stopped {
                break;
            }
        }

        info.unwrap().move_data
    }

    fn mvv_lva(&self, moves: &mut Vec<MoveData>) {
        moves.sort_unstable_by_key(|x| {
            if let Some(cap) = x.capture {
                return *self.mvv_lva_table.get(&(x.piece, cap)).unwrap();
            }
            10000
        })
    }

    pub fn root_alpha_beta(&mut self, board: &mut Board, depth: usize) -> Option<SearchInfo> {
        let start: Instant = Instant::now();
        let mut nodes = 0;

        let mut alpha = i32::MIN + 1;
        let beta = i32::MAX - 1;
        let mut search_info: Option<SearchInfo> = None;
        let mut legal_moves = 0;

        let mut moves = board.get_pseudo_legal_moves();
        self.mvv_lva(&mut moves);

        for m in &moves {
            board.make_move(&m);
            if !board.is_king_left_in_check() {
                let score = -self.alpha_beta(board, depth - 1, -beta, -alpha, &mut nodes);
                legal_moves += 1;

                if score > alpha {
                    alpha = score;
                    search_info = Some(SearchInfo {
                        depth,
                        nodes,
                        time: start.elapsed().as_millis() as usize,
                        move_data: m.clone(),
                    });
                }
            }
            board.unmake_move(&m);

            if self.is_stopped || Instant::now() > self.stop_time {
                return None
            }
        }

        if legal_moves == 0 {
            if board.is_player_mated() {
                return None;
            }
        }

        search_info
    }

    pub fn alpha_beta(
        &mut self,
        board: &mut Board,
        depth: usize,
        mut alpha: i32,
        beta: i32,
        nodes: &mut usize,
    ) -> i32 {
        *nodes += 1;

        if depth == 0 {
            return self.quiesce(board, alpha, beta, nodes);
            //return self.static_eval(board);
        }

        let mut moves = board.get_pseudo_legal_moves();
        self.mvv_lva(&mut moves);

        let mut legal_moves = 0;

        let mut max = i32::MIN;
        for m in moves.drain(..) {
            board.make_move(&m);
            if board.is_king_left_in_check() {
                board.unmake_move(&m)
            } else {
                legal_moves += 1;
                let score = -self.alpha_beta(board, depth - 1, -beta, -alpha, nodes);
                board.unmake_move(&m);

                if score >= beta {
                    return beta;
                }

                if score > max {
                    max = score;
                }

                alpha = alpha.max(score);
            }
            if self.is_stopped || Instant::now() > self.stop_time {
                return max;
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

    fn is_prom_move(&self, m: &MoveData) -> bool {
        m.move_type == MoveType::QueenPromotion
            || m.move_type == MoveType::RookPromotion
            || m.move_type == MoveType::BishopPromotion
            || m.move_type == MoveType::KnightPromotion
    }

    fn quiesce(
        &self,
        board: &mut Board,
        mut alpha: i32,
        beta: i32,
        nodes: &mut usize,
    ) -> i32 {
        *nodes += 1;
        let stand_pat = self.static_eval(board);

        if stand_pat >= beta {
            return beta;
        }
        if alpha < stand_pat {
            alpha = stand_pat;
        }

        let mut moves = board.get_pseudo_legal_moves();

        // Add basic sorting of captures
        self.mvv_lva(&mut moves);

        for m in moves {
            if let Some(cap) = m.capture {
                // Delta pruning
                if stand_pat + cap as i32 + 200 < alpha && !self.is_prom_move(&m) {
                    continue;
                }

                board.make_move(&m);
                let score = -self.quiesce(board, -beta, -alpha, nodes);
                board.unmake_move(&m);

                if score >= beta {
                    return beta;
                }
                if score > alpha {
                    alpha = score;
                }
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
