use chessbot::chessbot::board::Board;
use chessbot::chessbot::engine::Engine;

fn perft(board: Board, depth: usize) -> usize {
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

//https://www.chessprogramming.org/Perft_Results