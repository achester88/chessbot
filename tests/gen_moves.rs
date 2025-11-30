//use crate::board;
use chessbot::chessbot::{board, engine, bitboard};
use board::{Board};
use engine::{Engine};
use bitboard::{print_bitboard_pos};

mod common;
use common::{fen_arr, one_piece_move};

#[test]
fn pawn_empty() {
    let board = Board::new("8/8/8/8/8/4P3/8/8 w - - 0 1");
    let eng = Engine::new();
    let (pos, bb) = eng.gen_pawn_moves(&board, 20);
    print_bitboard_pos(bb, pos);
    
    assert_eq!(bb, 0x10000000);
}

#[test]
fn bishop_empty() {
    let board = Board::new("8/8/8/8/8/4B3/8/8 w - - 0 1");
    let eng = Engine::new();
    let (pos, bb) = eng.gen_bishop_moves(&board, 20, board.pieces[board.turn]);
    print_bitboard_pos(bb, pos);
    
    assert_eq!(bb, 0x1824428002844);
}

#[test]
fn knight_empty() {
    let board = Board::new("8/8/8/8/8/4N3/8/8 w - - 0 1");
    let eng = Engine::new();
    let (pos, bb) = eng.gen_knight_moves(&board, 20);
    print_bitboard_pos(bb, pos);
    
    assert_eq!(bb, 0x2844004428);
}

#[test]
fn rook_empty() {
    let board = Board::new("8/8/8/8/8/4R3/8/8 w - - 0 1");
    let eng = Engine::new();
    let (pos, bb) = eng.gen_rook_moves(&board, 20, board.pieces[board.turn]);
    print_bitboard_pos(bb, pos);
    
    assert_eq!(bb, 0x1010101010ef1010);
}

#[test]
fn queen_empty() {
    let board = Board::new("8/8/8/8/8/4Q3/8/8 w - - 0 1");
    let eng = Engine::new();
    let (pos, bb) = eng.gen_queen_moves(&board, 20, board.pieces[board.turn]);
    print_bitboard_pos(bb, pos);
    
    assert_eq!(bb, 0x1011925438ef3854);
}

#[test]
fn king_empty() {
    let board = Board::new("8/8/8/8/8/4B3/8/8 w - - 0 1");
    let eng = Engine::new();
    let (pos, bb) = eng.gen_king_moves(&board, 20);
    print_bitboard_pos(bb, pos);
    
    assert_eq!(bb, 0x38283800);
}

#[test]
fn pawn_capture() {
    let board = Board::new("8/8/8/8/8/4B3/8/8 w - - 0 1");
    let eng = Engine::new();
    let (pos, bb) = eng.gen_king_moves(&board, 20);
    print_bitboard_pos(bb, pos);
    
    assert_eq!(bb, 0x38283800);
}
