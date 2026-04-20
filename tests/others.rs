use chicory::chicory::board::{Board, PieceColor};
use chicory::chicory::engine::{Engine, Move};
use chicory::chicory::tables::ZobristKeys;

#[test]
fn hash() {
    let eng = Engine::new();
    let board = Board::new("8/8/8/8/8/3p4/2P5/8 w - - 0 1", &eng);
    let nbord = Board::new("8/8/8/8/8/2Pp4/8/8 b - - 0 1", &eng);

    //let zb = ZobristKeys::new();
    let ph = board.zobrist_hash;

    println!("PH: {:?}", ph);

    let moves = eng.gen_moves(board);

    //println!("Moves List: {:?}", moves);

    println!("MH: {:?}", moves[0].board.zobrist_hash);

    let mut nh = ph;

    nh ^= eng.zobrist_keys.pawns[PieceColor::White][10];
    nh ^= eng.zobrist_keys.pawns[PieceColor::White][18];
    nh ^= eng.zobrist_keys.black_turn;

    println!("NH: {:?}", nh);

    let ch = nbord.zobrist_hash;

    println!("CH: {:?}", ch);

    println!("\n\n\n\n");

    //println!("key: {:?}", zb.get_key(board));



    assert_eq!(0, 1);
}
