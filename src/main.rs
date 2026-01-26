mod chessbot;

use board::*;
use chessbot::*;
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
    //"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq e4 0 1"
    //let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    //let board = Board::new("8/8/8/8/3K4/8/8/8 w - - 0 1");
    //let eng = Engine::new();
    //let board = Board::new("8/8/8/8/8/4R3/8/8 w - - 0 1", &eng);
    //println!("{:?}", eng.gen_moves(board));

    let (tx, rx): (Sender<Cmd>, Receiver<Cmd>) = mpsc::channel();

    let eng = Engine::new(); //replace with ref or something :(


    let best_move = Arc::new(Mutex::new((0, 0, Board::new("8/8/8/8/8/8/8/8 w - - 0 1", &eng))));

    //TODO lock best move on search start and release after stop CMD


    /*
    thread::spawn(move || {
        let mut searching = false;
        let mut init_board: Option<Board> = None;

        let engine = Engine::new();

        //let mut best_move_lock = best_move_thread_clone.lock().unwrap();

        loop {

            if searching {
                let mut best_move_lock = best_move_thread_clone.lock().unwrap();

                let moves = engine.gen_moves(init_board.take().unwrap());

                //Testing
                *best_move_lock = moves[0];
                println!("Moves: {:?}", moves);

                let (from, to, board) = moves[0];


                init_board = Some(board);

                searching = false;
                drop(best_move_lock);

                //println!("bestmove {}{}", Board::pos_to_lan(from), Board::pos_to_lan(to));
                //board needs to go back to interface inf
            }

            let rx_mes = rx.recv();

            if !rx_mes.is_err() {
                let msg = rx_mes.unwrap();

                match msg {
                    Cmd::Set(board) => {
                        init_board = Some(board);
                    },
                    Cmd::GoInf => {
                        searching = true;
                        //best_move_lock = best_move_thread_clone.lock().unwrap();
                    }
                    Cmd::Stop => {
                        searching = false;
                    }
                };
            }

            //let mut best_move_lock = best_move_thread_clone.lock().unwrap();
            // *best_move_lock += 1;
            //drop(best_move_lock);
            //println!("{:?}", msg);
        }
        //let val = String::from("hi");
        //tx.send(val).unwrap();
    });
    */

    let stop_calculation = Arc::new(AtomicBool::new(false));;//Rc::new(false);//Arc::new(Mutex::new(false));
    let finished_calculation = Arc::new(AtomicBool::new(false));

    let mut interface = UciInterface::new();
    //let engine = Engine::new();

    //let engine = Arc::new(Mutex::new(Engine::new()));

    loop {
        //tx.send(0).unwrap();
        //let best_move_lock = best_move.lock().unwrap();
        //println!("- {}", *best_move_lock);
        //drop(best_move_lock);
        //thread::sleep(Duration::new(1, 0))

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

                    //let eng_clone = Arc::clone(&engine);

                    let cal_board = interface.current_board.clone();
                    //let eng_ref = &engine;

                    thread::spawn(move || {
                        let mut cur_best_move: Option<Move> = None;
                        let mut self_stop = false;

                        let engine = Engine::new();

                        //let mut eng_ref = best_move_thread_clone.lock().unwrap();

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
                        //let duration = start.elapsed();
                        //if duration.as_millis() > 10 { //Placeholder
                            //println!("info string Time elapsed: {:?}", duration);
                        //}
                    }

                    //shutdown.store(true, Ordering::Relaxed);
                    let best_move_lock = best_move.lock().unwrap();
                    let (from, to, board) = *best_move_lock;
                    match (from, to) {
                        (80, 80) => println!("bestmove O-O"),
                        (88, 88) => println!("bestmove O-O-O"),
                        _ => println!("bestmove {}{}", Board::pos_to_lan(from), Board::pos_to_lan(to))
                    }
                    interface.current_board = Some(board);
                    finished_calculation.store(false, Ordering::Relaxed);
                    stop_calculation.store(false, Ordering::Relaxed);

                },
                _ => {}
            }

            /*
            if cmd == Cmd::Stop || cmd_out.unwrap() == Cmd::GoInf {
                //break; //TODO Replace with wait for bestmove
                let best_move_lock = best_move.lock().unwrap();
                let (from, to, board) = *best_move_lock;
                println!("bestmove {}{}/n", Board::pos_to_lan(from), Board::pos_to_lan(to));
                interface.current_board = Some(board);
            }
            */
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