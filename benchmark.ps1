$positions = @(
    @{Name="StartPos"; Fen="rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"; Depth=7},
    @{Name="Kiwipete"; Fen="r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"; Depth=6},
    @{Name="Pos3"; Fen="8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1"; Depth=8},
    @{Name="Pos4"; Fen="r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1"; Depth=6},
    @{Name="Pos5"; Fen="rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8"; Depth=5},
    @{Name="Pos6"; Fen="r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10"; Depth=5}
)

foreach ($pos in $positions) {
    Write-Host "Running $($pos.Name)..."
    & target\release\movegen_cli.exe perft $pos.Fen $pos.Depth
    Write-Host "--------------------------------"
}
