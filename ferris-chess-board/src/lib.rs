use std::vec;
pub use squares::Square;

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

#[derive(Debug, PartialEq)]
pub enum MoveType {
    Regular,
    Castling,
    EnPassant,
    QueenPromotion,
    RookPromotion,
    BishopPromotion,
    KnightPromotion,
}

#[derive(Debug)]
pub enum GameStatus {
    WhiteWin,
    BlackWin,
    StaleMate,
    Ongoing,
}

#[derive(Debug)]
pub struct Board {
    pub game_status: GameStatus,
    pub is_white_to_move: bool,
    pub data: [Option<(Color, Piece)>; 64],
    can_castle_w_king_side: bool,
    can_castle_w_queen_side: bool,
    can_castle_b_king_side: bool,
    can_castle_b_queen_side: bool,
    en_passant_target: Option<usize>,
    half_moves: usize,
    full_moves: usize,
}

impl Board {
    // Starting pos: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    pub fn from_fen(fen: &str) -> Self {
        let mut data: [Option<(Color, Piece)>; 64] = [None; 64];
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
            idx = idx.saturating_sub(16);
        }

        let side_to_move = sections.next().expect("Invalid FEN string - side to move");
        let is_white_to_move = match side_to_move {
            "w" => true,
            "b" => false,
            _ => panic!("FEN string side to move data invalid"),
        };

        let castling_rights = sections
            .next()
            .expect("Invalid FEN string - castling rights");
        let can_castle_w_king_side = castling_rights.contains("K");
        let can_castle_w_queen_side = castling_rights.contains("Q");
        let can_castle_b_king_side = castling_rights.contains("k");
        let can_castle_b_queen_side = castling_rights.contains("q");

        let en_passant_target_str = sections.next().expect("Invalid FEN string - en passant");
        let en_passant_target = match en_passant_target_str {
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
            is_white_to_move,
            data,
            can_castle_w_king_side,
            can_castle_w_queen_side,
            can_castle_b_king_side,
            can_castle_b_queen_side,
            en_passant_target,
            half_moves,
            full_moves,
            game_status: GameStatus::Ongoing,
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
            self.can_castle_w_king_side,
            self.can_castle_w_queen_side,
            self.can_castle_b_king_side,
            self.can_castle_b_queen_side
        );
        println!("En passant target square: {:?}", self.en_passant_target);
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
        for ray in self.get_rook_rays(pos) {
            for ray_pos in ray {
                match self.data[ray_pos] {
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
        for ray in self.get_bishop_rays(pos) {
            for ray_pos in ray {
                match self.data[ray_pos] {
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
        for threat_position in self.get_knight_targets(pos) {
            match self.data[threat_position] {
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
        let neighbor_positions = self.get_neighbor_positions(pos);
        for neighbor_pos in neighbor_positions {
            if self.data[neighbor_pos] == Some((opponent_color, Piece::King)) {
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
        if let Some(piece) = self.data[instr.start_pos] {
            match instr.move_type {
                MoveType::Regular => {
                    self.data[instr.end_pos] = Some(piece);
                    self.data[instr.start_pos] = None;
                }
                MoveType::Castling => {
                    self.data[instr.end_pos] = Some(piece);
                    self.data[instr.start_pos] = None;
                    match instr.end_pos {
                        Square::C1 => {
                            self.data[Square::D1] = Some((Color::White, Piece::Rook));
                            self.data[Square::A1] = None;
                        },
                        Square::G1 => {
                            self.data[Square::F1] = Some((Color::White, Piece::Rook));
                            self.data[Square::H1] = None;
                        },
                        Square::C8 => {
                            self.data[Square::D8] = Some((Color::Black, Piece::Rook));
                            self.data[Square::A8] = None;
                        },
                        Square::G8 => {
                            self.data[Square::F8] = Some((Color::Black, Piece::Rook));
                            self.data[Square::H8] = None;
                        },
                        _ => panic!("Invalid castling destination square")
                    }
                },
                MoveType::EnPassant => {
                    self.data[instr.end_pos] = Some(piece);
                    self.data[instr.start_pos] = None;

                    // Handle en passant capture
                    if self.is_white_to_move {
                        self.data[instr.end_pos - 8] = None
                    } else {
                        self.data[instr.end_pos + 8] = None
                    }
                }
                MoveType::QueenPromotion => {
                    self.data[instr.end_pos] = Some((piece.0, Piece::Queen));
                    self.data[instr.start_pos] = None;
                }
                MoveType::RookPromotion => {
                    self.data[instr.end_pos] = Some((piece.0, Piece::Rook));
                    self.data[instr.start_pos] = None;
                }
                MoveType::BishopPromotion => {
                    self.data[instr.end_pos] = Some((piece.0, Piece::Bishop));
                    self.data[instr.start_pos] = None;
                }
                MoveType::KnightPromotion => {
                    self.data[instr.end_pos] = Some((piece.0, Piece::Knight));
                    self.data[instr.start_pos] = None;
                }
            };
        }
        if !self.is_white_to_move {
            self.full_moves += 1;
        }

        // Set en passant target square on double pawn push
        if instr.piece == Piece::Pawn && instr.start_pos.abs_diff(instr.end_pos) == 16 {
            self.en_passant_target = match self.is_white_to_move {
                true => Some(instr.start_pos + 8),
                false => Some(instr.start_pos - 8),
            }
        } else {
            self.en_passant_target = None;
        }

        // Castling
        if instr.piece == Piece::King {
            if self.is_white_to_move {
                self.can_castle_w_king_side = false;
                self.can_castle_w_queen_side = false;
            } else {
                self.can_castle_b_king_side = false;
                self.can_castle_b_queen_side = false;
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
                (true, true, _, _, _) => self.can_castle_w_queen_side = false,
                (true, _, true, _, _) => self.can_castle_w_king_side = false,
                (false, _, _, true, _) => self.can_castle_b_queen_side = false,
                (false, _, _, _, true) => self.can_castle_b_king_side = false,
                _ => (),
            }
        }

        self.is_white_to_move = !self.is_white_to_move;
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

    fn add_promotion_moves(&self, start_pos: usize, end_pos: usize, moves: &mut Vec<MoveData>) {
        let queen_promotion = MoveData {
            start_pos,
            end_pos,
            piece: Piece::Pawn,
            move_type: MoveType::QueenPromotion,
        };
        let rook_promotion = MoveData {
            move_type: MoveType::RookPromotion,
            ..queen_promotion
        };
        let bishop_promotion = MoveData {
            move_type: MoveType::BishopPromotion,
            ..queen_promotion
        };
        let knight_promotion = MoveData {
            move_type: MoveType::KnightPromotion,
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
                move_type: MoveType::Regular,
            });
        }
        if pos <= Square::H2 && self.is_unoccupied(pos + 8) && self.is_unoccupied(pos + 16) {
            moves.push(MoveData {
                start_pos: pos,
                end_pos: pos + 16,
                piece: Piece::Pawn,
                move_type: MoveType::Regular,
            });
        }

        if pos >= Square::A7 && self.is_unoccupied(pos + 8) {
            self.add_promotion_moves(pos, pos + 8, &mut moves);
        }

        let capture_rank_idx = pos / 8 + 1;
        let file_idx = pos % 8;

        // Left up pawn capture (looking at board from White's position)
        let left_file_idx = file_idx as isize - 1;

        if left_file_idx >= 0 && capture_rank_idx < 8 {
            let capture_pos = capture_rank_idx * 8 + left_file_idx as usize;
            if self.get_occupied_status(capture_pos) == OccupiedStatus::OccupiedOpponentColor {
                moves.push(MoveData {
                    start_pos: pos,
                    end_pos: capture_pos,
                    piece: Piece::Pawn,
                    move_type: MoveType::Regular,
                });
            } else if let Some(i) = self.en_passant_target {
                if i == capture_pos {
                    moves.push(MoveData {
                        start_pos: pos,
                        end_pos: capture_pos,
                        piece: Piece::Pawn,
                        move_type: MoveType::EnPassant,
                    });
                }
            }
        }

        // Right up pawn capture (looking at board from White's position)
        let right_file_idx = file_idx + 1;

        if right_file_idx < 8 && capture_rank_idx < 8 {
            let capture_pos = capture_rank_idx * 8 + right_file_idx;
            if self.get_occupied_status(capture_pos) == OccupiedStatus::OccupiedOpponentColor {
                moves.push(MoveData {
                    start_pos: pos,
                    end_pos: capture_pos,
                    piece: Piece::Pawn,
                    move_type: MoveType::Regular,
                });
            } else if let Some(i) = self.en_passant_target {
                if i == capture_pos {
                    moves.push(MoveData {
                        start_pos: pos,
                        end_pos: capture_pos,
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
                move_type: MoveType::Regular,
            });
        }
        if pos >= 48 && self.is_unoccupied(pos - 8) && self.is_unoccupied(pos - 16) {
            moves.push(MoveData {
                start_pos: pos,
                end_pos: pos - 16,
                piece: Piece::Pawn,
                move_type: MoveType::Regular,
            });
        }

        if pos <= Square::H2 && self.is_unoccupied(pos - 8) {
            self.add_promotion_moves(pos, pos - 8, &mut moves);
        }

        // Pawn captures
        let capture_rank_idx = pos as isize / 8 - 1;
        let file_idx = pos % 8;

        // Left down pawn capture (looking at board from White's position)
        let left_file_idx = file_idx as isize - 1;

        if left_file_idx >= 0 && capture_rank_idx >= 0 {
            let capture_pos = (capture_rank_idx * 8 + left_file_idx) as usize;
            if self.get_occupied_status(capture_pos) == OccupiedStatus::OccupiedOpponentColor {
                moves.push(MoveData {
                    start_pos: pos,
                    end_pos: capture_pos,
                    piece: Piece::Pawn,
                    move_type: MoveType::Regular,
                });
            } else if let Some(i) = self.en_passant_target {
                if i == capture_pos {
                    moves.push(MoveData {
                        start_pos: pos,
                        end_pos: capture_pos,
                        piece: Piece::Pawn,
                        move_type: MoveType::EnPassant,
                    });
                }
            }
        }

        // Right down pawn capture (looking at board from White's position)
        let right_file_idx = file_idx + 1;

        if right_file_idx < 8 && capture_rank_idx >= 0 {
            let capture_pos = capture_rank_idx as usize * 8 + right_file_idx;
            if self.get_occupied_status(capture_pos) == OccupiedStatus::OccupiedOpponentColor {
                moves.push(MoveData {
                    start_pos: pos,
                    end_pos: capture_pos,
                    piece: Piece::Pawn,
                    move_type: MoveType::Regular,
                });
            } else if let Some(i) = self.en_passant_target {
                if i == capture_pos {
                    moves.push(MoveData {
                        start_pos: pos,
                        end_pos: capture_pos,
                        piece: Piece::Pawn,
                        move_type: MoveType::EnPassant,
                    });
                }
            }
        }

        moves
    }

    fn get_rook_rays(&self, pos: usize) -> Vec<Vec<usize>> {
        let mut rook_rays: Vec<Vec<usize>> = vec![];

        let mut up: Vec<usize> = vec![];
        let mut up_pos = pos.checked_sub(8);
        while let Some(v) = up_pos {
            up.push(v);
            up_pos = v.checked_sub(8);
        }
        rook_rays.push(up);

        let mut down: Vec<usize> = vec![];
        let mut down_pos = pos + 8;
        while down_pos < 64 {
            down.push(down_pos);
            down_pos += 8;
        }
        rook_rays.push(down);

        let file_idx = pos % 8;

        let mut right: Vec<usize> = vec![];
        for p in (pos + 1)..(pos + 8 - file_idx) {
            right.push(p);
        }
        rook_rays.push(right);

        let mut left: Vec<usize> = vec![];
        for p in ((pos - file_idx)..pos).rev() {
            left.push(p);
        }
        rook_rays.push(left);

        rook_rays
    }

    fn get_rook_moves(&self, pos: usize) -> Vec<MoveData> {
        let mut new_positions: Vec<MoveData> = vec![];

        let rook_rays = self.get_rook_rays(pos);
        for ray in rook_rays {
            for ray_pos in ray {
                match self.get_occupied_status(ray_pos) {
                    OccupiedStatus::OccupiedOwnColor => break,
                    OccupiedStatus::OccupiedOpponentColor => {
                        new_positions.push(MoveData {
                            start_pos: pos,
                            end_pos: ray_pos,
                            piece: Piece::Rook,
                            move_type: MoveType::Regular,
                        });
                        break;
                    }
                    OccupiedStatus::Unoccupied => new_positions.push(MoveData {
                        start_pos: pos,
                        end_pos: ray_pos,
                        piece: Piece::Rook,
                        move_type: MoveType::Regular,
                    }),
                }
            }
        }

        new_positions
    }

    fn get_knight_targets(&self, pos: usize) -> Vec<usize> {
        let mut targets: Vec<usize> = vec![];
        let rank_idx = pos / 8;
        let file_idx = pos % 8;

        let offsets: [[isize; 2]; 8] = [
            [2, -1],
            [2, 1],
            [1, 2],
            [-1, 2],
            [-2, 1],
            [-2, -1],
            [-1, -2],
            [1, -2],
        ];
        for [rank_offset, file_offset] in offsets {
            let new_rank = rank_idx as isize + rank_offset;
            let new_file = file_idx as isize + file_offset;
            if new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
                let new_pos = new_rank as usize * 8 + new_file as usize;
                targets.push(new_pos);
            }
        }

        targets
    }

    fn get_knight_moves(&self, pos: usize) -> Vec<MoveData> {
        let mut new_positions: Vec<MoveData> = vec![];

        for target in self.get_knight_targets(pos) {
            match self.get_occupied_status(target) {
                OccupiedStatus::OccupiedOwnColor => (),
                _ => new_positions.push(MoveData {
                    start_pos: pos,
                    end_pos: target,
                    piece: Piece::Knight,
                    move_type: MoveType::Regular,
                }),
            }
        }

        new_positions
    }

    fn get_bishop_rays(&self, pos: usize) -> Vec<Vec<usize>> {
        let mut bishop_rays: Vec<Vec<usize>> = vec![];

        let mut down_left: Vec<usize> = vec![];
        let mut down_left_pos = pos.checked_sub(9);
        while let Some(v) = down_left_pos {
            if v % 8 == 7 {
                break;
            }
            down_left.push(v);
            down_left_pos = v.checked_sub(9);
        }
        bishop_rays.push(down_left);

        let mut down_right: Vec<usize> = vec![];
        let mut down_right_pos = pos.checked_sub(7);
        while let Some(v) = down_right_pos {
            if v % 8 == 0 {
                break;
            }
            down_right.push(v);
            down_right_pos = v.checked_sub(7);
        }
        bishop_rays.push(down_right);

        let mut up_right: Vec<usize> = vec![];
        let mut up_right_pos = pos + 9;
        while up_right_pos < 64 {
            if up_right_pos % 8 == 0 {
                break;
            }
            up_right.push(up_right_pos);
            up_right_pos += 9;
        }
        bishop_rays.push(up_right);

        let mut up_left: Vec<usize> = vec![];
        let mut up_left_pos = pos + 7;
        while up_left_pos < 64 {
            if up_left_pos % 8 == 7 {
                break;
            }
            up_left.push(up_left_pos);
            up_left_pos += 7;
        }
        bishop_rays.push(up_left);

        bishop_rays
    }

    fn get_bishop_moves(&self, pos: usize) -> Vec<MoveData> {
        let mut new_positions: Vec<MoveData> = vec![];

        let bishop_rays = self.get_bishop_rays(pos);
        for ray in bishop_rays {
            for ray_pos in ray {
                match self.get_occupied_status(ray_pos) {
                    OccupiedStatus::OccupiedOwnColor => break,
                    OccupiedStatus::OccupiedOpponentColor => {
                        new_positions.push(MoveData {
                            start_pos: pos,
                            end_pos: ray_pos,
                            piece: Piece::Bishop,
                            move_type: MoveType::Regular,
                        });
                        break;
                    }
                    OccupiedStatus::Unoccupied => new_positions.push(MoveData {
                        start_pos: pos,
                        end_pos: ray_pos,
                        piece: Piece::Bishop,
                        move_type: MoveType::Regular,
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

    fn get_neighbor_positions(&self, pos: usize) -> Vec<usize> {
        let mut new_positions: Vec<usize> = vec![];
        let rank_idx = pos / 8;
        let file_idx = pos % 8;

        let offsets: [[isize; 2]; 8] = [
            [1, -1],
            [1, 0],
            [1, 1],
            [0, -1],
            [0, 1],
            [-1, -1],
            [-1, 0],
            [-1, 1],
        ];
        for [rank_offset, file_offset] in offsets {
            let new_rank = rank_idx as isize + rank_offset;
            let new_file = file_idx as isize + file_offset;
            if new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
                let new_pos = new_rank as usize * 8 + new_file as usize;
                new_positions.push(new_pos);
            }
        }
        new_positions
    }

    fn get_king_moves(&self, pos: usize) -> Vec<MoveData> {
        let mut new_positions: Vec<MoveData> = vec![];
        let neighbor_positions = self.get_neighbor_positions(pos);

        for neighbor_pos in neighbor_positions {
            match self.get_occupied_status(neighbor_pos) {
                OccupiedStatus::OccupiedOwnColor => (),
                _ => new_positions.push(MoveData {
                    start_pos: pos,
                    end_pos: neighbor_pos,
                    piece: Piece::King,
                    move_type: MoveType::Regular,
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
            if self.can_castle_w_queen_side {
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
            if self.can_castle_w_king_side {
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
            if self.can_castle_b_queen_side {
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
            if self.can_castle_b_king_side {
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

        let is_legal = !self.is_position_threatened(self.get_king_pos());

        self.data[move_data.start_pos] = self.data[move_data.end_pos];
        self.data[move_data.end_pos] = temp;
        is_legal
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

