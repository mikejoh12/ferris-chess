use std::{io::BufRead, sync::mpsc, thread};
extern crate rand;
use ferris_chess_board::{Board, MoveData};
use ferris_chess_engine::{Engine, GoCommand};

pub struct Uci {
    engine: Engine,
}

impl Uci {
    pub fn new() -> Self {
        Uci {
            engine: Engine::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
        }
    }

    pub fn start_read_stdin_loop(&mut self, _board: &mut Board) {
        let (tx, rx) = mpsc::channel();

        // Start a new thread for reading from stdin
        let io_thread = thread::spawn(move || {
            for line in std::io::stdin().lock().lines() {
                match line {
                    Ok(l) => tx.send(l).unwrap(),
                    Err(_) => panic!("Error reading uci input"),
                }
            }
        });

        self.start_handle_stdin_loop(&rx);

        io_thread.join().unwrap();
    }

    fn start_handle_stdin_loop(&mut self, rx: &mpsc::Receiver<String>) {
        loop {
            let input = rx.try_recv();
            if input.is_err() {
                continue;
            }
            let cmd = input.unwrap();

            if !cmd.is_empty() {
                self.uci_action(cmd)
            }
        }
    }

    fn uci_action(&mut self, cmd: String) {
        let cmd_parts: Vec<String> = cmd.split_whitespace().map(|s| s.to_string()).collect();
        match cmd_parts[0].as_str() {
            "uci" => self.handle_uci(),
            "debug" => self.handle_debug(&cmd_parts),
            "isready" => self.handle_isready(&cmd_parts),
            "setoption" => self.handle_setoption(&cmd_parts),
            "register" => println!("Got register"),
            "ucinewgame" => self.handle_ucinewgame(&cmd_parts),
            "position" => self.handle_position(&cmd),
            "go" => self.handle_go(&cmd),
            "stop" => self.handle_stop(),
            "ponderhit" => self.handle_ponderhit(&cmd_parts),
            "quit" => self.handle_quit(),
            "d" => self.engine.board.print(),
            _ => (), // Ignore invalid inputs (UCI)
        }
    }

    fn handle_uci(&self) {
        println!("id name Ferris Chess 0.1");
        println!("id author Michael J");
        println!("uciok");
    }

    fn handle_debug(&self, cmd_parts: &Vec<String>) {
        match cmd_parts[1].as_str() {
            "on" => println!("Got debug on"),
            "off" => println!("Got debug off"),
            _ => (),
        }
    }

    fn handle_isready(&self, _cmd_parts: &Vec<String>) {
        println!("readyok"); // Required response to isready
    }

    fn handle_setoption(&self, _cmd_parts: &Vec<String>) {
        println!("Got setoption");
    }

    fn handle_ucinewgame(&mut self, _cmd_parts: &Vec<String>) {
        self.engine.new_game();
    }

    fn handle_position(&mut self, cmd: &String) {
        let mut parts = cmd.split("moves").map(|p| p.trim());
        let position_cmd = parts.next().unwrap();

        if position_cmd == "position startpos" {
            self.engine.board =
                Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        } else if position_cmd.starts_with("position fen") {
            let fen = cmd.strip_prefix("position fen ").unwrap();
            self.engine.board = Board::from_fen(fen);
        } else {
            panic!("Invalid position command: {}", position_cmd);
        }

        // Clear transposition table on each position
        self.engine.t_table.clear();

        if let Some(m) = parts.next() {
            for uci_move in m.split_ascii_whitespace() {
                let m = MoveData::from_uci(&uci_move.to_string(), &self.engine.board);
                self.engine.board.make_move(&m);
            }
        }
    }

    fn handle_go(&mut self, cmd: &String) {
        /* Add code to parse go options
        match cmd_parts[1].as_str() {
            "searchmoves" => println!("Got go searchmoves"),
            "ponder" => println!("Got go ponder"),
            "wtime" => println!("Got go wtime"),
            "btime" => println!("Got go btime"),
            "winc" => println!("Got go winc"),
            "binc" => println!("Got go binc"),
            "movestogo" => println!("Got go movestogo"),
            "depth" => println!("Got go depth"),
            "nodes" => println!("Got go nodes"),
            "mate" => println!("Got go mate"),
            "movetime" => println!("Got go movetime"),
            "infinite" => println!("Got go infinite"),
            _ => (),
        }
        */

        let go_cmd = GoCommand::new(cmd);
        let m = self.engine.iter_deepening(&go_cmd);

        let uci_move = m.to_uci_move(&self.engine.board);
        println!("bestmove {}", uci_move);
    }

    fn handle_stop(&mut self) {
        self.engine.stop();
    }

    fn handle_ponderhit(&self, _cmd_parts: &Vec<String>) {
        println!("Got ponderhit");
    }

    fn handle_quit(&self) {
        panic!("Shutting down engine");
    }
}
