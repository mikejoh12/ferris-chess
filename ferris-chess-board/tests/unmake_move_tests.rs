#[cfg(test)]
mod unmake_move_tests {

    use ferris_chess_board::*;

    #[test]
    fn unmake_white_double_pawn_push() {
        let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let starting_board = board.clone();

        let result = board.get_pseudo_legal_moves();
        let pawn_move = MoveData {
            start_pos: Square::E2,
            end_pos: Square::E4,
            piece: Piece::Pawn,
            move_type: MoveType::Regular(Capture(None)),
        };
        assert!(result.contains(&pawn_move));
        board.make_move(&pawn_move);
        board.unmake_move(&pawn_move);
        assert_eq!(starting_board, board);
    }

    #[test]
    fn unmake_white_capture_with_bishop() {
        let mut board =
            Board::from_fen("rnbqkbnr/p1pppppp/8/1p6/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1");
        let starting_board = board.clone();

        let result = board.get_pseudo_legal_moves();
        let bishop_move = MoveData {
            start_pos: Square::F1,
            end_pos: Square::B5,
            piece: Piece::Bishop,
            move_type: MoveType::Regular(Capture(Some(Piece::Pawn))),
        };
        assert!(result.contains(&bishop_move));

        board.make_move(&bishop_move);
        board.unmake_move(&bishop_move);

        assert_eq!(starting_board, board);
    }

    #[test]
    fn unmake_black_capture_with_knight() {
        let mut board =
            Board::from_fen("r1bqkbnr/pppp1ppp/2n5/4p3/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq d3 0 3");
        let starting_board = board.clone();

        let result = board.get_pseudo_legal_moves();
        let knight_move = MoveData {
            start_pos: Square::C6,
            end_pos: Square::D4,
            piece: Piece::Knight,
            move_type: MoveType::Regular(Capture(Some(Piece::Pawn))),
        };
        assert!(result.contains(&knight_move));

        board.make_move(&knight_move);
        board.unmake_move(&knight_move);

        assert_eq!(starting_board, board);
    }

    #[test]
    fn unmake_white_castling_queen_side() {
        let mut board =
            Board::from_fen("r3k2r/ppp1nppp/2nbbq2/3pp3/3PP3/2NBBQ2/PPP1NPPP/R3K2R w KQkq - 10 8");

        let start_position = board.clone();

        let w_moves = board.get_pseudo_legal_moves();

        let w_queen_castling = &MoveData {
            start_pos: Square::E1,
            end_pos: Square::C1,
            piece: Piece::King,
            move_type: MoveType::Castling,
        };

        assert!(w_moves.contains(w_queen_castling));

        board.make_move(w_queen_castling);
        board.unmake_move(w_queen_castling);

        assert_eq!(board, start_position);
    }

    #[test]
    fn unmake_black_castling_queen_side() {
        let mut board =
            Board::from_fen("r3k2r/ppp1nppp/2nbbq2/3pp3/3PP3/2NBBQ2/PPP1NPPP/R3K2R b KQkq - 10 8");

        let start_position = board.clone();

        let b_moves = board.get_pseudo_legal_moves();

        let b_queen_castling = &MoveData {
            start_pos: Square::E8,
            end_pos: Square::C8,
            piece: Piece::King,
            move_type: MoveType::Castling,
        };

        assert!(b_moves.contains(b_queen_castling));

        board.make_move(b_queen_castling);
        board.unmake_move(b_queen_castling);

        assert_eq!(board, start_position);
    }

    #[test]
    fn unmake_white_castling_king_side() {
        let mut board =
            Board::from_fen("r3k2r/ppp1nppp/2nbbq2/3pp3/3PP3/2NBBQ2/PPP1NPPP/R3K2R w KQkq - 10 8");

        let start_position = board.clone();

        let w_moves = board.get_pseudo_legal_moves();

        let w_king_castling = &MoveData {
            start_pos: Square::E1,
            end_pos: Square::G1,
            piece: Piece::King,
            move_type: MoveType::Castling,
        };

        assert!(w_moves.contains(w_king_castling));

        board.make_move(w_king_castling);
        board.unmake_move(w_king_castling);

        assert_eq!(board, start_position);
    }

    #[test]
    fn unmake_black_castling_king_side() {
        let mut board =
            Board::from_fen("r3k2r/ppp1nppp/2nbbq2/3pp3/3PP3/2NBBQ2/PPP1NPPP/R3K2R b KQkq - 10 8");

        let start_position = board.clone();

        let b_moves = board.get_pseudo_legal_moves();

        let b_king_castling = &MoveData {
            start_pos: Square::E8,
            end_pos: Square::G8,
            piece: Piece::King,
            move_type: MoveType::Castling,
        };

        assert!(b_moves.contains(b_king_castling));

        board.make_move(b_king_castling);
        board.unmake_move(b_king_castling);

        assert_eq!(board, start_position);
    }

    #[test]
    fn unmake_white_en_passant() {
        let mut board =
            Board::from_fen("rnbqkbnr/2pp1pp1/pp5p/3PpP2/8/8/PPP1P1PP/RNBQKBNR w KQkq e6 0 5");
        let start_board = board.clone();

        let w_moves = board.get_pseudo_legal_moves();

        let ep_move = &MoveData {
            start_pos: Square::D5,
            end_pos: Square::E6,
            piece: Piece::Pawn,
            move_type: MoveType::EnPassant,
        };

        assert!(w_moves.contains(ep_move));

        board.make_move(ep_move);
        board.unmake_move(ep_move);

        assert_eq!(board, start_board);
    }

    #[test]
    fn unmake_black_en_passant() {
        let mut board =
            Board::from_fen("rnbqkbnr/ppp1p1pp/8/8/3pPp2/PP4PP/2PP1P2/RNBQKBNR b KQkq e3 0 5");
        let start_board = board.clone();

        let b_moves = board.get_pseudo_legal_moves();

        let ep_move = &MoveData {
            start_pos: Square::D4,
            end_pos: Square::E3,
            piece: Piece::Pawn,
            move_type: MoveType::EnPassant,
        };

        assert!(b_moves.contains(ep_move));

        board.make_move(ep_move);
        board.unmake_move(ep_move);

        assert_eq!(board, start_board);
    }

    #[test]
    fn unmake_white_pawn_promotion_queen() {
        let mut board = Board::from_fen("8/P7/4k3/8/8/4K3/8/8 w - - 0 1");
        let start_pos = board.clone();

        let moves = board.get_pseudo_legal_moves();

        let w_promotion_move = &MoveData {
            start_pos: Square::A7,
            end_pos: Square::A8,
            piece: Piece::Pawn,
            move_type: MoveType::QueenPromotion(Capture(None)),
        };
        assert!(moves.contains(w_promotion_move));

        board.make_move(w_promotion_move);
        board.unmake_move(w_promotion_move);
        assert_eq!(board, start_pos);
    }

    #[test]
    fn unmake_black_pawn_promotion_queen() {
        let mut board = Board::from_fen("8/5k2/8/8/4K3/8/2p5/8 b - - 0 1");
        let start_pos = board.clone();

        let moves = board.get_pseudo_legal_moves();

        let b_promotion_move = &MoveData {
            start_pos: Square::C2,
            end_pos: Square::C1,
            piece: Piece::Pawn,
            move_type: MoveType::QueenPromotion(Capture(None)),
        };
        assert!(moves.contains(b_promotion_move));

        board.make_move(b_promotion_move);
        board.unmake_move(b_promotion_move);
        assert_eq!(board, start_pos);
    }

    #[test]
    fn unmake_white_pawn_promotion_with_capture() {
        let mut board = Board::from_fen("1n1r4/2P2k2/8/8/8/5K2/8/8 w - - 0 1");
        let start_pos = board.clone();

        let moves = board.get_pseudo_legal_moves();

        let w_promotion_capture_move = &MoveData {
            start_pos: Square::C7,
            end_pos: Square::B8,
            piece: Piece::Pawn,
            move_type: MoveType::QueenPromotion(Capture(Some(Piece::Knight))),
        };
        assert!(moves.contains(w_promotion_capture_move));

        board.make_move(w_promotion_capture_move);
        board.unmake_move(w_promotion_capture_move);
        assert_eq!(board, start_pos);
    }

    #[test]
    fn unmake_black_pawn_promotion_with_capture() {
        let mut board = Board::from_fen("8/4k3/8/8/8/4K3/6p1/5B1Q b - - 0 1");
        let start_pos = board.clone();

        let moves = board.get_pseudo_legal_moves();

        let b_promotion_capture_move = &MoveData {
            start_pos: Square::G2,
            end_pos: Square::H1,
            piece: Piece::Pawn,
            move_type: MoveType::QueenPromotion(Capture(Some(Piece::Queen))),
        };

        assert!(moves.contains(b_promotion_capture_move));

        board.make_move(b_promotion_capture_move);
        board.unmake_move(b_promotion_capture_move);
        assert_eq!(board, start_pos);
    }
}
