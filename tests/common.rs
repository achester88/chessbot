use chessbot::chessbot::{board, engine};
use board::{Board, PieceType, PieceColor};
use engine::{Move};

pub fn fen_arr(from: usize, new_boards: Vec<(usize, &str)> ) -> Vec<Move> {
    let mut boards: Vec<Move> = vec![];
    let eng = engine::Engine::new(); //Only for testing 

    for (to, fen) in new_boards.iter() {
        boards.push( (from, *to, Board::new(fen, &eng)) );
    }

    return boards;
}
