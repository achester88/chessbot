mod chessbot;
use board::*;
use chessbot::*;
use engine::*;
use std::io::{stdin, stdout, Write};

fn main() {
    let eng = Engine::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq e4 0 1");
    eng.gen_moves();
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
