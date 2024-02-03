pub use squares::Square;
use std::vec;

mod cache;
mod squares;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(PartialEq)]
enum OccupiedStatus {
    OccupiedOwnColor,
    OccupiedOpponentColor,
    Unoccupied,
}

#[derive(Debug, PartialEq)]
pub struct MoveData {
    pub start_pos: usize,
    pub end_pos: usize,
    pub piece: Piece,
    pub move_type: MoveType,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Capture(pub Option<Piece>);

#[derive(Debug, PartialEq)]
pub enum MoveType {
    Regular(Capture),
    Castling,
    EnPassant,
    QueenPromotion(Capture),
    RookPromotion(Capture),
    BishopPromotion(Capture),
    KnightPromotion(Capture),
}

#[derive(Debug, Clone, PartialEq)]
struct IrreversibleBoardState {
    castling_w_00: bool,
    castling_w_000: bool,
    castling_b_00: bool,
    castling_b_000: bool,
    half_moves: usize,
    ep_target: Option<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum GameStatus {
    WhiteWin,
    BlackWin,
    StaleMate,
    Ongoing,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    cache: cache::Cache,
    pub game_status: GameStatus,
    pub is_white_to_move: bool,
    pub data: [Option<(Color, Piece)>; 64],
    castling_w_00: bool,
    castling_w_000: bool,
    castling_b_00: bool,
    castling_b_000: bool,
    pub ep_target: Option<usize>,
    pub half_moves: usize,
    pub full_moves: usize,
    irreversible_board_state_stack: Vec<IrreversibleBoardState>,
}

impl Board {
    // Starting pos: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    pub fn from_fen(fen: &str) -> Self {
        let mut data: [Option<(Color, Piece)>; 64] = [None; 64];

        let cache_builder = cache::Cache::builder();
        let cache = cache_builder.build();

        let mut sections = fen.split(" ");
        let pieces = sections
            .next()
            .expect("Invalid FEN string - piece positioning");

        let mut idx: usize = 56;
        for row in pieces.split("/") {
            for c in row.chars() {
                if c.is_ascii_digit() {
                    idx += c.to_digit(10).expect("Should parse as digit") as usize;
                } else {
                    let piece = match c {
                        'P' => (Color::White, Piece::Pawn),
                        'N' => (Color::White, Piece::Knight),
                        'B' => (Color::White, Piece::Bishop),
                        'R' => (Color::White, Piece::Rook),
                        'Q' => (Color::White, Piece::Queen),
                        'K' => (Color::White, Piece::King),
                        'p' => (Color::Black, Piece::Pawn),
                        'n' => (Color::Black, Piece::Knight),
                        'b' => (Color::Black, Piece::Bishop),
                        'r' => (Color::Black, Piece::Rook),
                        'q' => (Color::Black, Piece::Queen),
                        'k' => (Color::Black, Piece::King),
                        _ => panic!("FEN string invalid"),
                    };
                    data[idx] = Some(piece);
                    idx += 1;
                }
            }
            if idx % 8 != 0 {
                panic!("Rank did not contain 8 squares when parsing FEN")
            }

            // Only move down when above the first rank
            if idx > 8 {
                idx = idx.saturating_sub(16);
            }
        }
        println!("Finish parse index {}", idx);

        let side_to_move = sections.next().expect("Invalid FEN string - side to move");
        let is_white_to_move = match side_to_move {
            "w" => true,
            "b" => false,
            _ => panic!("FEN string side to move data invalid"),
        };

        let castling_rights = sections
            .next()
            .expect("Invalid FEN string - castling rights");
        let castling_w_00 = castling_rights.contains("K");
        let castling_w_000 = castling_rights.contains("Q");
        let castling_b_00 = castling_rights.contains("k");
        let castling_b_000 = castling_rights.contains("q");

        let en_passant_target_str = sections.next().expect("Invalid FEN string - en passant");
        let ep_target = match en_passant_target_str {
            "-" => None,
            "a3" => Some(Square::A3),
            "b3" => Some(Square::B3),
            "c3" => Some(Square::C3),
            "d3" => Some(Square::D3),
            "e3" => Some(Square::E3),
            "f3" => Some(Square::F3),
            "g3" => Some(Square::G3),
            "h3" => Some(Square::H3),
            "a6" => Some(Square::A6),
            "b6" => Some(Square::B6),
            "c6" => Some(Square::C6),
            "d6" => Some(Square::D6),
            "e6" => Some(Square::E6),
            "f6" => Some(Square::F6),
            "g6" => Some(Square::G6),
            "h6" => Some(Square::H6),
            _ => panic!("Invalid FEN string - en passant"),
        };

        let half_moves: usize = sections
            .next()
            .expect("Invalid FEN string - half move clock")
            .parse()
            .expect("Half move clock should parse");

        let full_moves: usize = sections
            .next()
            .expect("Invalid FEN string - full move counter")
            .parse()
            .expect("Full move counter should parse");

        Board {
            cache,
            is_white_to_move,
            data,
            castling_w_00,
            castling_w_000,
            castling_b_00,
            castling_b_000,
            ep_target,
            half_moves,
            full_moves,
            game_status: GameStatus::Ongoing,
            irreversible_board_state_stack: vec![],
        }
    }

    pub fn print(&self) {
        for rank_idx in (0..8).rev() {
            print!("{} ", rank_idx + 1);
            for file_idx in 0..8 {
                if let Some(p) = self.data[rank_idx * 8 + file_idx] {
                    match p {
                        (Color::White, Piece::Pawn) => print!("P"),
                        (Color::White, Piece::Rook) => print!("R"),
                        (Color::White, Piece::Knight) => print!("N"),
                        (Color::White, Piece::Bishop) => print!("B"),
                        (Color::White, Piece::Queen) => print!("Q"),
                        (Color::White, Piece::King) => print!("K"),
                        (Color::Black, Piece::Pawn) => print!("p"),
                        (Color::Black, Piece::Rook) => print!("r"),
                        (Color::Black, Piece::Knight) => print!("n"),
                        (Color::Black, Piece::Bishop) => print!("b"),
                        (Color::Black, Piece::Queen) => print!("q"),
                        (Color::Black, Piece::King) => print!("k"),
                    }
                } else {
                    print!(".")
                }
            }
            print!("\n");
        }
        println!("  --------\n  abcdefgh\n");
        println!("Is white to move: {}", self.is_white_to_move);
        println!(
            "Castling ability -> K: {}, Q: {}, k: {}, q: {}",
            self.castling_w_00, self.castling_w_000, self.castling_b_00, self.castling_b_000,
        );
        println!("En passant target square: {:?}", self.ep_target);
        println!(
            "Halfmove Clock: {} Fullmove counter: {}",
            self.half_moves, self.full_moves
        );
    }

    pub fn print_moves(&self, moves: &Vec<MoveData>) {
        println!("Available moves ({}):", moves.len());
        for m in moves {
            let from = self.get_square_from_idx(m.start_pos);
            let to = self.get_square_from_idx(m.end_pos);
            print!("{}{} ({:?}) ", from, to, m.piece);
        }
        println!("\n");
    }

    fn is_unoccupied(&self, pos: usize) -> bool {
        self.data[pos] == None
    }

    fn get_occupied_status(&self, pos: usize) -> OccupiedStatus {
        if let Some(p) = self.data[pos] {
            match (self.is_white_to_move, p.0) {
                (true, Color::White) => OccupiedStatus::OccupiedOwnColor,
                (false, Color::Black) => OccupiedStatus::OccupiedOwnColor,
                _ => OccupiedStatus::OccupiedOpponentColor,
            }
        } else {
            OccupiedStatus::Unoccupied
        }
    }

    fn is_position_threatened(&self, pos: usize) -> bool {
        let opponent_color = match self.is_white_to_move {
            true => Color::Black,
            false => Color::White,
        };

        // Rook and queen threat
        for ray in &self.cache.rook_rays[pos] {
            for ray_pos in ray {
                match self.data[*ray_pos] {
                    Some(piece) => match (piece.0 == opponent_color, piece.1) {
                        (true, Piece::Rook) => return true,
                        (true, Piece::Queen) => return true,
                        _ => break,
                    },
                    None => (),
                }
            }
        }

        // Bishop and queen threat
        for ray in &self.cache.bishop_rays[pos] {
            for ray_pos in ray {
                match self.data[*ray_pos] {
                    Some(piece) => match (piece.0 == opponent_color, piece.1) {
                        (true, Piece::Bishop) => return true,
                        (true, Piece::Queen) => return true,
                        _ => break,
                    },
                    None => (),
                }
            }
        }

        // Knight threat
        for threat_position in &self.cache.knight_targets[pos] {
            match self.data[*threat_position] {
                Some(piece) => match (piece.0 == opponent_color, piece.1) {
                    (true, Piece::Knight) => return true,
                    _ => (),
                },
                None => (),
            }
        }

        // Pawn threat
        if self.is_white_to_move && self.is_position_threatened_by_black_pawn(pos) {
            return true;
        }

        if !self.is_white_to_move && self.is_position_threatened_by_white_pawn(pos) {
            return true;
        }

        // Opposite king threat
        let neighbor_positions = &self.cache.neighbor_targets[pos];
        for neighbor_pos in neighbor_positions {
            if self.data[*neighbor_pos] == Some((opponent_color, Piece::King)) {
                return false;
            }
        }

        false
    }

    fn is_position_threatened_by_white_pawn(&self, pos: usize) -> bool {
        let rank_idx = pos as isize / 8;
        let file_idx = pos as isize % 8;

        if file_idx - 1 >= 0 && rank_idx - 1 >= 1 {
            let left_down_pos = (rank_idx - 1) as usize * 8 + (file_idx - 1) as usize;
            if self.data[left_down_pos] == Some((Color::White, Piece::Pawn)) {
                return true;
            }
        }

        if file_idx + 1 < 8 && rank_idx - 1 >= 1 {
            let right_down_pos = (rank_idx - 1) as usize * 8 + (file_idx + 1) as usize;
            if self.data[right_down_pos] == Some((Color::White, Piece::Pawn)) {
                return true;
            }
        }

        false
    }

    fn is_position_threatened_by_black_pawn(&self, pos: usize) -> bool {
        let rank_idx = pos as usize / 8;
        let file_idx = pos as isize % 8;

        if file_idx - 1 >= 0 && rank_idx + 1 < 7 {
            let left_up_pos = (rank_idx + 1) * 8 + file_idx as usize - 1;
            if self.data[left_up_pos] == Some((Color::Black, Piece::Pawn)) {
                return true;
            }
        }

        if file_idx + 1 < 8 && rank_idx + 1 < 7 {
            let right_up_pos = (rank_idx + 1) * 8 + file_idx as usize + 1;
            if self.data[right_up_pos] == Some((Color::Black, Piece::Pawn)) {
                return true;
            }
        }

        false
    }

    pub fn make_move(&mut self, instr: &MoveData) {
        self.irreversible_board_state_stack
            .push(IrreversibleBoardState {
                castling_w_00: self.castling_w_00,
                castling_w_000: self.castling_w_000,
                castling_b_00: self.castling_b_00,
                castling_b_000: self.castling_b_000,
                half_moves: self.half_moves,
                ep_target: self.ep_target,
            });

        if let Some(piece) = self.data[instr.start_pos] {
            match instr.move_type {
                MoveType::Regular(cap) => {
                    self.data[instr.end_pos] = Some(piece);
                    self.data[instr.start_pos] = None;

                    // Reset half move clock on pawn move or capture
                    // Increment it otherwise
                    if piece.1 == Piece::Pawn || cap != Capture(None) {
                        self.half_moves = 0;
                    } else {
                        self.half_moves += 1;
                    }
                }
                MoveType::Castling => {
                    self.data[instr.end_pos] = Some(piece);
                    self.data[instr.start_pos] = None;

                    // Increment half move clock
                    self.half_moves += 1;

                    match instr.end_pos {
                        Square::C1 => {
                            self.data[Square::D1] = Some((Color::White, Piece::Rook));
                            self.data[Square::A1] = None;
                        }
                        Square::G1 => {
                            self.data[Square::F1] = Some((Color::White, Piece::Rook));
                            self.data[Square::H1] = None;
                        }
                        Square::C8 => {
                            self.data[Square::D8] = Some((Color::Black, Piece::Rook));
                            self.data[Square::A8] = None;
                        }
                        Square::G8 => {
                            self.data[Square::F8] = Some((Color::Black, Piece::Rook));
                            self.data[Square::H8] = None;
                        }
                        _ => panic!("Invalid castling destination square"),
                    }
                }

                // Reset half move clock on en passant
                MoveType::EnPassant => {
                    self.data[instr.end_pos] = Some(piece);
                    self.data[instr.start_pos] = None;

                    // Handle en passant capture
                    if self.is_white_to_move {
                        self.data[instr.end_pos - 8] = None
                    } else {
                        self.data[instr.end_pos + 8] = None
                    }
                    self.half_moves = 0;
                }

                // Reset half move clock for the 4 pawn promotions
                MoveType::QueenPromotion(_) => {
                    self.data[instr.end_pos] = Some((piece.0, Piece::Queen));
                    self.data[instr.start_pos] = None;
                    self.half_moves = 0;
                }
                MoveType::RookPromotion(_) => {
                    self.data[instr.end_pos] = Some((piece.0, Piece::Rook));
                    self.data[instr.start_pos] = None;
                    self.half_moves = 0;
                }
                MoveType::BishopPromotion(_) => {
                    self.data[instr.end_pos] = Some((piece.0, Piece::Bishop));
                    self.data[instr.start_pos] = None;
                    self.half_moves = 0;
                }
                MoveType::KnightPromotion(_) => {
                    self.data[instr.end_pos] = Some((piece.0, Piece::Knight));
                    self.data[instr.start_pos] = None;
                    self.half_moves = 0;
                }
            };
        }
        if !self.is_white_to_move {
            self.full_moves += 1;
        }

        // Set en passant target square on double pawn push
        if instr.piece == Piece::Pawn && instr.start_pos.abs_diff(instr.end_pos) == 16 {
            self.ep_target = match self.is_white_to_move {
                true => Some(instr.start_pos + 8),
                false => Some(instr.start_pos - 8),
            }
        } else {
            self.ep_target = None;
        }

        // Castling
        if instr.piece == Piece::King {
            if self.is_white_to_move {
                self.castling_w_00 = false;
                self.castling_w_000 = false;
            } else {
                self.castling_b_00 = false;
                self.castling_b_000 = false;
            }
        }

        if instr.piece == Piece::Rook {
            match (
                self.is_white_to_move,
                instr.start_pos == 0,
                instr.start_pos == 7,
                instr.start_pos == 56,
                instr.start_pos == 63,
            ) {
                (true, true, _, _, _) => self.castling_w_000 = false,
                (true, _, true, _, _) => self.castling_w_00 = false,
                (false, _, _, true, _) => self.castling_b_000 = false,
                (false, _, _, _, true) => self.castling_b_00 = false,
                _ => (),
            }
        }

        self.is_white_to_move = !self.is_white_to_move;
    }

    pub fn unmake_move(&mut self, last_move: &MoveData) {
        let irreversible_state = self.irreversible_board_state_stack.pop();

        if let Some(s) = irreversible_state {
            // Reverse color to move since we are going back one move
            self.is_white_to_move = !self.is_white_to_move;

            let color_to_move = match self.is_white_to_move {
                true => Color::White,
                false => Color::Black,
            };
            let opponent_color = match self.is_white_to_move {
                true => Color::Black,
                false => Color::White,
            };

            // Restore the "irreversible" board state
            self.castling_w_00 = s.castling_w_00;
            self.castling_w_000 = s.castling_w_000;
            self.castling_b_00 = s.castling_b_000;
            self.castling_b_000 = s.castling_b_000;
            self.ep_target = s.ep_target;
            self.half_moves = s.half_moves;

            // Rewind the full move counter when reversing a black move
            if !self.is_white_to_move {
                self.full_moves -= 1;
            }

            match last_move.move_type {
                MoveType::Regular(cap) => {
                    self.data[last_move.start_pos] = Some((color_to_move, last_move.piece));
                    if let Some(p) = cap.0 {
                        self.data[last_move.end_pos] = Some((opponent_color, p));
                    } else {
                        self.data[last_move.end_pos] = None;
                    }
                }
                MoveType::Castling => match (last_move.start_pos, last_move.end_pos) {
                    (Square::E1, Square::C1) => {
                        self.data[Square::A1] = Some((Color::White, Piece::Rook));
                        self.data[Square::E1] = Some((Color::White, Piece::King));
                        self.data[Square::C1] = None;
                        self.data[Square::D1] = None;
                    }
                    (Square::E1, Square::G1) => {
                        self.data[Square::H1] = Some((Color::White, Piece::Rook));
                        self.data[Square::E1] = Some((Color::White, Piece::King));
                        self.data[Square::F1] = None;
                        self.data[Square::G1] = None;
                    }
                    (Square::E8, Square::C8) => {
                        self.data[Square::A8] = Some((Color::Black, Piece::Rook));
                        self.data[Square::E8] = Some((Color::Black, Piece::King));
                        self.data[Square::C8] = None;
                        self.data[Square::D8] = None;
                    }
                    (Square::E8, Square::G8) => {
                        self.data[Square::H8] = Some((Color::Black, Piece::Rook));
                        self.data[Square::E8] = Some((Color::Black, Piece::King));
                        self.data[Square::F8] = None;
                        self.data[Square::G8] = None;
                    }
                    _ => panic!("Attempt to reverse invalid castling"),
                },
                MoveType::EnPassant => {
                    self.data[last_move.start_pos] = Some((color_to_move, Piece::Pawn));
                    self.data[last_move.end_pos] = None;

                    // Replace en passant captured piece
                    if self.is_white_to_move {
                        self.data[last_move.end_pos - 8] = Some((Color::Black, Piece::Pawn));
                    } else {
                        self.data[last_move.end_pos + 8] = Some((Color::White, Piece::Pawn));
                    }
                }
                MoveType::QueenPromotion(cap) => {
                    self.unmake_promotion_move(last_move, &cap);
                }
                MoveType::RookPromotion(cap) => {
                    self.unmake_promotion_move(last_move, &cap);
                }
                MoveType::BishopPromotion(cap) => {
                    self.unmake_promotion_move(last_move, &cap);
                }
                MoveType::KnightPromotion(cap) => {
                    self.unmake_promotion_move(last_move, &cap);
                }
            }
        } else {
            panic!("Attempt to unmake move without irreversible board state stored on stack")
        }
    }

    fn unmake_promotion_move(&mut self, last_move: &MoveData, cap: &Capture) {
        let color_to_move = match self.is_white_to_move {
            true => Color::White,
            false => Color::Black,
        };
        let opponent_color = match self.is_white_to_move {
            true => Color::Black,
            false => Color::White,
        };

        self.data[last_move.start_pos] = Some((color_to_move, Piece::Pawn));
        if let Some(p) = cap.0 {
            self.data[last_move.end_pos] = Some((opponent_color, p));
        } else {
            self.data[last_move.end_pos] = None;
        }
    }

    fn get_potential_move_positions(&self) -> Vec<(usize, Color, Piece)> {
        let target_color = match self.is_white_to_move {
            true => Color::White,
            false => Color::Black,
        };
        let mut positions: Vec<(usize, Color, Piece)> = vec![];

        for (idx, pos) in self.data.iter().enumerate() {
            if let Some(p) = pos {
                if p.0 == target_color {
                    positions.push((idx, p.0, p.1));
                }
            }
        }
        positions
    }

    fn get_square_from_idx(&self, idx: usize) -> String {
        let file_idx = idx % 8;
        let rank = 1 + idx / 8;
        let files: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];
        files[file_idx].to_owned() + &rank.to_string()
    }

    fn add_promotion_moves(
        &self,
        start_pos: usize,
        end_pos: usize,
        capture: Capture,
        moves: &mut Vec<MoveData>,
    ) {
        let queen_promotion = MoveData {
            start_pos,
            end_pos,
            piece: Piece::Pawn,
            move_type: MoveType::QueenPromotion(capture),
        };
        let rook_promotion = MoveData {
            move_type: MoveType::RookPromotion(capture),
            ..queen_promotion
        };
        let bishop_promotion = MoveData {
            move_type: MoveType::BishopPromotion(capture),
            ..queen_promotion
        };
        let knight_promotion = MoveData {
            move_type: MoveType::KnightPromotion(capture),
            ..queen_promotion
        };
        moves.push(queen_promotion);
        moves.push(rook_promotion);
        moves.push(bishop_promotion);
        moves.push(knight_promotion);
    }

    fn get_white_pawn_moves(&self, pos: usize) -> Vec<MoveData> {
        let mut moves: Vec<MoveData> = vec![];
        if pos + 8 < Square::A8 && self.is_unoccupied(pos + 8) {
            moves.push(MoveData {
                start_pos: pos,
                end_pos: pos + 8,
                piece: Piece::Pawn,
                move_type: MoveType::Regular(Capture(None)),
            });
        }
        if pos <= Square::H2 && self.is_unoccupied(pos + 8) && self.is_unoccupied(pos + 16) {
            moves.push(MoveData {
                start_pos: pos,
                end_pos: pos + 16,
                piece: Piece::Pawn,
                move_type: MoveType::Regular(Capture(None)),
            });
        }

        // Pawn promotion without capture
        if pos >= Square::A7 && self.is_unoccupied(pos + 8) {
            self.add_promotion_moves(pos, pos + 8, Capture(None), &mut moves);
        }

        // Pawn captures
        let file_idx = pos % 8;
        let capture_rank_idx = pos / 8 + 1;

        // Left up pawn captures (looking at board from White's position)
        let left_file_idx = file_idx as isize - 1;

        if left_file_idx >= 0 {
            let left_capture_pos = capture_rank_idx * 8 + left_file_idx as usize;

            // Regular pawn capture
            if capture_rank_idx < 7 {
                if self.get_occupied_status(left_capture_pos)
                    == OccupiedStatus::OccupiedOpponentColor
                {
                    moves.push(MoveData {
                        start_pos: pos,
                        end_pos: left_capture_pos,
                        piece: Piece::Pawn,
                        move_type: MoveType::Regular(Capture(Some(
                            self.data[left_capture_pos].unwrap().1,
                        ))),
                    });
                }
            } else if capture_rank_idx == 7 {
                // Pawn promotion with capture to the left
                if self.get_occupied_status(left_capture_pos)
                    == OccupiedStatus::OccupiedOpponentColor
                {
                    self.add_promotion_moves(
                        pos,
                        left_capture_pos,
                        Capture(Some(self.data[left_capture_pos].unwrap().1)),
                        &mut moves,
                    );
                }
            };

            // En passant capture to the left
            if let Some(i) = self.ep_target {
                if i == left_capture_pos {
                    moves.push(MoveData {
                        start_pos: pos,
                        end_pos: left_capture_pos,
                        piece: Piece::Pawn,
                        move_type: MoveType::EnPassant,
                    });
                }
            }
        }

        // Right up pawn capture (looking at board from White's position)
        let right_file_idx = file_idx + 1;

        if right_file_idx < 8 {
            let right_capture_pos = capture_rank_idx * 8 + right_file_idx as usize;

            if capture_rank_idx < 7 {
                if self.get_occupied_status(right_capture_pos)
                    == OccupiedStatus::OccupiedOpponentColor
                {
                    moves.push(MoveData {
                        start_pos: pos,
                        end_pos: right_capture_pos,
                        piece: Piece::Pawn,
                        move_type: MoveType::Regular(Capture(Some(
                            self.data[right_capture_pos].unwrap().1,
                        ))),
                    });
                }
            } else if capture_rank_idx == 7 {
                // Pawn promotion with capture to the left
                if self.get_occupied_status(right_capture_pos)
                    == OccupiedStatus::OccupiedOpponentColor
                {
                    self.add_promotion_moves(
                        pos,
                        right_capture_pos,
                        Capture(Some(self.data[right_capture_pos].unwrap().1)),
                        &mut moves,
                    );
                }
            };

            if let Some(i) = self.ep_target {
                if i == right_capture_pos {
                    moves.push(MoveData {
                        start_pos: pos,
                        end_pos: right_capture_pos,
                        piece: Piece::Pawn,
                        move_type: MoveType::EnPassant,
                    });
                }
            }
        }

        moves
    }

    fn get_black_pawn_moves(&self, pos: usize) -> Vec<MoveData> {
        let mut moves: Vec<MoveData> = vec![];
        if pos >= 8 && self.is_unoccupied(pos - 8) {
            moves.push(MoveData {
                start_pos: pos,
                end_pos: pos - 8,
                piece: Piece::Pawn,
                move_type: MoveType::Regular(Capture(None)),
            });
        }
        if pos >= 48 && self.is_unoccupied(pos - 8) && self.is_unoccupied(pos - 16) {
            moves.push(MoveData {
                start_pos: pos,
                end_pos: pos - 16,
                piece: Piece::Pawn,
                move_type: MoveType::Regular(Capture(None)),
            });
        }

        // Pawn promotion without capture
        if pos <= Square::H2 && self.is_unoccupied(pos - 8) {
            self.add_promotion_moves(pos, pos - 8, Capture(None), &mut moves);
        }

        // Pawn captures
        let file_idx = pos % 8;
        let capture_rank_idx = pos as isize / 8 - 1;

        // Left down pawn capture (looking at board from White's position)
        let left_file_idx = file_idx as isize - 1;

        if left_file_idx >= 0 {
            let left_capture_pos = (capture_rank_idx * 8 + left_file_idx) as usize;

            // Regular pawn capture
            if capture_rank_idx > 0 {
                if self.get_occupied_status(left_capture_pos)
                    == OccupiedStatus::OccupiedOpponentColor
                {
                    moves.push(MoveData {
                        start_pos: pos,
                        end_pos: left_capture_pos,
                        piece: Piece::Pawn,
                        move_type: MoveType::Regular(Capture(Some(
                            self.data[left_capture_pos].unwrap().1,
                        ))),
                    });
                }
            } else if capture_rank_idx == 0 {
                // Pawn promotion with capture to the left
                if self.get_occupied_status(left_capture_pos)
                    == OccupiedStatus::OccupiedOpponentColor
                {
                    self.add_promotion_moves(
                        pos,
                        left_capture_pos,
                        Capture(Some(self.data[left_capture_pos].unwrap().1)),
                        &mut moves,
                    );
                }
            };

            // En passant capture to the left
            if let Some(i) = self.ep_target {
                if i == left_capture_pos {
                    moves.push(MoveData {
                        start_pos: pos,
                        end_pos: left_capture_pos,
                        piece: Piece::Pawn,
                        move_type: MoveType::EnPassant,
                    });
                }
            }
        }

        // Right down pawn capture (looking at board from White's position)
        let right_file_idx = file_idx + 1;

        if right_file_idx <= 7 {
            let right_capture_pos = capture_rank_idx as usize * 8 + right_file_idx;

            // Regular pawn capture to the right
            if capture_rank_idx > 0 {
                if self.get_occupied_status(right_capture_pos)
                    == OccupiedStatus::OccupiedOpponentColor
                {
                    moves.push(MoveData {
                        start_pos: pos,
                        end_pos: right_capture_pos,
                        piece: Piece::Pawn,
                        move_type: MoveType::Regular(Capture(Some(
                            self.data[right_capture_pos].unwrap().1,
                        ))),
                    });
                }
            } else if capture_rank_idx == 0 {
                // Pawn promotion with capture to the right
                if self.get_occupied_status(right_capture_pos)
                    == OccupiedStatus::OccupiedOpponentColor
                {
                    self.add_promotion_moves(
                        pos,
                        right_capture_pos,
                        Capture(Some(self.data[right_capture_pos].unwrap().1)),
                        &mut moves,
                    );
                }
            };

            // En passant capture to the right
            if let Some(i) = self.ep_target {
                if i == right_capture_pos {
                    moves.push(MoveData {
                        start_pos: pos,
                        end_pos: right_capture_pos,
                        piece: Piece::Pawn,
                        move_type: MoveType::EnPassant,
                    });
                }
            }
        }

        moves
    }

    fn get_rook_moves(&self, pos: usize) -> Vec<MoveData> {
        let mut new_positions: Vec<MoveData> = vec![];

        let rook_rays = &self.cache.rook_rays[pos];
        for ray in rook_rays {
            for ray_pos in ray {
                match self.get_occupied_status(*ray_pos) {
                    OccupiedStatus::OccupiedOwnColor => break,
                    OccupiedStatus::OccupiedOpponentColor => {
                        new_positions.push(MoveData {
                            start_pos: pos,
                            end_pos: *ray_pos,
                            piece: Piece::Rook,
                            move_type: MoveType::Regular(Capture(Some(
                                self.data[*ray_pos].unwrap().1,
                            ))),
                        });
                        break;
                    }
                    OccupiedStatus::Unoccupied => new_positions.push(MoveData {
                        start_pos: pos,
                        end_pos: *ray_pos,
                        piece: Piece::Rook,
                        move_type: MoveType::Regular(Capture(None)),
                    }),
                }
            }
        }

        new_positions
    }

    fn get_knight_moves(&self, pos: usize) -> Vec<MoveData> {
        let mut new_positions: Vec<MoveData> = vec![];

        for target in &self.cache.knight_targets[pos] {
            match self.get_occupied_status(*target) {
                OccupiedStatus::OccupiedOwnColor => (),
                OccupiedStatus::OccupiedOpponentColor => new_positions.push(MoveData {
                    start_pos: pos,
                    end_pos: *target,
                    piece: Piece::Knight,
                    move_type: MoveType::Regular(Capture(Some(self.data[*target].unwrap().1))),
                }),
                OccupiedStatus::Unoccupied => new_positions.push(MoveData {
                    start_pos: pos,
                    end_pos: *target,
                    piece: Piece::Knight,
                    move_type: MoveType::Regular(Capture(None)),
                }),
            }
        }

        new_positions
    }

    fn get_bishop_moves(&self, pos: usize) -> Vec<MoveData> {
        let mut new_positions: Vec<MoveData> = vec![];

        for ray in &self.cache.bishop_rays[pos] {
            for ray_pos in ray {
                match self.get_occupied_status(*ray_pos) {
                    OccupiedStatus::OccupiedOwnColor => break,
                    OccupiedStatus::OccupiedOpponentColor => {
                        new_positions.push(MoveData {
                            start_pos: pos,
                            end_pos: *ray_pos,
                            piece: Piece::Bishop,
                            move_type: MoveType::Regular(Capture(Some(
                                self.data[*ray_pos].unwrap().1,
                            ))),
                        });
                        break;
                    }
                    OccupiedStatus::Unoccupied => new_positions.push(MoveData {
                        start_pos: pos,
                        end_pos: *ray_pos,
                        piece: Piece::Bishop,
                        move_type: MoveType::Regular(Capture(None)),
                    }),
                }
            }
        }

        new_positions
    }

    fn get_queen_moves(&self, pos: usize) -> Vec<MoveData> {
        let mut new_moves: Vec<MoveData> = self.get_rook_moves(pos);
        new_moves.extend(self.get_bishop_moves(pos));
        for m in &mut new_moves {
            m.piece = Piece::Queen;
        }
        new_moves
    }

    fn get_king_moves(&self, pos: usize) -> Vec<MoveData> {
        let mut new_positions: Vec<MoveData> = vec![];
        let neighbor_positions = &self.cache.neighbor_targets[pos];

        for neighbor_pos in neighbor_positions {
            match self.get_occupied_status(*neighbor_pos) {
                OccupiedStatus::OccupiedOwnColor => (),
                OccupiedStatus::OccupiedOpponentColor => new_positions.push(MoveData {
                    start_pos: pos,
                    end_pos: *neighbor_pos,
                    piece: Piece::King,
                    move_type: MoveType::Regular(Capture(Some(
                        self.data[*neighbor_pos].unwrap().1,
                    ))),
                }),
                OccupiedStatus::Unoccupied => new_positions.push(MoveData {
                    start_pos: pos,
                    end_pos: *neighbor_pos,
                    piece: Piece::King,
                    move_type: MoveType::Regular(Capture(None)),
                }),
            }
        }
        new_positions
    }

    fn get_king_pos(&self) -> usize {
        let target_color = match self.is_white_to_move {
            true => Color::White,
            false => Color::Black,
        };
        match self
            .data
            .iter()
            .position(|&p| p == Some((target_color, Piece::King)))
        {
            Some(v) => return v,
            None => panic!("A valid board should always have 2 kings"),
        }
    }

    fn get_castling_moves(&self) -> Vec<MoveData> {
        let mut moves: Vec<MoveData> = vec![];

        if self.is_white_to_move {
            if self.castling_w_000 {
                if [1, 2, 3].iter().all(|s| self.data[*s] == None)
                    && [2, 3, 4].iter().all(|s| !self.is_position_threatened(*s))
                {
                    moves.push(MoveData {
                        start_pos: 4,
                        end_pos: 2,
                        piece: Piece::King,
                        move_type: MoveType::Castling,
                    })
                }
            }
            if self.castling_w_00 {
                if [5, 6].iter().all(|s| self.data[*s] == None)
                    && [4, 5, 6].iter().all(|s| !self.is_position_threatened(*s))
                {
                    moves.push(MoveData {
                        start_pos: 4,
                        end_pos: 6,
                        piece: Piece::King,
                        move_type: MoveType::Castling,
                    })
                }
            }
        } else {
            if self.castling_b_000 {
                if [57, 58, 59].iter().all(|s| self.data[*s] == None)
                    && [58, 59, 60]
                        .iter()
                        .all(|s| !self.is_position_threatened(*s))
                {
                    moves.push(MoveData {
                        start_pos: 60,
                        end_pos: 58,
                        piece: Piece::King,
                        move_type: MoveType::Castling,
                    })
                }
            }
            if self.castling_b_00 {
                if [61, 62].iter().all(|s| self.data[*s] == None)
                    && [60, 61, 62]
                        .iter()
                        .all(|s| !self.is_position_threatened(*s))
                {
                    moves.push(MoveData {
                        start_pos: 60,
                        end_pos: 62,
                        piece: Piece::King,
                        move_type: MoveType::Castling,
                    })
                }
            }
        }

        moves
    }

    fn is_legal_move(&mut self, move_data: &MoveData) -> bool {
        let temp = self.data[move_data.end_pos];
        self.data[move_data.end_pos] = self.data[move_data.start_pos];
        self.data[move_data.start_pos] = None;

        if move_data.move_type == MoveType::EnPassant {
            if self.is_white_to_move {
                self.data[move_data.end_pos - 8] = None;
            } else {
                self.data[move_data.end_pos + 8] = None;
            }

            let is_legal = !self.is_position_threatened(self.get_king_pos());
            self.data[move_data.start_pos] = self.data[move_data.end_pos];
            self.data[move_data.end_pos] = temp;

            if self.is_white_to_move {
                self.data[move_data.end_pos - 8] = Some((Color::Black, Piece::Pawn));
            } else {
                self.data[move_data.end_pos + 8] = Some((Color::White, Piece::Pawn));
            }
            is_legal
        } else {
            let is_legal = !self.is_position_threatened(self.get_king_pos());
            self.data[move_data.start_pos] = self.data[move_data.end_pos];
            self.data[move_data.end_pos] = temp;
            is_legal
        }
    }

    pub fn get_valid_moves(&mut self) -> Vec<MoveData> {
        let mut moves: Vec<MoveData> = vec![];
        let positions = self.get_potential_move_positions();

        for position in positions {
            let position_moves: Vec<MoveData> = match (position.1, position.2) {
                (Color::White, Piece::Pawn) => self.get_white_pawn_moves(position.0),
                (Color::Black, Piece::Pawn) => self.get_black_pawn_moves(position.0),
                (_, Piece::Rook) => self.get_rook_moves(position.0),
                (_, Piece::Knight) => self.get_knight_moves(position.0),
                (_, Piece::Bishop) => self.get_bishop_moves(position.0),
                (_, Piece::Queen) => self.get_queen_moves(position.0),
                (_, Piece::King) => self.get_king_moves(position.0),
            };
            for m in position_moves {
                if self.is_legal_move(&m) {
                    moves.push(m);
                }
            }
        }

        moves.extend(self.get_castling_moves());

        if self.half_moves >= 100 && !moves.is_empty() {
            self.game_status = GameStatus::StaleMate;
        }

        if moves.is_empty() {
            self.update_game_status()
        }

        moves
    }

    fn update_game_status(&mut self) {
        let king_pos = self.get_king_pos();
        let is_mated = self.is_position_threatened(king_pos);
        self.game_status = match (is_mated, self.is_white_to_move) {
            (true, true) => GameStatus::BlackWin,
            (true, false) => GameStatus::WhiteWin,
            _ => GameStatus::StaleMate,
        }
    }
}
