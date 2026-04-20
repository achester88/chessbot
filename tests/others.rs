use chicory::chicory::board::Board;
use chicory::chicory::engine::{Engine, Move};
use chicory::chicory::tables::ZobristKeys;

#[test]
fn hash() {
    let eng = Engine::new();
    let board = Board::new("8/8/8/8/8/3p4/2P5/8 w - - 0 1", &eng);

    let zb = ZobristKeys::new();

    println!("{:?}", zb);

    println!("\n\n\n\n");

    println!("key: {:?}", zb.get_key(board));

    assert_eq!(0, 1);
}
