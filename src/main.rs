mod chessbot;
use board::*;
use chessbot::*;
use engine::*;
use std::io::{stdin, stdout, Write};

fn main() {
    //"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq e4 0 1"
    //let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    //let board = Board::new("8/8/8/8/3K4/8/8/8 w - - 0 1");
    let board = Board::new("8/8/8/8/8/4R3/8/8 w - - 0 1");
    let eng = Engine::new();
    println!("{:?}", eng.gen_moves(board));
    /*loop {
        let _=stdout().flush();
        let mut input=String::new();
        stdin().read_line(&mut input).unwrap();
        let commands: Vec<&str> = input.split(" ").collect();

        match commands[0] {
            "uci\n" => {//option //uciok
                println!("id name My Engine");
                println!("id author Anthony Chester");
                println!("uciok");
            }
            _ => {}
        }
        println!("Cmd: |{}|", commands[0]);
    }*/
}

/*

    (x >> n) & 1 gives you the nth bit,
    x & !(1 << n) clears the nth bit,
    and x | (1 << n) sets the nth bit.

*/

//https://shaack.com/projekte/cm-fen-editor/
