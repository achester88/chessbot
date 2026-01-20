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

pub fn assert_fen_arr(eng_arr: &mut Vec<Move>, expc_arr: &mut Vec<Move>) {
    for mut i in 0..eng_arr.len() {
        //let (from, to, board) = m;
        //let mut board2 = board.clone();
        eng_arr[i].2.casling &= 0b0000_1111;
    }

    for mut i in 0..expc_arr.len() {
        //let (from, to, board) = m;
        //let mut board2 = board.clone();
        expc_arr[i].2.casling &= 0b0000_1111;
    }

    assert_eq!(eng_arr, expc_arr);
}