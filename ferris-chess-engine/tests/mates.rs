use ferris_chess_board::{MoveData, MoveType, Piece, Square};
use ferris_chess_engine::{Engine, GoCommand};

// Current performance
// n = 1-6 time is 7.74s with cargo test --release

#[test]
fn mate_in_1_for_black_using_rooks_() {
    let mut engine = Engine::new("8/4k3/1r6/8/8/8/r7/4K3 b - - 0 1");
    let go_input = "go wtime 2000 btime 2000 movestogo 1".to_string();

    let go_cmd = GoCommand::new(&go_input);
    let result = engine.iter_deepening(&go_cmd).to_uci_move(&engine.board);
    assert_eq!(result, "b6b1".to_string());
}

#[test]
fn mate_in_1_for_white_using_bishop_queen() {
    let mut engine = Engine::new("2k5/6Q1/8/8/8/6B1/8/3K4 w - - 0 1");
    let go_input = "go wtime 2000 btime 2000 movestogo 1".to_string();

    let go_cmd = GoCommand::new(&go_input);
    let result = engine.iter_deepening(&go_cmd).to_uci_move(&engine.board);
    assert_eq!(result, "g7c7".to_string());
}

#[test]
fn mate_in_1_for_black_using_rooks_with_capture() {
    let mut engine = Engine::new("4k3/8/2r5/8/8/8/6r1/K1B5 b - - 0 1");
    let go_input = "go wtime 2000 btime 2000 movestogo 1".to_string();

    let go_cmd = GoCommand::new(&go_input);
    let result = engine.iter_deepening(&go_cmd).to_uci_move(&engine.board);
    assert_eq!(result, "c6c1".to_string());
}

#[test]
fn mate_in_1_for_white_using_queen_with_capture() {
    let mut engine = Engine::new("1kr5/ppp5/8/1N6/8/8/8/4K1Q1 w - - 0 1");
    let go_input = "go wtime 2000 btime 2000 movestogo 1".to_string();

    let go_cmd = GoCommand::new(&go_input);
    let result = engine.iter_deepening(&go_cmd).to_uci_move(&engine.board);
    assert_eq!(result, "g1a7".to_string());
}

#[test]
fn mate_in_2_for_white_using_queen() {
    let mut engine = Engine::new("4Nnk1/5ppb/3N3p/8/8/8/5Q2/2K5 w - - 0 1");
    let go_input = "go wtime 2000 btime 2000 movestogo 1".to_string();
    let go_cmd = GoCommand::new(&go_input);

    let w_move_1 = engine.iter_deepening(&go_cmd);
    assert_eq!(w_move_1.to_uci_move(&engine.board), "f2f7".to_string());
    engine.board.make_move(&w_move_1);

    let b_move_1 = engine.iter_deepening(&go_cmd);
    assert_eq!(b_move_1.to_uci_move(&engine.board), "g8h8".to_string());
    engine.board.make_move(&b_move_1);

    let w_move_2 = engine.iter_deepening(&go_cmd);
    assert_eq!(w_move_2.to_uci_move(&engine.board), "f7g7".to_string());
}

#[test]
fn mate_in_2_for_black_knight_rook() {
    let mut engine = Engine::new("3k4/8/3r4/b7/5n2/8/PPP5/2K5 b - - 0 1");
    let go_input = "go wtime 2000 btime 2000 movestogo 1".to_string();

    let go_cmd = GoCommand::new(&go_input);

    let b_move_1 = engine.iter_deepening(&go_cmd);
    assert_eq!(b_move_1.to_uci_move(&engine.board), "f4e2".to_string());
    engine.board.make_move(&b_move_1);

    let w_move_1 = engine.iter_deepening(&go_cmd);
    assert_eq!(w_move_1.to_uci_move(&engine.board), "c1b1".to_string());
    engine.board.make_move(&w_move_1);

    let b_move_2 = engine.iter_deepening(&go_cmd);
    assert_eq!(b_move_2.to_uci_move(&engine.board), "d6d1".to_string());
}

#[test]
fn mate_in_3_for_black_with_rooks() {
    let mut engine = Engine::new("4k3/1r1pn3/2r2p2/8/4p3/1P2P3/2P5/6K1 b - - 0 1");
    let go_input = "go wtime 2000 btime 2000 movestogo 1".to_string();

    let go_cmd = GoCommand::new(&go_input);

    let b_move_1 = engine.iter_deepening(&go_cmd);
    assert_eq!(b_move_1.to_uci_move(&engine.board), "c6c2".to_string());
    engine.board.make_move(&b_move_1);

    // White has some options but it is mate in 3 for black in either case
    // Make a white king move to predict black path
    let w_move_1: MoveData = MoveData {
        start_pos: Square::G1,
        end_pos: Square::F1,
        piece: Piece::King,
        move_type: MoveType::Regular,
        capture: None,
    };
    engine.board.make_move(&w_move_1);

    let b_move_2 = engine.iter_deepening(&go_cmd);
    assert_eq!(b_move_2.to_uci_move(&engine.board), "b7b3".to_string());
    engine.board.make_move(&b_move_2);

    let w_move_2 = engine.iter_deepening(&go_cmd);
    engine.board.make_move(&w_move_2);

    let b_move_3 = engine.iter_deepening(&go_cmd);
    assert_eq!(b_move_3.to_uci_move(&engine.board), "b3b1".to_string());
}

#[test]
fn mate_in_3_for_white_with_knight_bishop_queen() {
    let mut engine = Engine::new("4r3/pk3pb1/1pNp2p1/3P1q2/2Qp1B1P/8/PPP2PP1/2K5 w - - 6 24");

    // Give 10 seconds time for now due to somewhat complex position
    let go_input = "go wtime 10000 btime 10000 movestogo 1".to_string();

    let go_cmd = GoCommand::new(&go_input);

    let w_move_1 = engine.iter_deepening(&go_cmd);
    assert_eq!(w_move_1.to_uci_move(&engine.board), "c6a5".to_string());
    engine.board.make_move(&w_move_1);

    // Black has some options but it is mate in 3 for white in either case
    // Make a black king move
    let b_move_1: MoveData = MoveData {
        start_pos: Square::B7,
        end_pos: Square::B8,
        piece: Piece::King,
        move_type: MoveType::Regular,
        capture: None,
    };
    engine.board.make_move(&b_move_1);

    let w_move_2 = engine.iter_deepening(&go_cmd);
    assert_eq!(w_move_2.to_uci_move(&engine.board), "f4d6".to_string());
    engine.board.make_move(&w_move_2);

    let b_move_2: MoveData = MoveData {
        start_pos: Square::B8,
        end_pos: Square::A8,
        piece: Piece::King,
        move_type: MoveType::Regular,
        capture: None,
    };
    engine.board.make_move(&b_move_2);

    let w_move_3 = engine.iter_deepening(&go_cmd);
    assert_eq!(w_move_3.to_uci_move(&engine.board), "c4c6".to_string());
}
