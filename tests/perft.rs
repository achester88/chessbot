use chessbot::chessbot::board::Board;
use chessbot::chessbot::engine::Engine;

fn perft(eng: &Engine, board: Board, depth: usize) -> usize {
    //println!("--------------------[ Perft depth: {} ]---------------------", depth);
    let mut count = 0;

    if depth == 0 {
        return 1;
    }

    let moves = eng.gen_moves(board);
    //println!("moves count: {:?}", moves);
    for m in moves {
        let (from, to, new_board) = m;
        //new_board.print_board();
        count += perft(&eng, new_board, depth - 1);
    }

    return count;
}

#[test]
fn ip_one() {
    let engine = Engine::new();
    let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &engine);
    let count = perft(&engine, board, 1);

    assert_eq!(count, 20);
}

#[test]
fn ip_two() {
    let engine = Engine::new();
    let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &engine);
    let count = perft(&engine, board, 2);

    assert_eq!(count, 400);
}

#[test]
fn ip_three() {
    let engine = Engine::new();
    let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &engine);
    let count = perft(&engine, board, 3);

    assert_eq!(count, 8902);
}

#[test]
fn ip_four() {
    let engine = Engine::new();
    let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &engine);
    let count = perft(&engine, board, 4);

    assert_eq!(count, 197281);
}

#[test]
fn ip_five() {
    let engine = Engine::new();
    let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &engine);
    let count = perft(&engine, board, 5);

    assert_eq!(count, 4865609);
}

#[test]
fn p2_one() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 1 1", &engine);
    let count = perft(&engine, board, 1);

    assert_eq!(count, 48);
}

#[test]
fn p2_two() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 1 1", &engine);
    let count = perft(&engine, board, 2);

    assert_eq!(count, 2039);
}

#[test]
fn p2_three() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 1 1", &engine);
    let count = perft(&engine, board, 3);

    assert_eq!(count, 97862);
}

#[test]
fn p3_one() {
    let engine = Engine::new();
    let board = Board::new("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", &engine);
    let count = perft(&engine, board, 1);

    assert_eq!(count, 14);
}

#[test]
fn p3_two() {
    let engine = Engine::new();
    let board = Board::new("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", &engine);
    let count = perft(&engine, board, 2);

    assert_eq!(count, 191);
}

#[test]
fn p3_three() { //E.P?
    let engine = Engine::new();
    let board = Board::new("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", &engine);
    let count = perft(&engine, board, 3);

    assert_eq!(count, 2812);
}

#[test]
fn p4_one() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", &engine);
    let count = perft(&engine, board, 1);

    assert_eq!(count, 6);
}

#[test]
fn p4_two() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", &engine);
    let count = perft(&engine, board, 2);

    assert_eq!(count, 264);
}

#[test]
fn p4_three() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", &engine);
    let count = perft(&engine, board, 3);

    assert_eq!(count, 9467);
}

#[test]
fn p5_one() {
    let engine = Engine::new();
    let board = Board::new("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", &engine);
    let count = perft(&engine, board, 1);

    assert_eq!(count, 44);
}

#[test]
fn p5_two() {
    let engine = Engine::new();
    let board = Board::new("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", &engine);
    let count = perft(&engine, board, 2);

    assert_eq!(count, 1486);
}

#[test]
fn p5_three() {
    let engine = Engine::new();
    let board = Board::new("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", &engine);
    let count = perft(&engine, board, 3);

    assert_eq!(count, 62379);
}

#[test]
fn p6_one() {
    let engine = Engine::new();
    let board = Board::new("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", &engine);
    let count = perft(&engine, board, 1);

    assert_eq!(count, 46);
}

#[test]
fn p6_two() {
    let engine = Engine::new();
    let board = Board::new("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", &engine);
    let count = perft(&engine, board, 2);

    assert_eq!(count, 2079);
}

#[test]
fn p6_three() {
    let engine = Engine::new();
    let board = Board::new("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", &engine);
    let count = perft(&engine, board, 3);

    assert_eq!(count, 89890);
}

#[test]
fn p6_four() {
    let engine = Engine::new();
    let board = Board::new("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", &engine);
    let count = perft(&engine, board, 4);

    assert_eq!(count, 3894594);
}

#[test]
fn custom() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/8/8/8/8/8/8/R3K1R1 w Qkq - 0 1", &engine);
    let count = perft(&engine, board, 2);

    assert_eq!(count, 547);
}

#[test]
fn custom_c() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/8/8/8/8/8/8/R3K1R1 w Qkq - 0 1", &engine);

    let new_board = board.move_lan(&engine, "a1b1");
    let count = perft(&engine, board, 1);

    assert_eq!(perft(&engine, board.move_lan(&engine, "a1b1"), 1), 25);
    assert_eq!(perft(&engine, board.move_lan(&engine, "a1c1"), 1), 24);
    assert_eq!(perft(&engine, board.move_lan(&engine, "a1d1"), 1), 22);
    assert_eq!(perft(&engine, board.move_lan(&engine, "a1a2"), 1), 24);
    assert_eq!(perft(&engine, board.move_lan(&engine, "a1a3"), 1), 23);
    assert_eq!(perft(&engine, board.move_lan(&engine, "a1a4"), 1), 22);
    assert_eq!(perft(&engine, board.move_lan(&engine, "a1a5"), 1), 21);
    assert_eq!(perft(&engine, board.move_lan(&engine, "a1a6"), 1), 20);
    assert_eq!(perft(&engine, board.move_lan(&engine, "a1a7"), 1), 16);
    assert_eq!(perft(&engine, board.move_lan(&engine, "a1a8"), 1), 3);

    assert_eq!(perft(&engine, board.move_lan(&engine, "g1f1"), 1), 23);
    assert_eq!(perft(&engine, board.move_lan(&engine, "g1h1"), 1), 26);
    assert_eq!(perft(&engine, board.move_lan(&engine, "g1g2"), 1), 25);
    assert_eq!(perft(&engine, board.move_lan(&engine, "g1g3"), 1), 25);
    assert_eq!(perft(&engine, board.move_lan(&engine, "g1g4"), 1), 25);
    assert_eq!(perft(&engine, board.move_lan(&engine, "g1g5"), 1), 25);
    assert_eq!(perft(&engine, board.move_lan(&engine, "g1g6"), 1), 25);
    assert_eq!(perft(&engine, board.move_lan(&engine, "g1g7"), 1), 22);
    assert_eq!(perft(&engine, board.move_lan(&engine, "g1g8"), 1), 4);

    assert_eq!(perft(&engine, board.move_lan(&engine, "e1d1"), 1), 25);
    assert_eq!(perft(&engine, board.move_lan(&engine, "e1f1"), 1), 25);
    assert_eq!(perft(&engine, board.move_lan(&engine, "e1d2"), 1), 25);
    assert_eq!(perft(&engine, board.move_lan(&engine, "e1e2"), 1), 25);
    assert_eq!(perft(&engine, board.move_lan(&engine, "e1f2"), 1), 25);
    let (_, _, cm) = board.castle(88);
    assert_eq!(perft(&engine, cm, 1), 22);
}

#[test]
fn custom_c_1() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/8/8/8/8/8/8/R3K1R1 w Qkq - 0 1", &engine);
    println!("-----------");
    let (_, _, cm) = board.castle(88);
    assert_eq!(perft(&engine, cm, 1), 22);


}

//Rook Capture, respawn when calsing

/*
#[test]
fn p6_five() {
    let engine = Engine::new();
    let board = Board::new("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", &engine);
    let count = perft(board, 5);

    assert_eq!(count, 164075551);
}
*/
//https://www.chessprogramming.org/Perft_Results
//http://www.rocechess.ch/perft.html