//! NPS benchmark for perft.

use std::time::Instant;
use movegen::Board;
use movegen::testing::perft;

fn main() {
    println!("=== PERFT NPS BENCHMARK ===\n");
    
    let board = Board::default();
    
    // Warm up
    let _ = perft(&board, 3);
    
    // Benchmark different depths
    for depth in 4..=6 {
        let start = Instant::now();
        let nodes = perft(&board, depth);
        let elapsed = start.elapsed();
        
        let secs = elapsed.as_secs_f64();
        let nps = (nodes as f64 / secs) as u64;
        
        println!(
            "Depth {}: {:>12} nodes in {:>8.3}s = {:>10} NPS ({:.2}M NPS)",
            depth, nodes, secs, nps, nps as f64 / 1_000_000.0
        );
    }
    
    println!("\nDone!");
}
