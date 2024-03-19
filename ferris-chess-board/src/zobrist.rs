use std::fs::File;

use rand::Rng;
use crate::{Board, Color, MoveData, Piece};

enum Castling {
    CastlingOO,
    CastlingOOO,
    Castlingoo,
    Castlingooo,
}

enum BoardFile {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
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

pub struct Zobrist {
    // 12*64: Piece/square combo
    // 1: Side to move is black
    // 4: Castling rights
    // 8: File of en passant target square
    board_rnd_nums: [u64; 12*64 + 1 + 4 + 8],
    hash: u64,
}

impl Zobrist {

    pub fn new(board: &Board) -> Self {
        let mut board_rnd_nums: [u64; 12*64+1+4+8] = [0; 781];
        board_rnd_nums[0] = 42;

        let mut hash = 0;

        for num in &mut board_rnd_nums {
            *num = rand::thread_rng().gen_range(0..=u64::MAX);
        }

        for (idx, piece) in board.data.iter().enumerate() {
            if let Some(p) = piece {
                let piece_offset = get_piece_idx(*p);
                hash ^= board_rnd_nums[idx * 12 + piece_offset];
            }
        }

        // Set black to move
        if board.black_to_move {
            hash ^= board_rnd_nums[12 * 64 + 1];
        }

        // Set initial castling rights
        for castling_offset in 0..4 {
            hash ^= board_rnd_nums[12 * 64 + 1 + castling_offset];
        }

        // Set ep target
        if let Some(_ep) = board.ep_target {

        }

        Zobrist { board_rnd_nums, hash }
    }

    pub fn invert_piece(&mut self, idx: usize, piece: (Color, Piece)) {
        self.hash ^= self.board_rnd_nums[idx * 12 + get_piece_idx(piece)];
    }

    pub fn invert_black_to_move(&mut self) {
        let idx = 12 * 64;
        self.hash ^= self.board_rnd_nums[idx];
    }

    pub fn invert_castling(&mut self, castling: Castling) {
        let idx = 12 * 64 + 1 + match castling {
            Castling::CastlingOO => 0,
            Castling::CastlingOOO => 1,
            Castling::Castlingoo => 2,
            Castling::Castlingooo => 3,
        };
        self.hash ^= self.board_rnd_nums[idx];
    }

    pub fn invert_ep_file(&mut self, ep_file: BoardFile) {
        let idx = 12 * 64 + 1 + 4 + match ep_file {
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

