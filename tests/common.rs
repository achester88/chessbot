use board::Board;
use chicory::chicory::{board, engine};
use engine::Move;

pub fn fen_arr(from: usize, new_boards: Vec<(usize, &str)>) -> Vec<Move> {
    let mut boards: Vec<Move> = vec![];
    let eng = engine::Engine::new(); //Only for testing

    for (to, fen) in new_boards.iter() {
        boards.push((from, *to, Board::new(fen, &eng), None));
    }

    boards
}

pub fn assert_fen_arr(eng_arr: &mut Vec<Move>, expc_arr: &mut Vec<Move>) {
    for i in 0..eng_arr.len() {
        //let mut board2 = board.clone();

        //Would be better to add proper check, but would need to change all fen values in all test
        eng_arr[i].2.castling &= 0b0000_1111;
        eng_arr[i].3 = None;
    }

    for i in 0..expc_arr.len() {
        //let (from, to, board) = m;
        //let mut board2 = board.clone();
        expc_arr[i].2.castling &= 0b0000_1111;
    }

    assert_eq!(eng_arr, expc_arr);
}
