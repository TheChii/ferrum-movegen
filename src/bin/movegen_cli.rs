//! CLI for move generation - outputs legal moves for a FEN position.

use std::env;
use movegen::Board;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: movegen_cli <fen>");
        eprintln!("       movegen_cli perft <fen> <depth>");
        std::process::exit(1);
    }
    
    if args[1] == "perft" {
        if args.len() < 4 {
            eprintln!("Usage: movegen_cli perft <fen> <depth>");
            std::process::exit(1);
        }
        let fen = &args[2];
        let depth: u32 = args[3].parse().unwrap_or(1);
        
        match Board::from_fen(fen) {
            Ok(board) => {
                let start = std::time::Instant::now();
                let nodes = movegen::testing::perft(&board, depth);
                let elapsed = start.elapsed();
                let params = elapsed.as_secs_f64();
                let nps = if params > 0.0 { nodes as f64 / params } else { 0.0 };
                
                println!("Nodes: {}", nodes);
                println!("Time:  {:.3} s", params);
                println!("NPS:   {:.0}", nps);
            }
            Err(e) => {
                eprintln!("Invalid FEN: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        // Assume the rest is a FEN string
        let fen = args[1..].join(" ");
        
        match Board::from_fen(&fen) {
            Ok(board) => {
                let moves = board.generate_moves();
                let mut uci_moves: Vec<String> = moves.iter().map(|m| m.to_uci()).collect();
                uci_moves.sort();
                
                for mv in uci_moves {
                    println!("{}", mv);
                }
            }
            Err(e) => {
                eprintln!("Invalid FEN: {}", e);
                std::process::exit(1);
            }
        }
    }
}
