use chessbot::chessbot::bitboard::print_bitboard_pos;
use chessbot::chessbot::board::Board;
use chessbot::chessbot::engine::Engine;

mod common;
use common::{fen_arr};
//FEN string should be -1 fore moves i.e.
#[test]
fn pawn_base() {
    let board = Board::new("8/8/8/8/8/3p4/2P5/8 w - - 0 1");
    let eng = Engine::new();
    let moves = eng.gen_moves(board);
    println!("Moves: {:?}", moves);

    assert_eq!(moves, fen_arr(10, vec!(
        (18, "8/8/8/8/8/2Pp4/8/8 b - - 1 1"),
        (19, "8/8/8/8/8/3P4/8/8 b - - 1 1"),
        (26, "8/8/8/8/2P5/3p4/8/8 b - c3 1 1"),
    )));
}

#[test]
fn knight_base() {
    let board = Board::new("8/8/2k2n2/8/2pNp3/1r6/4p3/8 w - - 0 1");
    let eng = Engine::new();
    let moves = eng.gen_moves(board);
    println!("Moves: {:?}", moves);

    assert_eq!(moves, fen_arr(27, vec!(
        (10, "8/8/2k2n2/8/2p1p3/1r6/2N1p3/8 b - - 1 1"),
        (12, "8/8/2k2n2/8/2p1p3/1r6/4N3/8 b - - 1 1"),
        (17, "8/8/2k2n2/8/2p1p3/1N6/4p3/8 b - - 1 1"),
        (21, "8/8/2k2n2/8/2p1p3/1r3N2/4p3/8 b - - 1 1"),
        (33, "8/8/2k2n2/1N6/2p1p3/1r6/4p3/8 b - - 1 1"),
        (37, "8/8/2k2n2/5N2/2p1p3/1r6/4p3/8 b - - 1 1"),
        (42, "8/8/2N2n2/8/2p1p3/1r6/4p3/8 b - - 1 1"),
        (44, "8/8/2k1Nn2/8/2p1p3/1r6/4p3/8 b - - 1 1"),
    )));
}

#[test]
fn queen_base() {
    let board = Board::new("8/1K6/6p1/8/1p2Q3/4p3/8/8 w - - 0 1");
    let eng = Engine::new();
    let moves = eng.gen_moves(board);
    println!("Moves: {:?}", moves);

    //Queen
    let mut fen_moves = fen_arr(28, vec!(
        (1,  "8/1K6/6p1/8/1p6/4p3/8/1Q6 b - - 1 1"),
        (7,   "8/1K6/6p1/8/1p6/4p3/8/7Q b - - 1 1"),
        (10, "8/1K6/6p1/8/1p6/4p3/2Q5/8 b - - 1 1"),
        (14, "8/1K6/6p1/8/1p6/4p3/6Q1/8 b - - 1 1"),
        (19,  "8/1K6/6p1/8/1p6/3Qp3/8/8 b - - 1 1"),
        (20,   "8/1K6/6p1/8/1p6/4Q3/8/8 b - - 1 1"),
        (21,  "8/1K6/6p1/8/1p6/4pQ2/8/8 b - - 1 1"),
        (25,   "8/1K6/6p1/8/1Q6/4p3/8/8 b - - 1 1"),
        (26,  "8/1K6/6p1/8/1pQ5/4p3/8/8 b - - 1 1"),
        (27, "8/1K6/6p1/8/1p1Q4/4p3/8/8 b - - 1 1"),
        (29, "8/1K6/6p1/8/1p3Q2/4p3/8/8 b - - 1 1"),
        (30, "8/1K6/6p1/8/1p4Q1/4p3/8/8 b - - 1 1"),
        (31,  "8/1K6/6p1/8/1p5Q/4p3/8/8 b - - 1 1"),
        (35, "8/1K6/6p1/3Q4/1p6/4p3/8/8 b - - 1 1"),
        (36, "8/1K6/6p1/4Q3/1p6/4p3/8/8 b - - 1 1"),
        (37, "8/1K6/6p1/5Q2/1p6/4p3/8/8 b - - 1 1"),
        (42, "8/1K6/2Q3p1/8/1p6/4p3/8/8 b - - 1 1"),
        (44, "8/1K6/4Q1p1/8/1p6/4p3/8/8 b - - 1 1"),
        (46,   "8/1K6/6Q1/8/1p6/4p3/8/8 b - - 1 1"),
        (52, "8/1K2Q3/6p1/8/1p6/4p3/8/8 b - - 1 1"),
        (60, "4Q3/1K6/6p1/8/1p6/4p3/8/8 b - - 1 1"),

    ));

    //King
    fen_moves.append(&mut fen_arr(49, vec!(
        (40,  "8/8/K5p1/8/1p2Q3/4p3/8/8 b - - 1 1"),
        (41, "8/8/1K4p1/8/1p2Q3/4p3/8/8 b - - 1 1"),
        (42, "8/8/2K3p1/8/1p2Q3/4p3/8/8 b - - 1 1"),
        (48,  "8/K7/6p1/8/1p2Q3/4p3/8/8 b - - 1 1"),
        (50, "8/2K5/6p1/8/1p2Q3/4p3/8/8 b - - 1 1"),
        (56,  "K7/8/6p1/8/1p2Q3/4p3/8/8 b - - 1 1"),
        (57, "1K6/8/6p1/8/1p2Q3/4p3/8/8 b - - 1 1"),
        (58, "2K5/8/6p1/8/1p2Q3/4p3/8/8 b - - 1 1"),
    )));

    assert_eq!(moves, fen_moves);
}

//en_passant
#[test]
fn pawn_en_passant() {
    let board = Board::new("8/8/8/3Pp3/8/8/8/8 w - e6 0 1");
    let eng = Engine::new();
    let moves = eng.gen_moves(board);
    println!("Moves: {:?}", moves);

    assert_eq!(moves, fen_arr(35, vec!(
        (43, "8/8/3P4/4p3/8/8/8/8 b - - 1 1"),
        (44, "8/8/4P3/8/8/8/8/8 b - - 1 1")
    )));
}

#[test]
fn pawn_promote() {
    let board = Board::new("8/1P6/8/8/8/8/8/8 w - - 0 1");
    let eng = Engine::new();
    let moves = eng.gen_moves(board);
    println!("Moves: {:?}", moves);

    assert_eq!(moves, fen_arr(49, vec!(
        (57, "1N6/8/8/8/8/8/8/8 b - - 1 1"),
        (57, "1B6/8/8/8/8/8/8/8 b - - 1 1"),
        (57, "1R6/8/8/8/8/8/8/8 b - - 1 1"),
        (57, "1Q6/8/8/8/8/8/8/8 b - - 1 1")
    )));
}

#[test]
fn pawn_promote_black() {
    let board = Board::new("8/8/8/8/8/8/1p6/8 b - - 1 1");
    let eng = Engine::new();
    let moves = eng.gen_moves(board);
    println!("Moves: {:?}", moves);

    assert_eq!(moves, fen_arr(09, vec!(
        (01, "8/8/8/8/8/8/8/1n6 w - - 2 2"),
        (01, "8/8/8/8/8/8/8/1b6 w - - 2 2"),
        (01, "8/8/8/8/8/8/8/1r6 w - - 2 2"),
        (01, "8/8/8/8/8/8/8/1q6 w - - 2 2")
    )));
}

#[test]
fn pawn_capture_promote() {
    let board = Board::new("1p6/2P5/8/8/8/8/8/8 w - - 0 1");
    let eng = Engine::new();
    let moves = eng.gen_moves(board);
    println!("Moves: {:?}", moves);

    assert_eq!(moves, fen_arr(50, vec!(
        (57, "1N6/8/8/8/8/8/8/8 b - - 1 1"),
        (57, "1B6/8/8/8/8/8/8/8 b - - 1 1"),
        (57, "1R6/8/8/8/8/8/8/8 b - - 1 1"),
        (57, "1Q6/8/8/8/8/8/8/8 b - - 1 1")
    )));
}

#[test]
fn king_check() {
    let mut board = Board::new("7b/8/8/4K3/7B/8/8/8 w - - 0 1");
    board.check = 0x8040201000000000;
    let eng = Engine::new();
    //println!("{:?}", eng.gen_bishop_moves(&board, 63, board.pieces[board.turn]));

    let moves = eng.gen_moves(board);
    println!("Moves: {:?}", moves);

    let mut fen_moves = fen_arr(31, vec!(
        (45, "7b/8/5B2/4K3/8/8/8/8 b - - 1 1"),
    ));

    fen_moves.append(&mut fen_arr(36, vec!(
        (28,  "7b/8/8/8/4K2B/8/8/8 b - - 1 1"),
        (29, "7b/8/8/8/5K1B/8/8/8 b - - 1 1"),
        (35, "7b/8/8/3K4/7B/8/8/8 b - - 1 1"),
        (37,  "7b/8/8/5K2/7B/8/8/8 b - - 1 1"),
        (43, "7b/8/3K4/8/7B/8/8/8 b - - 1 1"),
        (44,  "7b/8/4K3/8/7B/8/8/8 b - - 1 1"),
    )));

    assert_eq!(moves, fen_moves);
}