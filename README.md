# Ferrum Movegen

**Ferrum Movegen** is a blazing fast, legal-only chess move generator written in Rust. Optimized for speed and correctness, it achieves over **465 million nodes per second (Mnps)** in single-threaded perft benchmarks on modern hardware.

## Features

- **Magic Bitboards**: Ultra-fast sliding piece attack lookups (supports BMI2 `PEXT` for extra speed).
- **Copy-Make Architecture**: Uses an efficient "Copy-Make" approach (returning new board states by value) which proved faster than traditional Make/Unmake for this implementation.
- **Bulk Counting**: Specialized optimization for leaf-node move counting, boosting perft speeds by ~2x.
- **Mailbox/Bitboard Hybrid**: Combines bitboards for global queries with a mailbox for fast piece-at lookups.
- **Robust Verification**: Fully verified against Kiwipete and standard perft positions.

## Performance


| Position | Depth | Nodes | NPS |
|----------|-------|-------|-----|
| Start Position | 7 | 3,195,901,860 | **343 Mnps** |
| Kiwipete | 6 | 8,031,647,685 | **434 Mnps** |
| Position 3 | 8 | 3,009,794,393 | **177 Mnps** |
| Position 4 | 6 | 706,045,033 | **400 Mnps** |
| Position 5 | 5 | 89,941,194 | **386 Mnps** |

*Benchmarks run with `target-cpu=native`.*

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

Apache 2.0
