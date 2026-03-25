# Chicory - Chess Engine 
![GitHub Release](https://img.shields.io/github/v/release/achester88/chicory-engine)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/achester88/chicory-engine/rust.yml)
![GitHub License](https://img.shields.io/github/license/achester88/chicory-engine)

###### a UCI-compatible, dependence-less chess engine built in Rust 

Chicory uses bitboard for move generation and alpha-beta pruning, minmax for evaluation


## Features
- [x] Bitboard movement generation
- [x] UCI protocol support
- [x] Minimax search
- [x] Alpha-beta pruning
- [x] Piece move table
- [ ] Magic bitboards
- [ ] Transposition tables
- [ ] Opening book support
- [ ] Endgame tablebases

#### UCI Options
- MaxSearchDepth \<depth\> - Limits the engine to only search to a preset depth before reporting a best move

#### UCI Extensions
  Chicory also implements additional debug commands
  ```bash
  perft <depth> # Return a count of total child nodes
  ```

## Getting Started

Pre-built release version of Chicory, for Linux, Mac, and Windows, can be found [here](https://github.com/achester88/chicory-engine/releases)

### Building From Scratch

Step 1: Clone repo

```bash
git clone https://github.com/achester88/chicory-engine.git && cd chicory-engine
```

Step 2: Build via Cargo

```bash
cargo build --release
```

Step 3: Find Binaries at 

```bash
./target/release/chicory
```

## License

The software is available under the GPL License.
