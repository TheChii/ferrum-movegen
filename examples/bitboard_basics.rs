//! Bitboard operations and attack tables example.
//!
//! Bitboards are 64-bit integers where each bit represents a square on the chessboard.
//! They enable fast set operations for piece movement and attack calculations.
//!
//! Run with: cargo run --example bitboard_basics

use movegen::{Board, Bitboard, Square, Color, Piece};
use movegen::attacks::{knight_attacks, king_attacks, bishop_attacks, rook_attacks, pawn_attacks};

fn main() {
    println!("=== Ferrum Movegen: Bitboard Basics ===\n");

    // --- Bitboard Basics ---
    println!("--- Bitboard Fundamentals ---");
    
    // Creating bitboards
    let empty = Bitboard::EMPTY;
    let e4 = Bitboard::from_square(Square::E4);
    
    println!("Empty bitboard: {:064b}", empty.0);
    println!("E4 square:      bits set = {}", e4.count());
    
    // Set operations
    let d4 = Bitboard::from_square(Square::D4);
    let center = e4 | d4 | Bitboard::from_square(Square::E5) | Bitboard::from_square(Square::D5);
    
    println!("Center squares (d4, e4, d5, e5): {} squares", center.count());

    // --- Attack Tables ---
    println!("\n--- Piece Attacks ---");
    
    // Knight attacks
    let knight_on_e4 = knight_attacks(Square::E4);
    println!("Knight on e4 attacks {} squares", knight_on_e4.count());
    print!("  Squares: ");
    for sq in knight_on_e4 {
        print!("{:?} ", sq);
    }
    println!();

    // King attacks
    let king_on_e4 = king_attacks(Square::E4);
    println!("King on e4 attacks {} squares", king_on_e4.count());

    // Bishop attacks (need occupancy)
    let occupied = Bitboard::EMPTY; // Empty board
    let bishop_on_e4 = bishop_attacks(Square::E4, occupied);
    println!("Bishop on e4 (empty board) attacks {} squares", bishop_on_e4.count());
    
    // Rook attacks
    let rook_on_e4 = rook_attacks(Square::E4, occupied);
    println!("Rook on e4 (empty board) attacks {} squares", rook_on_e4.count());

    // With blockers
    let blockers = Bitboard::from_square(Square::E7) | Bitboard::from_square(Square::B4);
    let bishop_blocked = bishop_attacks(Square::E4, blockers);
    let rook_blocked = rook_attacks(Square::E4, blockers);
    println!("\nWith blockers on e7 and b4:");
    println!("  Bishop attacks {} squares", bishop_blocked.count());
    println!("  Rook attacks {} squares", rook_blocked.count());

    // Pawn attacks - note: pawn_attacks takes (color, square)
    let white_pawn_attacks = pawn_attacks(Color::White, Square::E4);
    let black_pawn_attacks = pawn_attacks(Color::Black, Square::E4);
    println!("\nPawn on e4:");
    println!("  White pawn attacks: {} squares (d5, f5)", white_pawn_attacks.count());
    println!("  Black pawn attacks: {} squares (d3, f3)", black_pawn_attacks.count());

    // --- Working with Board State ---
    println!("\n--- Board Bitboards ---");
    
    let board = Board::default();
    
    // Color bitboards
    let white_pieces = board.color_bb(Color::White);
    let black_pieces = board.color_bb(Color::Black);
    println!("White pieces: {} total", white_pieces.count());
    println!("Black pieces: {} total", black_pieces.count());
    
    // Piece type bitboards
    let all_pawns = board.piece_bb(Piece::Pawn);
    let all_knights = board.piece_bb(Piece::Knight);
    println!("Total pawns: {}", all_pawns.count());
    println!("Total knights: {}", all_knights.count());
    
    // Combined queries
    let white_pawns = board.piece_color_bb(Piece::Pawn, Color::White);
    println!("White pawns: {}", white_pawns.count());
    
    // Occupied and empty
    let occupied = board.occupied();
    let empty = board.empty_squares();
    println!("Occupied squares: {}", occupied.count());
    println!("Empty squares: {}", empty.count());

    // --- Iterating over Bitboards ---
    println!("\n--- Iterating over Squares ---");
    
    let board = Board::from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1").unwrap();
    let rooks = board.piece_bb(Piece::Rook);
    
    println!("Rook positions:");
    for sq in rooks {
        if let Some((_, color)) = board.piece_at(sq) {
            println!("  {:?} ({:?})", sq, color);
        }
    }

    // --- Attack Detection ---
    println!("\n--- Attack Detection ---");
    
    let board = Board::from_fen("r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 2 3").unwrap();
    
    // Who attacks e5?
    let e5 = Square::E5;
    let attackers = board.attackers_to(e5, board.occupied());
    
    println!("Attackers to e5:");
    for sq in attackers {
        if let Some((piece, color)) = board.piece_at(sq) {
            println!("  {:?} {:?} on {:?}", color, piece, sq);
        }
    }

    // Is the king in check?
    let king_sq = board.king_square(board.turn());
    let enemy = !board.turn();
    let enemy_attacks = board.attackers_to(king_sq, board.occupied()) & board.color_bb(enemy);
    
    println!("\n{:?} king on {:?}", board.turn(), king_sq);
    println!("In check: {}", !enemy_attacks.is_empty());
    println!("Checkers: {:?}", board.checkers());
}
