mod chicory;

use board::*;
use chicory::*;
use engine::*;
use uci_interface::*;
use std::io::{stdin, stdout, Write};
use std::rc::{Rc, Weak};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};

fn main() {
    let (tx, rx): (Sender<Cmd>, Receiver<Cmd>) = mpsc::channel();

    let eng = Engine::new(); //replace with ref or something :(


    let best_move = Arc::new(Mutex::new((0, 0, Board::new("8/8/8/8/8/8/8/8 w - - 0 1", &eng), None)));

    //TODO lock best move on search start and release after stop CMD

    let stop_calculation = Arc::new(AtomicBool::new(false));;//Rc::new(false);//Arc::new(Mutex::new(false));
    let finished_calculation = Arc::new(AtomicBool::new(false));

    let mut interface = UciInterface::new();

    loop {
        let _=stdout().flush();
        let mut input=String::new();
        stdin().read_line(&mut input).unwrap();
        let commands: Vec<&str> = input.trim_end().split(" ").collect();
        let cmd_out = match commands[0] {
            "uci" => interface.uci(),
            "isready" => interface.isready(),
            "position" => interface.position(commands),
            "go" => interface.go(),
            "ucinewgame" => interface.uci_new_game(),
            "stop" => interface.stop(),
            "quit" => interface.quit(),
            "setoption" => interface.set_option(),
            _ => { None }

        };

        if cmd_out.is_some() {
            let cmd = cmd_out.clone().unwrap();
            tx.send(cmd).unwrap();

            match cmd_out.unwrap() {
                Cmd::GoInf => {

                    let best_move_thread_clone = Arc::clone(&best_move);

                    let stop_calculation_clone = Arc::clone(&stop_calculation);
                    let finished_calculation_clone = Arc::clone(&finished_calculation);

                    let cal_board = interface.current_board.clone();

                    thread::spawn(move || {
                        let mut cur_best_move: Option<Move> = None;
                        let mut self_stop = false;

                        let engine = Engine::new();

                        while !stop_calculation_clone.load(Ordering::Relaxed) && !self_stop {
                            let new_move = engine.gen_moves(cal_board.unwrap());

                            cur_best_move = Some(new_move[0]);
                            self_stop = true;
                        }

                        let mut best_move_lock = best_move_thread_clone.lock().unwrap();
                        *best_move_lock = cur_best_move.unwrap();
                        finished_calculation_clone.store(true, Ordering::Relaxed);


                    });
                    let start = Instant::now();

                    while !finished_calculation.load(Ordering::Relaxed) {
                    }

                    let best_move_lock = best_move.lock().unwrap();
                    let (from, to, board, promo) = *best_move_lock;
                    println!("bestmove {}", Board::move_to_lan(&*best_move_lock));
                    interface.current_board = Some(board);
                    finished_calculation.store(false, Ordering::Relaxed);
                    stop_calculation.store(false, Ordering::Relaxed);

                },
                _ => {}
            }

       }
    }
}

/*

    (x >> n) & 1 gives you the nth bit,
    x & !(1 << n) clears the nth bit,
    and x | (1 << n) sets the nth bit.

*/

//https://shaack.com/projekte/cm-fen-editor/

//      󰡜 󰡗 󰡘 󰡙 󰡚 󰡛   NERD FONT
//

//TODO REVILED CALSING SQUARE BLOCK
//TODO MAKE CALSING INIT BETTER?
//CUSTOM COMPARE FUNCTION (IGNORE TEMP CASTLING STATE)
