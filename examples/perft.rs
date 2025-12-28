//! Perft (performance test) example.
//!
//! Perft is a debugging and benchmarking tool that counts all legal move positions
//! at a given depth. It's the gold standard for verifying move generator correctness.
//!
//! Run with: cargo run --example perft --release
//! For best performance: RUSTFLAGS="-C target-cpu=native" cargo run --example perft --release --features pext

use movegen::Board;
use movegen::testing::perft;

fn main() {
    println!("=== Ferrum Movegen: Perft Benchmarking ===\n");

    // Standard perft test positions
    let positions = [
        ("Starting Position", "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
        ("Kiwipete", "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"),
        ("Position 3", "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1"),
        ("Position 4", "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1"),
        ("Position 5", "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8"),
    ];

    // Expected node counts for verification (depth 5)
    let expected = [
        4865609,    // Starting position, depth 5
        193690690,  // Kiwipete, depth 5 (too slow, we'll use depth 4: 4085603)
        674624,     // Position 3, depth 5
        15833292,   // Position 4, depth 5
        89941194,   // Position 5, depth 5
    ];

    for (i, (name, fen)) in positions.iter().enumerate() {
        println!("--- {} ---", name);
        println!("FEN: {}", fen);
        
        let board = Board::from_fen(fen).expect("Valid FEN");
        
        // Use lower depth for faster testing
        let depth = if i == 1 { 4 } else { 5 }; // Kiwipete is slower
        
        let start = std::time::Instant::now();
        let nodes = perft(&board, depth);
        let elapsed = start.elapsed();
        
        let nps = if elapsed.as_secs_f64() > 0.0 {
            nodes as f64 / elapsed.as_secs_f64()
        } else {
            0.0
        };
        
        println!("Depth {}: {} nodes", depth, nodes);
        println!("Time: {:.3}s | NPS: {:.2}M", elapsed.as_secs_f64(), nps / 1_000_000.0);
        
        // Verify against expected (for depth 5 positions)
        if i != 1 {
            let expected_val = expected[i];
            if nodes == expected_val {
                println!("✓ Verified correct");
            } else {
                println!("✗ Expected {} but got {}", expected_val, nodes);
            }
        }
        
        println!();
    }

    // Detailed perft with divide (per-move breakdown)
    println!("--- Divide Example (Starting Position, Depth 2) ---");
    let board = Board::default();
    let moves = board.generate_moves();
    
    let mut total = 0u64;
    for mv in moves.iter() {
        let new_board = board.make_move_new(mv);
        let nodes = perft(&new_board, 1);
        println!("{}: {}", mv.to_uci(), nodes);
        total += nodes;
    }
    println!("Total: {}", total);
}
