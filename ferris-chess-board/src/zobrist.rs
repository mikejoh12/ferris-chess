use crate::BoardFile;

use crate::{Color, Piece};
use rand::Rng;

pub enum Castling {
    CastlingOO,
    CastlingOOO,
    Castlingoo,
    Castlingooo,
}

fn get_piece_idx(piece: (Color, Piece)) -> usize {
    match piece {
        (Color::White, Piece::Pawn) => 0,
        (Color::White, Piece::Knight) => 1,
        (Color::White, Piece::Bishop) => 2,
        (Color::White, Piece::Rook) => 3,
        (Color::White, Piece::Queen) => 4,
        (Color::White, Piece::King) => 5,
        (Color::Black, Piece::Pawn) => 6,
        (Color::Black, Piece::Knight) => 7,
        (Color::Black, Piece::Bishop) => 8,
        (Color::Black, Piece::Rook) => 9,
        (Color::Black, Piece::Queen) => 10,
        (Color::Black, Piece::King) => 11,
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Zobrist {
    // 12*64: Piece/square combo
    // 1: Side to move is black
    // 4: Castling rights
    // 8: File of en passant target square
    board_rnd_nums: [u64; 12 * 64 + 1 + 4 + 8],
    pub hash: u64,
}

pub struct ZobristData {
    pub board_data: [Option<(Color, Piece)>; 64],
    pub black_to_move: bool,
    pub castling_w_00: bool,
    pub castling_w_000: bool,
    pub castling_b_00: bool,
    pub castling_b_000: bool,
    pub ep_target: Option<BoardFile>,
}

impl Zobrist {
    pub fn new(z_data: ZobristData) -> Self {
        let mut board_rnd_nums: [u64; 12 * 64 + 1 + 4 + 8] = [0; 781];

        let mut hash = 0;

        for num in &mut board_rnd_nums {
            *num = rand::thread_rng().gen_range(0..=u64::MAX);
        }

        for (idx, piece) in z_data.board_data.iter().enumerate() {
            if let Some(p) = piece {
                let piece_offset = get_piece_idx(*p);
                hash ^= board_rnd_nums[idx * 12 + piece_offset];
            }
        }

        // Set black to move
        if z_data.black_to_move {
            hash ^= board_rnd_nums[12 * 64 + 1];
        }

        // Set initial castling rights
        if z_data.castling_w_00 {
            hash ^= board_rnd_nums[12 * 64 + 1 + 0];
        }
        if z_data.castling_w_000 {
            hash ^= board_rnd_nums[12 * 64 + 1 + 1];
        }
        if z_data.castling_w_00 {
            hash ^= board_rnd_nums[12 * 64 + 1 + 2];
        }
        if z_data.castling_w_000 {
            hash ^= board_rnd_nums[12 * 64 + 1 + 3];
        }

        // Set ep target
        if let Some(ep_file) = z_data.ep_target {
            let ep_file_offset = match ep_file {
                BoardFile::A => 0,
                BoardFile::B => 1,
                BoardFile::C => 2,
                BoardFile::D => 3,
                BoardFile::E => 4,
                BoardFile::F => 5,
                BoardFile::G => 6,
                BoardFile::H => 7,
            };
            hash ^= board_rnd_nums[12 * 64 + 1 + 4 + ep_file_offset]
        }

        Zobrist {
            board_rnd_nums,
            hash,
        }
    }

    pub fn invert_piece(&mut self, idx: usize, piece: (Color, Piece)) {
        self.hash ^= self.board_rnd_nums[idx * 12 + get_piece_idx(piece)];
    }

    pub fn invert_black_to_move(&mut self) {
        let idx = 12 * 64;
        self.hash ^= self.board_rnd_nums[idx];
    }

    pub fn invert_castling(&mut self, castling: Castling) {
        let idx = 12 * 64
            + 1
            + match castling {
                Castling::CastlingOO => 0,
                Castling::CastlingOOO => 1,
                Castling::Castlingoo => 2,
                Castling::Castlingooo => 3,
            };
        self.hash ^= self.board_rnd_nums[idx];
    }

    pub fn invert_ep_file(&mut self, ep_file: BoardFile) {
        let idx = 12 * 64
            + 1
            + 4
            + match ep_file {
                BoardFile::A => 0,
                BoardFile::B => 1,
                BoardFile::C => 2,
                BoardFile::D => 3,
                BoardFile::E => 4,
                BoardFile::F => 5,
                BoardFile::G => 6,
                BoardFile::H => 7,
            };
        self.hash ^= self.board_rnd_nums[idx];
    }
}
