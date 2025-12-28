//! Making and unmaking moves example.
//!
//! Demonstrates both copy-make and make/unmake patterns for traversing game trees.
//!
//! Run with: cargo run --example move_making

use movegen::{Board, Move, MoveFlag, Square};

fn main() {
    println!("=== Ferrum Movegen: Making Moves ===\n");

    // Start with the initial position
    let board = Board::default();
    println!("Starting position: {}", board.to_fen());

    // --- Copy-Make Pattern (Recommended) ---
    println!("\n--- Copy-Make Pattern ---");
    println!("This pattern creates a new board for each move.");
    println!("It's simpler and often faster for small search depths.\n");

    // Play 1. e4
    let e2e4 = find_move(&board, "e2e4").expect("e2e4 should be legal");
    let board_after_e4 = board.make_move_new(e2e4);
    println!("After 1. e4: {}", board_after_e4.to_fen());
    
    // Original board is unchanged
    println!("Original still: {}", board.to_fen());

    // Continue the game: 1... e5 2. Nf3 Nc6 3. Bb5 (Ruy Lopez)
    let moves = ["e7e5", "g1f3", "b8c6", "f1b5"];
    let mut current = board_after_e4;
    
    for uci in moves {
        let mv = find_move(&current, uci).expect("Move should be legal");
        current = current.make_move_new(mv);
    }
    println!("Ruy Lopez after 3. Bb5: {}", current.to_fen());

    // --- Make/Unmake Pattern ---
    println!("\n--- Make/Unmake Pattern ---");
    println!("This pattern modifies the board in place and can undo moves.");
    println!("Useful when you need to restore the exact board state.\n");

    let mut board = Board::default();
    let initial_hash = board.hash();
    println!("Initial hash: {:016x}", initial_hash);

    // Make a move
    let e2e4 = find_move(&board, "e2e4").expect("e2e4 should be legal");
    let undo = board.make_move(e2e4);
    println!("After e2e4 hash: {:016x}", board.hash());

    // Unmake the move
    board.unmake_move(e2e4, undo);
    println!("After unmake hash: {:016x}", board.hash());
    println!("Hashes match: {}", board.hash() == initial_hash);

    // --- Building a Game Tree ---
    println!("\n--- Game Tree Traversal ---");
    println!("Counting positions at depth 3 using copy-make...\n");

    let board = Board::default();
    let positions = count_positions(&board, 3);
    println!("Positions at depth 3: {}", positions);

    // --- Special Moves ---
    println!("\n--- Special Moves ---");
    
    // Castling
    let castling_pos = "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1";
    let board = Board::from_fen(castling_pos).unwrap();
    
    let kingside = find_move(&board, "e1g1");
    let queenside = find_move(&board, "e1c1");
    
    println!("Castling position: {}", castling_pos);
    println!("Kingside castle available: {}", kingside.is_some());
    println!("Queenside castle available: {}", queenside.is_some());

    if let Some(mv) = kingside {
        let after = board.make_move_new(mv);
        println!("After O-O: {}", after.to_fen());
    }

    // En passant
    let ep_pos = "rnbqkbnr/ppp1p1pp/8/3pPp2/8/8/PPPP1PPP/RNBQKBNR w KQkq f6 0 1";
    let board = Board::from_fen(ep_pos).unwrap();
    
    println!("\nEn passant position: {}", ep_pos);
    println!("En passant square: {:?}", board.ep_square());
    
    let ep_capture = find_move(&board, "e5f6");
    if let Some(mv) = ep_capture {
        println!("En passant capture available: e5f6");
        let after = board.make_move_new(mv);
        println!("After exf6 e.p.: {}", after.to_fen());
    }

    // Promotion
    let promo_pos = "8/P7/8/8/8/8/8/4K2k w - - 0 1";
    let board = Board::from_fen(promo_pos).unwrap();
    
    println!("\nPromotion position: {}", promo_pos);
    
    let moves = board.generate_moves();
    let promotions: Vec<_> = moves.iter()
        .filter(|m| m.is_promotion())
        .collect();
    
    println!("Promotion moves available:");
    for mv in &promotions {
        println!("  {}", mv.to_uci());
    }
    
    // Make the queen promotion
    if let Some(mv) = promotions.iter().find(|m| m.to_uci() == "a7a8q") {
        let after = board.make_move_new(*mv);
        println!("After a8=Q: {}", after.to_fen());
    }
}

/// Find a legal move by its UCI string.
fn find_move(board: &Board, uci: &str) -> Option<Move> {
    board.generate_moves()
        .iter()
        .find(|m| m.to_uci() == uci)
}

/// Count positions at a given depth (minimax-style traversal).
fn count_positions(board: &Board, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }
    
    let moves = board.generate_moves();
    let mut count = 0;
    
    for mv in moves.iter() {
        let new_board = board.make_move_new(mv);
        count += count_positions(&new_board, depth - 1);
    }
    
    count
}
