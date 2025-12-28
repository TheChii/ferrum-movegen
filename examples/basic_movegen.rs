//! Basic move generation example.
//!
//! Run with: cargo run --example basic_movegen

use movegen::{Board, Square, Piece, Color};

fn main() {
    println!("=== Ferrum Movegen: Basic Move Generation ===\n");

    // Create the starting position
    let board = Board::default();
    
    println!("Starting position:");
    println!("{:?}\n", board);
    println!("FEN: {}", board.to_fen());
    println!("Side to move: {:?}", board.turn());
    
    // Generate all legal moves
    let moves = board.generate_moves();
    println!("\nLegal moves: {}", moves.len());
    
    // Group moves by piece type
    println!("\nMoves by type:");
    
    let mut pawn_moves = Vec::new();
    let mut knight_moves = Vec::new();
    
    for mv in moves.iter() {
        let from = mv.from();
        if let Some((piece, _color)) = board.piece_at(from) {
            match piece {
                Piece::Pawn => pawn_moves.push(mv.to_uci()),
                Piece::Knight => knight_moves.push(mv.to_uci()),
                _ => {}
            }
        }
    }
    
    println!("  Pawn moves ({}): {}", pawn_moves.len(), pawn_moves.join(", "));
    println!("  Knight moves ({}): {}", knight_moves.len(), knight_moves.join(", "));
    
    // Test from a different position - Kiwipete
    println!("\n--- Kiwipete Position ---");
    let kiwipete = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    let board = Board::from_fen(kiwipete).expect("Valid FEN");
    
    let moves = board.generate_moves();
    println!("Kiwipete legal moves: {}", moves.len());
    println!("In check: {}", board.in_check());
    
    // Show capture moves
    let captures: Vec<_> = moves.iter()
        .filter(|m| m.is_capture())
        .map(|m| m.to_uci())
        .collect();
    println!("Captures ({}): {}", captures.len(), captures.join(", "));
    
    // Bulk counting (faster for perft)
    println!("\n--- Bulk Counting ---");
    let count = board.generate_moves_count();
    println!("Move count (bulk): {}", count);
}
