pub mod board;
pub mod engine;

pub fn print_bitboard(bitboard: &u64) {
    println!("-----");
    for r in [7, 6, 5, 4, 3, 2, 1, 0] {
        //cant be bothered
        for f in 0..8 {
            let i = (r * 8) + f;

            if ((bitboard >> i) & 1) == 1 {
                print!("1 ");
            } else {
                print!(". ")
            }
        }
        println!("")
    }
    println!("-----");
}
