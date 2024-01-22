# A chess library in rust
This project is under very early development and consist of three rust crates related to chess - a binary crate that can be used to test out the project (ferris-chess-game), a move generation library (ferris-chess-board), and a chess engine crate (ferris-chess-engine, still TODO).

## To run locally
To start the project in it's current very early stage, make sure a recent version of rust is installed and enter the following in the project root directory:
```
cargo run
```

### Notes about purpose and implementation
This is a project mainly to learn rust while combining it with an interest in chess. The crates are orginized in a rust workspace. The board is stored in a 64 element array. Optimizations will be added over time.

#### Current status
Working on the ferris-chess-board crate to implement board functionality and move generation.