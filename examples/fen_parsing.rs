//! FEN string parsing and board setup example.
//!
//! FEN (Forsyth-Edwards Notation) is a standard way to represent chess positions.
//!
//! Run with: cargo run --example fen_parsing

use movegen::{Board, Color, Piece, Square, CastleRights};

fn main() {
    println!("=== Ferrum Movegen: FEN Parsing ===\n");

    // Starting position FEN
    let starting_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    println!("Parsing starting position:");
    println!("FEN: {}", starting_fen);
    
    let board = Board::from_fen(starting_fen).expect("Valid FEN");
    println!("Side to move: {:?}", board.turn());
    println!("Castling rights: {:?}", board.castling());
    println!("En passant: {:?}", board.ep_square());
    println!("Halfmove clock: {}", board.halfmove_clock());
    println!("Fullmove number: {}", board.fullmove_number());
    println!();

    // Position after 1. e4
    let after_e4 = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
    println!("Position after 1. e4:");
    println!("FEN: {}", after_e4);
    
    let board = Board::from_fen(after_e4).expect("Valid FEN");
    println!("Side to move: {:?}", board.turn());
    println!("En passant square: {:?}", board.ep_square());
    println!();

    // Middlegame position
    let middlegame = "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4";
    println!("Italian Game position:");
    println!("FEN: {}", middlegame);
    
    let board = Board::from_fen(middlegame).expect("Valid FEN");
    println!("Pieces on board:");
    for sq_idx in 0..64 {
        let sq = Square::from_index(sq_idx).unwrap();
        if let Some((piece, color)) = board.piece_at(sq) {
            println!("  {:?}: {:?} {:?}", sq, color, piece);
        }
    }
    println!();

    // Endgame position with limited castling
    let endgame = "4k3/8/8/8/8/8/4P3/4K2R w K - 0 1";
    println!("Simple endgame:");
    println!("FEN: {}", endgame);
    
    let board = Board::from_fen(endgame).expect("Valid FEN");
    println!("White king: {:?}", board.king_square(Color::White));
    println!("Black king: {:?}", board.king_square(Color::Black));
    println!("Castling: {:?}", board.castling());
    println!("Legal moves: {}", board.generate_moves().len());
    println!();

    // Converting back to FEN
    println!("--- Round-trip test ---");
    let original = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    let board = Board::from_fen(original).expect("Valid FEN");
    let roundtrip = board.to_fen();
    
    println!("Original:  {}", original);
    println!("Roundtrip: {}", roundtrip);
    println!("Match: {}", original == roundtrip);
    println!();

    // Invalid FEN handling
    println!("--- Error handling ---");
    let invalid_fens = [
        "invalid",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP w KQkq - 0 1", // Missing rank
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR x KQkq - 0 1", // Invalid side
    ];
    
    for fen in invalid_fens {
        match Board::from_fen(fen) {
            Ok(_) => println!("'{}' - unexpectedly valid", fen),
            Err(e) => println!("'{}' - Error: {}", fen, e),
        }
    }
}
