# A UCI chess engine in rust
This project is under early development and consist of three rust crates related to chess - a binary crate that can be used to test out the project (ferris-chess-cli), a move generation library (ferris-chess-board), and a chess engine crate (ferris-chess-engine).

## To run locally
To start the project make sure a recent version of rust is installed and enter the following in the project root directory to see available command line options:
```
cargo run -- -h
```
Modes available:
```
perft  Runs perft performance test to a given depth
uci    Start the engine in UCI mode (default)
debug  Used during development for debugging
help   Print this message or the help of the given subcommand(s)
```

The default start mode is UCI which is to be used with a chess GUI such as Cutechess.

### Notes about purpose and implementation
This is a project mainly to learn rust. The crates are organized in a rust workspace. The board is stored in a 64 element array. Optimizations (such as bitboards and hashing) will be added over time.

The ferris-chess-engine crate is currently using a negamax algorithm with basic piece value evaluation.

#### Current status
Basic UCI commands work and have been tested in Cutechess. It should work in other chess programs supporting the UCI protocol.
