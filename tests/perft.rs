use chessbot::chessbot::board::Board;
use chessbot::chessbot::engine::Engine;

fn perft(board: Board, depth: usize) -> usize {
    println!("--------------------[ Perft depth: {} ]---------------------", depth);
    let mut count = 0;

    if depth == 0 {
        return 1;
    }

    let eng = Engine::new();
    let moves = eng.gen_moves(board);
    println!("moves count: {:?}", moves);
    for m in moves {
        let (_, _, new_board) = m;
        count += perft(new_board, depth - 1);
    }

    return count;
}

#[test]
fn ip_one() {
    let engine = Engine::new();
    let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &engine);
    let count = perft(board, 1);

    assert_eq!(count, 20);
}

#[test]
fn ip_two() {
    let engine = Engine::new();
    let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &engine);
    let count = perft(board, 2);

    assert_eq!(count, 400);
}

#[test]
fn p2_one() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 1 1", &engine);
    let count = perft(board, 1);

    assert_eq!(count, 48);
}

#[test]
fn p3_one() {
    let engine = Engine::new();
    let board = Board::new("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1 ", &engine);
    let count = perft(board, 1);

    assert_eq!(count, 14);
}

#[test]
fn p3_two() {
    let engine = Engine::new();
    let board = Board::new("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1 ", &engine);
    let count = perft(board, 2);

    assert_eq!(count, 191);
}
//https://www.chessprogramming.org/Perft_Results