// Speed without transposition table for n=5: 3.16s
// Speed with table for n=5:

use ferris_chess_engine::Engine;

#[test]
fn tt_perft_pos_1_n_4() {
    let mut engine = Engine::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let result = engine.tt_perft(4);
    assert_eq!(result, 197281);
}

#[test]
fn tt_perft_pos_1_n_5() {
    let mut engine = Engine::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let result = engine.tt_perft(5);
    assert_eq!(result, 4865609);
}

#[ignore]
#[test]
fn tt_perft_pos_1_n_6() {
    let mut engine = Engine::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let result = engine.tt_perft(6);
    assert_eq!(result, 119060324);
}

// TODO Fix parsing of FEN string with ending - (no halfmove counts).
// Will then work with original Kiwipete
#[test]
fn tt_perft_pos_2_n_1() {
    let mut engine =
        Engine::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 1 1");
    let result = engine.tt_perft(1);
    assert_eq!(result, 48);
}

#[test]
fn tt_perft_pos_2_n_2() {
    let mut engine =
        Engine::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 1 1");
    let result = engine.tt_perft(2);
    assert_eq!(result, 2039);
}

#[test]
fn tt_perft_pos_2_n_3() {
    let mut engine =
        Engine::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 1 1");
    let result = engine.tt_perft(3);
    assert_eq!(result, 97862);
}

#[test]
fn tt_perft_pos_2_n_4() {
    let mut engine =
        Engine::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 1 1");
    let result = engine.tt_perft(4);
    assert_eq!(result, 4085603);
}

#[test]
fn tt_perft_pos_3_n_1() {
    let mut engine = Engine::new("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    let result = engine.tt_perft(1);
    assert_eq!(result, 14);
}

#[test]
fn tt_perft_pos_3_n_2() {
    let mut engine = Engine::new("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    let result = engine.tt_perft(2);
    assert_eq!(result, 191);
}

#[test]
fn tt_perft_pos_3_n_3() {
    let mut engine = Engine::new("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    let result = engine.tt_perft(3);
    assert_eq!(result, 2812);
}

#[test]
fn tt_perft_pos_3_n_4() {
    let mut engine = Engine::new("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    let result = engine.tt_perft(4);
    assert_eq!(result, 43238);
}

#[test]
fn tt_perft_pos_4_n_1() {
    let mut engine =
        Engine::new("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1");
    let result = engine.tt_perft(1);
    assert_eq!(result, 6);
}

#[test]
fn tt_perft_pos_4_n_2() {
    let mut engine =
        Engine::new("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1");
    let result = engine.tt_perft(2);
    assert_eq!(result, 264);
}

#[test]
fn tt_perft_pos_4_n_3() {
    let mut engine =
        Engine::new("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1");
    let result = engine.tt_perft(3);
    assert_eq!(result, 9467);
}

#[test]
fn tt_perft_pos_4_n_4() {
    let mut engine =
        Engine::new("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1");
    let result = engine.tt_perft(4);
    assert_eq!(result, 422333);
}

#[test]
fn tt_perft_pos_5_n_1() {
    let mut engine = Engine::new("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");
    let result = engine.tt_perft(1);
    assert_eq!(result, 44);
}

#[test]
fn tt_perft_pos_5_n_2() {
    let mut engine = Engine::new("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");
    let result = engine.tt_perft(2);
    assert_eq!(result, 1486);
}

#[test]
fn tt_perft_pos_5_n_3() {
    let mut engine = Engine::new("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");
    let result = engine.tt_perft(3);
    assert_eq!(result, 62379);
}

#[test]
fn tt_perft_pos_5_n_4() {
    let mut engine = Engine::new("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");
    let result = engine.tt_perft(4);
    assert_eq!(result, 2103487);
}

#[test]
fn tt_perft_pos_6_n_1() {
    let mut engine =
        Engine::new("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10");
    let result = engine.tt_perft(1);
    assert_eq!(result, 46);
}

#[test]
fn tt_perft_pos_6_n_2() {
    let mut engine =
        Engine::new("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10");
    let result = engine.tt_perft(2);
    assert_eq!(result, 2079);
}

#[test]
fn tt_perft_pos_6_n_3() {
    let mut engine =
        Engine::new("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10");
    let result = engine.tt_perft(3);
    assert_eq!(result, 89890);
}

#[test]
fn tt_perft_pos_6_n_4() {
    let mut engine =
        Engine::new("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10");
    let result = engine.tt_perft(4);
    assert_eq!(result, 3894594);
}
