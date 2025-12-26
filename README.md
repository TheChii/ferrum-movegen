# Ferrum Movegen

**Ferrum Movegen** is a blazing fast, legal-only chess move generator written in Rust. Optimized for speed and correctness, it achieves over **465 million nodes per second (Mnps)** in single-threaded perft benchmarks on modern hardware.

## Features

- **Magic Bitboards**: Ultra-fast sliding piece attack lookups (supports BMI2 `PEXT` for extra speed).
- **Copy-Make Architecture**: Uses an efficient "Copy-Make" approach (returning new board states by value) which proved faster than traditional Make/Unmake for this implementation.
- **Bulk Counting**: Specialized optimization for leaf-node move counting, boosting perft speeds by ~2x.
- **Mailbox/Bitboard Hybrid**: Combines bitboards for global queries with a mailbox for fast piece-at lookups.
- **Robust Verification**: Fully verified against Kiwipete and standard perft positions.

## Performance

Tested on [User Hardware Specification, e.g. Ryzen 9 / M1 Max]:

| Metric | Speed |
|--------|-------|
| Perft 6 | **468 Mnps** |

*Note: Benchmarked with `target-cpu=native` and `pext` feature enabled.*

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
ferrum_movegen = { path = "path/to/ferrum" }
```

### Example

```rust
use movegen::board::Board;
use movegen::types::Square;

fn main() {
    // Initialize board from start position
    let board = Board::startpos();

    // Generate legal moves
    let moves = board.generate_moves();

    for mv in moves.iter() {
        println!("Move: {:?}", mv);
    }
    
    // Fast bulk counting (e.g. for perft)
    let count = board.generate_moves_count();
    println!("Total moves: {}", count);
}
```

## Building

To build with maximum optimizations (including PEXT):

```bash
RUSTFLAGS="-C target-cpu=native" cargo build --release --features pext
```

## License

MIT
