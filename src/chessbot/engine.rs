use super::{bitboard, board};
use bitboard::*;
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
        
        if self.board.white_turn {
          let a = self.gen_ray_attacks(self.board.occupied(), Dir::South, 25);
          print_bitboard(a);
            //print_bitboard(!self.board.black_bishops);
            //gen_attacks();
        }
        return vec![];
    }

    pub fn gen_rook_moves() -> Vec<u8> {
        return vec![];
    }

    pub fn gen_ray_attacks(&self, occupied: u64, dir: Dir, square: usize) -> u64 {
        print_bitboard(occupied);
        print_bitboard(1 << square);
        let set = &self.ray_attacks[dir as usize];
        let mut attack: u64 = set[square];
        let block = attack & occupied;
        if block != 0 {
            let stop: usize;
            if dir_is_pos(dir) {
              stop = bit_scan(block);
            } else {
              stop = bit_scan_neg(block);
            }
            attack = attack ^ set[stop];
        }
        return attack;
    }
    //move fuction that return a new board after that move
}

//https://www.chessprogramming.org/On_an_empty_Board#Rays_by_Line

fn bit_scan(num: u64) -> usize {
  if num != 0 {
    println!("{}", (0b000010001010100_i32).trailing_zeros());
    return num.trailing_zeros() as usize;
  }
  return 0;
}

fn bit_scan_neg(num: u64) -> usize {
  (num.leading_zeros() ^ 63) as usize
}

fn dir_is_pos(dir: Dir) -> bool {
  match dir {
    Dir::North | Dir::NOWE | Dir::NOEA | Dir::East => true,
    Dir::South | Dir::West | Dir::SOWE | Dir::SOEA => false
  }
}

fn gen_attacks() -> Vec<Vec<u64>> {
    
    let mut attacks: Vec<Vec<u64>> = vec![vec![0; 64]; 8];

    //North
    let mut nort: u64 = 0x0101010101010100;
    for i in 0..64 {
        //print_bitboard(&nort);
        attacks[Dir::North as usize][i as usize] = nort;
        nort <<= 1;
    }
    //South
    let mut south: u64 = 0x80808080808080;
    for i in (0..64).rev() {
      //println!("{}", i);
        //print_bitboard(south);
        attacks[Dir::South as usize][i as usize] = south;
          south >>= 1;
    }
  
    //East
    let mut east: u64;
    for i in 0..8 {
      east = 0xfe << (8 * i);
      for ii in 0..8 {
        //print_bitboard(east);
        attacks[Dir::East as usize][((i*8)+ii) as usize] = east;
        east = postshift::east_one(east);
      }
    }

    //West
    let mut west: u64;
    for i in 0..8 {
        west = 0x7f << (8 * i);
      for ii in 0..8 {
        //print_bitboard(west);
        attacks[Dir::West as usize][((i*8)+ii) as usize] = west;
        west = postshift::west_one(west);
      }
    }
  
    return attacks;
}

//https://tearth.dev/bitboard-viewer/