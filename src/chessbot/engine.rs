use super::{board, print_bitboard};
use board::*;

#[derive(Debug)]
pub enum Dir {
    North,
    South,
    West,
    East,
    NOEA,
    NOWE,
    SOWE,
    SOEA,
}

pub struct Engine {
    ray_attacks: Vec<Vec<u64>>,
    board: Board,
}

impl Engine {
    pub fn new(init_pos: &str) -> Self {
        return Engine {
            ray_attacks: gen_attacks(),
            board: Board::new(init_pos),
        };
    }

    pub fn gen_moves(self) -> Vec<(u8, u8)> {
        self.board.print_board();

        if self.board.white_turn {
            print_bitboard(&(!self.board.black_bishops));
            gen_attacks();
        }
        return vec![];
    }

    pub fn gen_rook_moves() -> Vec<u8> {
        return vec![];
    }

    pub fn gen_ray_attacks(occupied: u64, direction: Dir, square: usize) -> u64 {
        let attack: u64 = 0; //ATTACKS[Dir::East as usize][square];
        let block = attack & occupied;
        if block != 0 {
            let stop: u64;
            if direction as usize > 0 {
                //stop = bitScan(block);
            } else {
                //stop = bitScanNeg(block);
            }
            // attack = attack ^ rayAttacks[dir8][stop];
        }
        return attack;
    }
    //move fuction that return a new board after that move
}


//https://www.chessprogramming.org/On_an_empty_Board#Rays_by_Line
fn gen_attacks() -> Vec<Vec<u64>> {
    let mut nort = 0x0101010101010100;
    let mut attacks: Vec<Vec<u64>> = vec![vec![0;8];64];
    for i in 0..64 {
        //print_bitboard(&nort);
        attacks[i as usize][Dir::North as usize] = nort;
        nort <<= 1;
    }
    print_bitboard(&attacks[63][Dir::North as usize]);
    return attacks;
}
