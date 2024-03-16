use ferris_chess_engine::Engine;

#[test]
fn evalation_for_startpos_within_50_centipawns() {
    let engine = Engine::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let w_eval = engine.static_eval();
    assert!(w_eval >= -50 && w_eval <= 50);
}
