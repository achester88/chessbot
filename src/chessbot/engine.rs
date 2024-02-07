use super::{bitboard, board, utils};
use bitboard::*;
use board::*;
use utils::*;

type Move = (usize, usize, Board);

#[derive(Debug, Clone, Copy)]
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
    //from, to, new board
    pub fn gen_moves(&self) -> Vec<Move> {

        let mut possable: Vec<(usize, u64)> = vec![];
        
        let queens = board_serialize(self.board.queens[self.board.turn]);
        for i in queens {
          possable.push(self.gen_queen_moves(i, self.board.pieces[self.board.turn]));
        }
        
        let mut all_moves: Vec<Move> = vec![];

        for i in 0..possable.len() {
          let (from, moves) = possable[i];
          let moves_to = board_serialize(moves);

          for i in 0..moves_to.len() {
            let to = moves_to[i];
            let new_board = self.board.move_piece(to, from);
            all_moves.push((from, to, new_board));
          }
        }

        return all_moves;
    }

    pub fn gen_rook_moves(&self, sq: usize, pieces: u64) -> (usize, u64) {
        let all_moves = 
        self.gen_ray_attacks(self.board.occupied, Dir::North, sq) | 
        self.gen_ray_attacks(self.board.occupied, Dir::South, sq) | 
        self.gen_ray_attacks(self.board.occupied, Dir::East, sq) | 
        self.gen_ray_attacks(self.board.occupied, Dir::West, sq);

        let attack = all_moves & !pieces;

        //print_bitboard_pos(attack, sq);
        return (sq, attack);//board_serialize(attack);
    }

    pub fn gen_bishop_moves(&self, sq: usize, pieces: u64) -> (usize, u64) {
      let all_moves = 
      self.gen_ray_attacks(self.board.occupied, Dir::NOEA, sq) | 
      self.gen_ray_attacks(self.board.occupied, Dir::NOWE, sq) | 
      self.gen_ray_attacks(self.board.occupied, Dir::SOEA, sq) | 
      self.gen_ray_attacks(self.board.occupied, Dir::SOWE, sq);

      let attack = all_moves & !pieces;

      //print_bitboard_pos(attack, sq);
      return (sq, attack);//board_serialize(attack);
  }

  pub fn gen_queen_moves(&self, sq: usize, pieces: u64) -> (usize, u64) {
    let all_moves = 
    self.gen_rook_moves(sq, self.board.occupied).1 | 
    self.gen_bishop_moves(sq, self.board.occupied).1;

    let attack = all_moves & !pieces;

    //print_bitboard_pos(attack, sq);
    return (sq, attack);//board_serialize(attack);
}

    pub fn gen_ray_attacks(&self, occupied: u64, dir: Dir, square: usize) -> u64 {
        //print_bitboard(occupied);
        //print_bitboard(1 << square);
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
        attacks[Dir::North as usize][i as usize] = nort;
        nort <<= 1;
    }
    //South
    let mut south: u64 = 0x80808080808080;
    for i in (0..64).rev() {
        attacks[Dir::South as usize][i as usize] = south;
          south >>= 1;
    }
  
    //East
    let mut east: u64;
    for i in 0..8 {
      east = 0xfe << (8 * i);
      for ii in 0..8 {
        attacks[Dir::East as usize][((i*8)+ii) as usize] = east;
        east = postshift::east_one(east);
      }
    }

    //West
    let mut west: u64;
    for i in 0..8 {
        west = 0x7f << (8 * i);
      for ii in 0..8 {
        attacks[Dir::West as usize][((i*8)+(7-ii)) as usize] = west;
        west = postshift::west_one(west);
      }
    }

    //noea
    let mut noea;
    for i in 0..8 {
        noea = 0x8040201008040200 << (8*i);
      for ii in 0..8 {
        attacks[Dir::NOEA as usize][((i*8)+ii) as usize] = noea;

        noea = postshift::east_one(noea);
      }
    }

    //nowe
    let mut nowe: u64 = 0x8040201008040200;
    for i in 0..8 {
      nowe = 0x102040810204000 << (8*i);
      for ii in 0..8 {
        attacks[Dir::NOWE as usize][((i*8)+(7-ii)) as usize] = nowe;

        nowe = postshift::west_one(nowe);
      }
    }

    //soea
    let mut soea: u64;
    for i in (0..8).rev() {
      soea = 0x2040810204080 >> (8*(7-i));
      for ii in 0..8 {
        attacks[Dir::SOEA as usize][((i*8)+ii) as usize] = soea;

        soea = postshift::east_one(soea);
      }
    }

    //sowe
    let mut sowe: u64;
    for i in (0..8).rev() {
      sowe = 0x40201008040201 >> (8*(7-i));
      for ii in 0..8 {
        attacks[Dir::SOWE as usize][((i*8)+(7-ii)) as usize] = sowe;

        sowe = postshift::west_one(sowe);
      }
    }
  
    /*for i in 0..64 {
      print_bitboard_pos(attacks[Dir::SOWE as usize][i],i);
    }*/

    return attacks;
}

//https://tearth.dev/bitboard-viewer/