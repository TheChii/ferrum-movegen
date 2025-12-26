#!/usr/bin/env python3
"""
Validation tests for the Rust move generator using python-chess as reference.
Tests random positions, Kiwipete, and other tricky positions.
"""

import subprocess
import chess
import random
import sys
from pathlib import Path

# Path to the compiled Rust CLI
MOVEGEN_CLI = Path(__file__).parent.parent / "target" / "release" / "movegen_cli.exe"

def get_rust_moves(fen: str) -> set[str]:
    """Get legal moves from our Rust move generator."""
    try:
        result = subprocess.run(
            [str(MOVEGEN_CLI), fen],
            capture_output=True,
            text=True,
            timeout=5
        )
        if result.returncode != 0:
            print(f"Error from Rust: {result.stderr}")
            return set()
        return set(result.stdout.strip().split('\n')) if result.stdout.strip() else set()
    except Exception as e:
        print(f"Failed to run Rust CLI: {e}")
        return set()

def get_rust_perft(fen: str, depth: int) -> int:
    """Get perft count from our Rust move generator."""
    try:
        result = subprocess.run(
            [str(MOVEGEN_CLI), "perft", fen, str(depth)],
            capture_output=True,
            text=True,
            timeout=60
        )
        if result.returncode != 0:
            print(f"Error from Rust: {result.stderr}")
            return -1
        return int(result.stdout.strip())
    except Exception as e:
        print(f"Failed to run Rust CLI: {e}")
        return -1

def get_python_moves(board: chess.Board) -> set[str]:
    """Get legal moves from python-chess."""
    return set(move.uci() for move in board.legal_moves)

def get_python_perft(board: chess.Board, depth: int) -> int:
    """Get perft count from python-chess."""
    if depth == 0:
        return 1
    if depth == 1:
        return len(list(board.legal_moves))
    
    nodes = 0
    for move in board.legal_moves:
        board.push(move)
        nodes += get_python_perft(board, depth - 1)
        board.pop()
    return nodes

def compare_moves(fen: str, verbose: bool = True) -> bool:
    """Compare moves from Rust and Python for a given position."""
    board = chess.Board(fen)
    
    rust_moves = get_rust_moves(fen)
    python_moves = get_python_moves(board)
    
    if rust_moves == python_moves:
        if verbose:
            print(f"✓ {len(rust_moves)} moves match")
        return True
    else:
        missing = python_moves - rust_moves
        extra = rust_moves - python_moves
        
        print(f"✗ MISMATCH for FEN: {fen}")
        if missing:
            print(f"  Missing moves: {sorted(missing)}")
        if extra:
            print(f"  Extra moves: {sorted(extra)}")
        return False

def test_startpos():
    """Test starting position."""
    print("\n=== Testing Starting Position ===")
    fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    return compare_moves(fen)

def test_kiwipete():
    """Test Kiwipete position - complex with many tactical elements."""
    print("\n=== Testing Kiwipete ===")
    fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"
    success = compare_moves(fen)
    
    # Also test perft
    board = chess.Board(fen)
    for depth in [1, 2, 3]:
        rust_perft = get_rust_perft(fen, depth)
        python_perft = get_python_perft(board, depth)
        if rust_perft == python_perft:
            print(f"  ✓ Perft({depth}) = {rust_perft}")
        else:
            print(f"  ✗ Perft({depth}) mismatch: Rust={rust_perft}, Python={python_perft}")
            success = False
    
    return success

def test_tricky_positions():
    """Test various tricky positions."""
    print("\n=== Testing Tricky Positions ===")
    
    positions = [
        # Position 3 - en passant and promotion
        ("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", "Position 3"),
        # Position 4 - promotions
        ("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", "Position 4"),
        # Position 5 - discovered check
        ("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", "Position 5"),
        # En passant capture prevents check
        ("8/8/8/8/k2Pp2Q/8/8/3K4 b - d3 0 1", "EP prevents check"),
        # Double check - only king can move
        ("r1bqkb1r/pppp1Qpp/2n2n2/4p2N/2B1P3/8/PPPP1PPP/RNB1K2R b KQkq - 0 1", "Double check"),
        # Castling through check (illegal)
        ("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1", "Castling test"),
        # Pinned piece
        ("rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 0 1", "Pinned pawn"),
    ]
    
    all_pass = True
    for fen, name in positions:
        print(f"\n  {name}:")
        if not compare_moves(fen, verbose=True):
            all_pass = False
    
    return all_pass

def test_random_positions(num_tests: int = 100, max_moves: int = 50):
    """Test randomly generated positions."""
    print(f"\n=== Testing {num_tests} Random Positions ===")
    
    passed = 0
    failed = 0
    
    for i in range(num_tests):
        # Start from initial position
        board = chess.Board()
        
        # Make random moves
        num_moves = random.randint(0, max_moves)
        for _ in range(num_moves):
            if board.is_game_over():
                break
            move = random.choice(list(board.legal_moves))
            board.push(move)
        
        if board.is_game_over():
            continue
        
        fen = board.fen()
        
        rust_moves = get_rust_moves(fen)
        python_moves = get_python_moves(board)
        
        if rust_moves == python_moves:
            passed += 1
        else:
            failed += 1
            missing = python_moves - rust_moves
            extra = rust_moves - python_moves
            print(f"\n✗ Test {i+1} FAILED")
            print(f"  FEN: {fen}")
            if missing:
                print(f"  Missing: {sorted(missing)}")
            if extra:
                print(f"  Extra: {sorted(extra)}")
            
            # Stop after first few failures for debugging
            if failed >= 5:
                print("\n  ... stopping after 5 failures")
                break
    
    print(f"\n  Results: {passed}/{passed+failed} passed")
    return failed == 0

def test_random_perft(num_tests: int = 20, depth: int = 2):
    """Test perft on random positions."""
    print(f"\n=== Testing Perft on {num_tests} Random Positions (depth {depth}) ===")
    
    passed = 0
    failed = 0
    
    for i in range(num_tests):
        board = chess.Board()
        
        # Make some random moves
        num_moves = random.randint(5, 30)
        for _ in range(num_moves):
            if board.is_game_over():
                break
            move = random.choice(list(board.legal_moves))
            board.push(move)
        
        if board.is_game_over():
            continue
        
        fen = board.fen()
        
        rust_perft = get_rust_perft(fen, depth)
        python_perft = get_python_perft(board, depth)
        
        if rust_perft == python_perft:
            passed += 1
            print(f"  ✓ Test {i+1}: {rust_perft} nodes")
        else:
            failed += 1
            print(f"  ✗ Test {i+1} FAILED: Rust={rust_perft}, Python={python_perft}")
            print(f"    FEN: {fen}")
            
            if failed >= 3:
                print("\n  ... stopping after 3 failures")
                break
    
    print(f"\n  Results: {passed}/{passed+failed} passed")
    return failed == 0

def main():
    print("=" * 60)
    print("  Move Generator Validation with python-chess")
    print("=" * 60)
    
    # Check if CLI exists
    if not MOVEGEN_CLI.exists():
        print(f"\nError: Rust CLI not found at {MOVEGEN_CLI}")
        print("Please run: cargo build --release")
        sys.exit(1)
    
    all_pass = True
    
    all_pass &= test_startpos()
    all_pass &= test_kiwipete()
    all_pass &= test_tricky_positions()
    all_pass &= test_random_positions(num_tests=100)
    all_pass &= test_random_perft(num_tests=20, depth=2)
    
    print("\n" + "=" * 60)
    if all_pass:
        print("  ALL TESTS PASSED ✓")
    else:
        print("  SOME TESTS FAILED ✗")
    print("=" * 60)
    
    sys.exit(0 if all_pass else 1)

if __name__ == "__main__":
    main()
