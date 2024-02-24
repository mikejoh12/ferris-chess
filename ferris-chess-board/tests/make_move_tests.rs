#[cfg(test)]
mod make_move_tests {

    use ferris_chess_board::*;

    #[test]
    fn start_position_20_moves() {
        let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let result = board.get_valid_moves();
        assert_eq!(result.len(), 20);
    }

    #[test]
    fn castling_allowed_white_both_sides() {
        let mut board =
            Board::from_fen("r3k2r/ppp1nppp/2nbbq2/3pp3/3PP3/2NBBQ2/PPP1NPPP/R3K2R w KQkq - 10 8");
        let result = board.get_valid_moves();
        assert!(result.contains(&MoveData {
            start_pos: Square::E1,
            end_pos: Square::C1,
            piece: Piece::King,
            move_type: MoveType::Castling
        }));
        assert!(result.contains(&MoveData {
            start_pos: Square::E1,
            end_pos: Square::G1,
            piece: Piece::King,
            move_type: MoveType::Castling
        }));
    }

    #[test]
    fn castling_allowed_black_both_sides() {
        let mut board =
            Board::from_fen("r3k2r/ppp1nppp/2nbbq2/3pp3/3PP3/P1NBBQ2/1PP1NPPP/R3K2R b KQkq - 0 8");
        let result = board.get_valid_moves();
        assert!(result.contains(&MoveData {
            start_pos: Square::E8,
            end_pos: Square::C8,
            piece: Piece::King,
            move_type: MoveType::Castling
        }));
        assert!(result.contains(&MoveData {
            start_pos: Square::E8,
            end_pos: Square::G8,
            piece: Piece::King,
            move_type: MoveType::Castling
        }));
    }

    #[test]
    fn castling_white_king_side() {
        let mut board = Board::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1");
        let result = board.get_valid_moves();

        let white_king_castling = MoveData {
            start_pos: Square::E1,
            end_pos: Square::G1,
            piece: Piece::King,
            move_type: MoveType::Castling,
        };

        assert!(result.contains(&white_king_castling));
        board.make_move(&white_king_castling);
        assert_eq!(board.data[Square::G1], Some((Color::White, Piece::King)));
        assert_eq!(board.data[Square::F1], Some((Color::White, Piece::Rook)));
    }

    #[test]
    fn castling_white_queen_side() {
        let mut board = Board::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1");
        let result = board.get_valid_moves();

        let white_queen_castling = MoveData {
            start_pos: Square::E1,
            end_pos: Square::C1,
            piece: Piece::King,
            move_type: MoveType::Castling,
        };

        assert!(result.contains(&white_queen_castling));
        board.make_move(&white_queen_castling);
        assert_eq!(board.data[Square::C1], Some((Color::White, Piece::King)));
        assert_eq!(board.data[Square::D1], Some((Color::White, Piece::Rook)));
    }

    #[test]
    fn castling_black_king_side() {
        let mut board = Board::from_fen("r3k2r/8/8/8/8/8/8/R3K2R b KQkq - 0 1");
        let result = board.get_valid_moves();

        let black_king_castling = MoveData {
            start_pos: Square::E8,
            end_pos: Square::G8,
            piece: Piece::King,
            move_type: MoveType::Castling,
        };

        assert!(result.contains(&black_king_castling));
        board.make_move(&black_king_castling);
        assert_eq!(board.data[Square::G8], Some((Color::Black, Piece::King)));
        assert_eq!(board.data[Square::F8], Some((Color::Black, Piece::Rook)));
    }

    #[test]
    fn castling_black_queen_side() {
        let mut board = Board::from_fen("r3k2r/8/8/8/8/8/8/R3K2R b KQkq - 0 1");
        let result = board.get_valid_moves();

        let black_queen_castling = MoveData {
            start_pos: Square::E8,
            end_pos: Square::C8,
            piece: Piece::King,
            move_type: MoveType::Castling,
        };

        assert!(result.contains(&black_queen_castling));
        board.make_move(&black_queen_castling);
        assert_eq!(board.data[Square::C8], Some((Color::Black, Piece::King)));
        assert_eq!(board.data[Square::D8], Some((Color::Black, Piece::Rook)));
    }

    #[test]
    fn no_white_castling_king_crosses_attack_has_castling_rights() {
        let mut board = Board::from_fen("4k3/8/8/3r1r2/8/8/8/R3K2R w KQ - 1 1");
        let result = board.get_valid_moves();
        assert!(!result.contains(&MoveData {
            start_pos: Square::E1,
            end_pos: Square::C1,
            piece: Piece::King,
            move_type: MoveType::Castling
        }));
        assert!(!result.contains(&MoveData {
            start_pos: Square::E1,
            end_pos: Square::G1,
            piece: Piece::King,
            move_type: MoveType::Castling
        }));
    }

    #[test]
    fn no_black_castling_king_crosses_attack_has_castling_rights() {
        let mut board = Board::from_fen("r3k2r/8/8/8/3R1R2/8/8/4K3 b kq - 1 1");
        let result = board.get_valid_moves();
        assert!(!result.contains(&MoveData {
            start_pos: Square::E8,
            end_pos: Square::C8,
            piece: Piece::King,
            move_type: MoveType::Castling
        }));
        assert!(!result.contains(&MoveData {
            start_pos: Square::E8,
            end_pos: Square::G8,
            piece: Piece::King,
            move_type: MoveType::Castling
        }));
    }

    #[test]
    fn en_passant_move_available_for_white_both_directions() {
        let mut board =
            Board::from_fen("rnbqkbnr/2pp1pp1/pp5p/3PpP2/8/8/PPP1P1PP/RNBQKBNR w KQkq e6 0 5");
        let result = board.get_valid_moves();
        assert!(result.contains(&MoveData {
            start_pos: Square::D5,
            end_pos: Square::E6,
            piece: Piece::Pawn,
            move_type: MoveType::EnPassant
        }));
        assert!(result.contains(&MoveData {
            start_pos: Square::F5,
            end_pos: Square::E6,
            piece: Piece::Pawn,
            move_type: MoveType::EnPassant,
        }));
    }

    #[test]
    fn en_passant_move_available_for_black_both_directions() {
        let mut board =
            Board::from_fen("rnbqkbnr/ppp1p1pp/8/8/3pPp2/PP4PP/2PP1P2/RNBQKBNR b KQkq e3 0 5");
        let result = board.get_valid_moves();
        assert!(result.contains(&MoveData {
            start_pos: Square::D4,
            end_pos: Square::E3,
            piece: Piece::Pawn,
            move_type: MoveType::EnPassant
        }));
        assert!(result.contains(&MoveData {
            start_pos: Square::F4,
            end_pos: Square::E3,
            piece: Piece::Pawn,
            move_type: MoveType::EnPassant,
        }));
    }

    #[test]
    fn en_passant_for_white_captures_pawn() {
        // Target square E6 for white en passant. E5 has black pawn.
        let mut board =
            Board::from_fen("rnbqkbnr/2pp1pp1/pp5p/3PpP2/8/8/PPP1P1PP/RNBQKBNR w KQkq e6 0 5");

        board.make_move(&MoveData {
            start_pos: Square::D5,
            end_pos: Square::E6,
            piece: Piece::Pawn,
            move_type: MoveType::EnPassant,
        });
        assert_eq!(board.data[Square::E5], None);
    }

    #[test]
    fn en_passant_for_black_captures_pawn() {
        // Target square E3 for black en passant. E4 has white pawn.
        let mut board =
            Board::from_fen("rnbqkbnr/ppp1p1pp/8/8/3pPp2/PP4PP/2PP1P2/RNBQKBNR b KQkq e3 0 5");

        board.make_move(&MoveData {
            start_pos: Square::D4,
            end_pos: Square::E3,
            piece: Piece::Pawn,
            move_type: MoveType::EnPassant,
        });
        assert_eq!(board.data[Square::E4], None);
    }

    #[test]
    fn pawn_promotion_queen_white() {
        let mut board = Board::from_fen("8/P7/4k3/8/8/4K3/8/8 w - - 0 1");
        let result = board.get_valid_moves();

        assert!(result.contains(&MoveData {
            start_pos: Square::A7,
            end_pos: Square::A8,
            piece: Piece::Pawn,
            move_type: MoveType::QueenPromotion(Capture(None)),
        }));
    }

    #[test]
    fn pawn_promotion_rook_white() {
        let mut board = Board::from_fen("8/P7/4k3/8/8/4K3/8/8 w - - 0 1");
        let result = board.get_valid_moves();

        assert!(result.contains(&MoveData {
            start_pos: Square::A7,
            end_pos: Square::A8,
            piece: Piece::Pawn,
            move_type: MoveType::RookPromotion(Capture(None)),
        }));
    }

    #[test]
    fn pawn_promotion_bishop_white() {
        let mut board = Board::from_fen("8/P7/4k3/8/8/4K3/8/8 w - - 0 1");
        let result = board.get_valid_moves();

        assert!(result.contains(&MoveData {
            start_pos: Square::A7,
            end_pos: Square::A8,
            piece: Piece::Pawn,
            move_type: MoveType::BishopPromotion(Capture(None)),
        }));
    }

    #[test]
    fn pawn_promotion_knight_white() {
        let mut board = Board::from_fen("8/P7/4k3/8/8/4K3/8/8 w - - 0 1");
        let result = board.get_valid_moves();

        assert!(result.contains(&MoveData {
            start_pos: Square::A7,
            end_pos: Square::A8,
            piece: Piece::Pawn,
            move_type: MoveType::KnightPromotion(Capture(None)),
        }));
    }

    #[test]
    fn pawn_promotion_w_left_capture_white() {
        let mut board = Board::from_fen("1n1r4/2P2k2/8/8/8/5K2/8/8 w - - 0 1");
        let result = board.get_valid_moves();

        assert!(result.contains(&MoveData {
            start_pos: Square::C7,
            end_pos: Square::B8,
            piece: Piece::Pawn,
            move_type: MoveType::QueenPromotion(Capture(Some(Piece::Knight))),
        }));
    }

    #[test]
    fn pawn_promotion_w_right_capture_white() {
        let mut board = Board::from_fen("1n1r4/2P2k2/8/8/8/5K2/8/8 w - - 0 1");
        let result = board.get_valid_moves();

        assert!(result.contains(&MoveData {
            start_pos: Square::C7,
            end_pos: Square::D8,
            piece: Piece::Pawn,
            move_type: MoveType::QueenPromotion(Capture(Some(Piece::Rook))),
        }));
    }

    #[test]
    fn pawn_promotion_queen_black() {
        let mut board = Board::from_fen("8/5k2/8/8/4K3/8/2p5/8 b - - 0 1");
        let result = board.get_valid_moves();

        assert!(result.contains(&MoveData {
            start_pos: Square::C2,
            end_pos: Square::C1,
            piece: Piece::Pawn,
            move_type: MoveType::QueenPromotion(Capture(None)),
        }));
    }

    #[test]
    fn pawn_promotion_rook_black() {
        let mut board = Board::from_fen("8/P7/4k3/8/8/4K3/8/8 w - - 0 1");
        let result = board.get_valid_moves();

        assert!(result.contains(&MoveData {
            start_pos: Square::A7,
            end_pos: Square::A8,
            piece: Piece::Pawn,
            move_type: MoveType::RookPromotion(Capture(None)),
        }));
    }

    #[test]
    fn pawn_promotion_bishop_black() {
        let mut board = Board::from_fen("8/P7/4k3/8/8/4K3/8/8 w - - 0 1");
        let result = board.get_valid_moves();

        assert!(result.contains(&MoveData {
            start_pos: Square::A7,
            end_pos: Square::A8,
            piece: Piece::Pawn,
            move_type: MoveType::BishopPromotion(Capture(None)),
        }));
    }

    #[test]
    fn pawn_promotion_knight_black() {
        let mut board = Board::from_fen("8/P7/4k3/8/8/4K3/8/8 w - - 0 1");
        let result = board.get_valid_moves();

        assert!(result.contains(&MoveData {
            start_pos: Square::A7,
            end_pos: Square::A8,
            piece: Piece::Pawn,
            move_type: MoveType::KnightPromotion(Capture(None)),
        }));
    }

    #[test]
    fn pawn_promotion_w_left_capture_black() {
        let mut board = Board::from_fen("8/4k3/8/8/8/4K3/6p1/5B1Q b - - 0 1");
        let result = board.get_valid_moves();

        assert!(result.contains(&MoveData {
            start_pos: Square::G2,
            end_pos: Square::F1,
            piece: Piece::Pawn,
            move_type: MoveType::QueenPromotion(Capture(Some(Piece::Bishop))),
        }));
    }

    #[test]
    fn pawn_promotion_w_right_capture_black() {
        let mut board = Board::from_fen("8/4k3/8/8/8/4K3/6p1/5B1Q b - - 0 1");
        let result = board.get_valid_moves();

        assert!(result.contains(&MoveData {
            start_pos: Square::G2,
            end_pos: Square::H1,
            piece: Piece::Pawn,
            move_type: MoveType::QueenPromotion(Capture(Some(Piece::Queen))),
        }));
    }

    #[test]
    fn half_move_clock_reset_on_pawn_move() {
        let mut board = Board::from_fen(
            "r1b1k2r/pppp1ppp/2nb1q1n/4p3/4P3/2NB1Q1N/PPPP1PPP/R1B1K2R w KQkq - 8 6",
        );
        let result = board.get_valid_moves();
        let pawn_move = MoveData {
            start_pos: Square::A2,
            end_pos: Square::A3,
            piece: Piece::Pawn,
            move_type: MoveType::Regular(Capture(None)),
        };
        assert!(result.contains(&pawn_move));
        board.make_move(&pawn_move);
        assert_eq!(board.half_moves, 0);
    }

    #[test]
    fn half_move_clock_reset_on_capture() {
        let mut board = Board::from_fen(
            "r1b1k2r/pppp1ppp/2nb1q1n/4p3/4P3/2NB1Q1N/PPPP1PPP/R1B1K2R w KQkq - 8 6",
        );
        let result = board.get_valid_moves();
        let queen_move = MoveData {
            start_pos: Square::F3,
            end_pos: Square::F6,
            piece: Piece::Queen,
            move_type: MoveType::Regular(Capture(Some(Piece::Queen))),
        };
        assert!(result.contains(&queen_move));
        board.make_move(&queen_move);
        assert_eq!(board.half_moves, 0);
    }

    #[test]
    fn half_move_clock_inc_on_non_capture_move() {
        let mut board = Board::from_fen(
            "r1b1k2r/pppp1ppp/2nb1q1n/4p3/4P3/2NB1Q1N/PPPP1PPP/R1B1K2R w KQkq - 8 6",
        );
        let result = board.get_valid_moves();
        let bishop_move = MoveData {
            start_pos: Square::D3,
            end_pos: Square::B5,
            piece: Piece::Bishop,
            move_type: MoveType::Regular(Capture(None)),
        };
        assert!(result.contains(&bishop_move));
        board.make_move(&bishop_move);
        assert_eq!(board.half_moves, 9);
    }
}
