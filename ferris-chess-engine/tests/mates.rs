use ferris_chess_engine::{Engine, GoCommand};

// Current performance
// n = 1-6 time is 7.74s with cargo test --release

#[test]
fn mate_in_1_for_black_using_rooks_() {
    let mut engine = Engine::new("8/4k3/1r6/8/8/8/r7/4K3 b - - 0 1");
    let go_input = "go wtime 10000 btime 10000 movestogo 10".to_string();

    let go_cmd = GoCommand::new(&go_input);
    let result = engine.iter_deepening(&go_cmd).to_uci_move(&engine.board);
    assert_eq!(result, "b6b1".to_string());
}

#[test]
fn mate_in_1_for_white_using_bishop_queen() {
    let mut engine = Engine::new("2k5/6Q1/8/8/8/6B1/8/3K4 w - - 0 1");
    let go_input = "go wtime 10000 btime 10000 movestogo 10".to_string();

    let go_cmd = GoCommand::new(&go_input);
    let result = engine.iter_deepening(&go_cmd).to_uci_move(&engine.board);
    assert_eq!(result, "g7c7".to_string());
}

#[test]
fn mate_in_1_for_black_using_rooks_with_capture() {
    let mut engine = Engine::new("4k3/8/2r5/8/8/8/6r1/K1B5 b - - 0 1");
    let go_input = "go wtime 10000 btime 10000 movestogo 10".to_string();

    let go_cmd = GoCommand::new(&go_input);
    let result = engine.iter_deepening(&go_cmd).to_uci_move(&engine.board);
    assert_eq!(result, "c6c1".to_string());
}

#[test]
fn mate_in_1_for_white_using_queen_with_capture() {
    let mut engine = Engine::new("1kr5/ppp5/8/1N6/8/8/8/4K1Q1 w - - 0 1");
    let go_input = "go wtime 10000 btime 10000 movestogo 10".to_string();

    let go_cmd = GoCommand::new(&go_input);
    let result = engine.iter_deepening(&go_cmd).to_uci_move(&engine.board);
    assert_eq!(result, "g1a7".to_string());
}

#[test]
fn mate_in_2_for_white_using_queen() {
    let mut engine = Engine::new("4Nnk1/5ppb/3N3p/8/8/8/5Q2/2K5 w - - 0 1");
    let go_input = "go wtime 10000 btime 10000 movestogo 10".to_string();
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
    let go_input = "go wtime 10000 btime 10000 movestogo 10".to_string();

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
