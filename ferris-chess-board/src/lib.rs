use std::vec;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug)]
enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

enum OccupiedStatus {
    OccupiedOwnColor,
    OccupiedOpponentColor,
    Unoccupied,
}

#[derive(Debug)]
pub struct MoveData {
    pub start_pos: usize,
    pub end_pos: usize,
}

impl MoveData {
    pub fn get_uci_string(&self) -> &String {
        todo!()
    }
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
    is_white_to_move: bool,
    move_count: u32,
    data: [Option<(Color, Piece)>; 64],
    can_castle_white_king_side: bool,
    can_castle_white_queen_side: bool,
    can_castle_black_king_side: bool,
    can_castle_black_queen_side: bool,
}

impl Board {
    pub fn new() -> Self {
        let mut data: [Option<(Color, Piece)>; 64] = [None; 64];
        for i in 0..8 {
            data[8 + i] = Some((Color::White, Piece::Pawn));
            data[8 * 6 + i] = Some((Color::Black, Piece::Pawn));
        }
        data[0] = Some((Color::White, Piece::Rook));
        data[1] = Some((Color::White, Piece::Knight));
        data[2] = Some((Color::White, Piece::Bishop));
        data[3] = Some((Color::White, Piece::Queen));
        data[4] = Some((Color::White, Piece::King));
        data[5] = Some((Color::White, Piece::Bishop));
        data[6] = Some((Color::White, Piece::Knight));
        data[7] = Some((Color::White, Piece::Rook));
        data[56] = Some((Color::Black, Piece::Rook));
        data[57] = Some((Color::Black, Piece::Knight));
        data[58] = Some((Color::Black, Piece::Bishop));
        data[59] = Some((Color::Black, Piece::Queen));
        data[60] = Some((Color::Black, Piece::King));
        data[61] = Some((Color::Black, Piece::Bishop));
        data[62] = Some((Color::Black, Piece::Knight));
        data[63] = Some((Color::Black, Piece::Rook));
        Board {
            move_count: 1,
            is_white_to_move: true,
            data,
            can_castle_white_king_side: false,
            can_castle_white_queen_side: false,
            can_castle_black_king_side: false,
            can_castle_black_queen_side: false,
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
                        (Color::Black, Piece::Knight) => print!("k"),
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
    }

    fn is_unoccupied(&self, pos: usize) -> bool {
        if let Some(_) = self.data[pos] {
            return false;
        }
        true
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

    pub fn make_move(&mut self, instr: &MoveData) {
        if let Some(piece) = self.data[instr.start_pos] {
            self.data[instr.end_pos] = Some(piece);
            self.data[instr.start_pos] = None;
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

    fn get_white_pawn_moves(&self, pos: usize) -> Vec<usize> {
        let mut new_positions: Vec<usize> = vec![];
        if pos + 8 < 64 && self.is_unoccupied(pos + 8) {
            new_positions.push(pos + 8);
        }
        if pos <= 15 && self.is_unoccupied(pos + 8) && self.is_unoccupied(pos + 16) {
            new_positions.push(pos + 16);
        }

        // TODO: Pawn captures, promotion, en-passant
        new_positions
    }

    fn get_black_pawn_moves(&self, pos: usize) -> Vec<usize> {
        let mut new_positions: Vec<usize> = vec![];
        if pos >= 8 && self.is_unoccupied(pos - 8) {
            new_positions.push(pos - 8);
        }
        if pos >= 48 && self.is_unoccupied(pos - 8) && self.is_unoccupied(pos - 16) {
            new_positions.push(pos - 16);
        }

        // TODO: Pawn captures, promotion, en-passant
        new_positions
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

    fn get_rook_moves(&self, pos: usize) -> Vec<usize> {
        let mut new_positions: Vec<usize> = vec![];

        let rook_rays = self.get_rook_rays(pos);
        for ray in rook_rays {
            for ray_pos in ray {
                match self.get_occupied_status(ray_pos) {
                    OccupiedStatus::OccupiedOwnColor => break,
                    OccupiedStatus::OccupiedOpponentColor => {
                        new_positions.push(ray_pos);
                        break;
                    }
                    OccupiedStatus::Unoccupied => new_positions.push(ray_pos),
                }
            }
        }

        new_positions
    }

    fn get_knight_moves(pos: usize) -> Vec<usize> {
        todo!()
    }

    fn get_bishop_rays(&self, pos: usize) -> Vec<Vec<usize>> {
        let mut bishop_rays: Vec<Vec<usize>> = vec![];
        let file_idx = pos % 8;

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

    fn get_bishop_moves(&self, pos: usize) -> Vec<usize> {
        let mut new_positions: Vec<usize> = vec![];

        let bishop_rays = self.get_bishop_rays(pos);
        for ray in bishop_rays {
            for ray_pos in ray {
                match self.get_occupied_status(ray_pos) {
                    OccupiedStatus::OccupiedOwnColor => break,
                    OccupiedStatus::OccupiedOpponentColor => {
                        new_positions.push(ray_pos);
                        break;
                    }
                    OccupiedStatus::Unoccupied => new_positions.push(ray_pos),
                }
            }
        }

        new_positions
    }

    fn get_queen_moves(&self, pos: usize) -> Vec<usize> {
        let mut new_positions: Vec<usize> = self.get_rook_moves(pos);
        new_positions.extend(self.get_bishop_moves(pos));
        new_positions
    }

    fn get_king_moves(pos: usize) -> Vec<usize> {
        todo!()
    }

    pub fn get_valid_moves(&self) -> Vec<MoveData> {
        let mut moves: Vec<MoveData> = vec![];
        let positions = self.get_potential_move_positions();

        for position in positions {
            let move_targets: Vec<usize> = match (position.1, position.2) {
                (Color::White, Piece::Pawn) => self.get_white_pawn_moves(position.0),
                (Color::Black, Piece::Pawn) => self.get_black_pawn_moves(position.0),
                (_, Piece::Rook) => self.get_rook_moves(position.0),
                (_, Piece::Knight) => vec![],
                (_, Piece::Bishop) => self.get_bishop_moves(position.0),
                (_, Piece::Queen) => self.get_queen_moves(position.0),
                (_, Piece::King) => vec![],
            };
            for target in move_targets {
                moves.push(MoveData { 
                    start_pos: position.0, end_pos: target });
            }
        }
        moves
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
