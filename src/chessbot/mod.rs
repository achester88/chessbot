pub mod board;

pub fn print_board(bitboard: &u64) {
    for i in 0..64 {
        print!("{} ", (bitboard >> i) & 1);
        if (i+1) % 8 == 0 {
            println!();
        }
    }
}