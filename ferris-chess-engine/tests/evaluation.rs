use ferris_chess_board::Board;
use ferris_chess_engine::Engine;

#[test]
fn evalation_for_startpos_within_50_centipawns() {
    let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let engine = Engine::new();
    let w_eval = engine.static_eval(&board);
    assert!(w_eval >= -50 && w_eval <= 50);
}
