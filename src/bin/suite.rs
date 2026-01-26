/*
  TO USE RUN
    cargo run -- {filename.epd}
 */

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::env;
use std::process::ExitCode;
use std::time::Instant;
use chessbot::chessbot::board::Board;
use chessbot::chessbot::engine::Engine;

#[derive(Debug, Clone)]
struct LineInfo {
    fen: String,
    node_count: Vec<usize>,
}

enum Status {
    Passed,
    Acceptable,
    Failed
}


fn perft(eng: &Engine, board: Board, depth: usize) -> usize {
    //println!("--------------------[ Perft depth: {} ]---------------------", depth);
    let mut count = 0;

    if depth == 0 {
        return 1;
    }

    let moves = eng.gen_moves(board);
    //println!("moves count: {:?}", moves);
    for m in moves {
        let (_, _, new_board) = m;
        count += perft(eng, new_board, depth - 1);
    }

    return count;
}

fn main()-> ExitCode {

    let args: Vec<String> = env::args().collect();

    let depth_limit = args[2].parse().unwrap();

    let file_path = Path::new(r"");
    let file = File::open(args[1].clone()).unwrap();
    let reader = BufReader::new(file);

    let mut tests: Vec<LineInfo> = vec![];

    let mut total_test = 0;

    for line_result in reader.lines() {
        let line = line_result.unwrap(); // Handle potential errors per line
        let data = read_line(&line);
        total_test += data.node_count.len();

        tests.push(data);
    }

    println!("--------------------------------------------------");
    println!("Starting Perft Testing Suite\n");
    println!("Total Positions: {:?}", tests.len());
    println!("Total Tests Loaded: {:?}", total_test);
    println!("--------------------------------------------------");

    //Passed, Acceptable, Failed
    let mut results = vec!(0, 0, 0);
    let engine = Engine::new();
    let mut smallest: Option<LineInfo> = None;
    let mut smallest_i = 0;

    let mut test_done = 0;

    let full_start = Instant::now();

    for test in tests {
        let test_start = Instant::now();
        println!("FEN: |{}|", test.fen);

        let board = Board::new(&test.fen, &engine);
        let mut i = 0;
        while i < test.node_count.len() && i <= depth_limit {
            print!("    D{} ", i);
            let count = perft(&engine, board, i);
            match check_outcome(count, test.node_count[i]) {
                Status::Passed => {
                    println!(" \x1b[1;32mPassed\x1b[0m");
                    results[0] += 1;
                },
                Status::Acceptable => {
                    println!(" \x1b[1;33mAcceptable\x1b[0m ({}%)", calc_over_under(count, test.node_count[i]));
                    results[1] += 1;

                    if smallest.is_none() == true {
                        smallest = Some(test.clone());
                        smallest_i = i;
                    } else if test.node_count[i] < smallest.clone().unwrap().node_count[smallest_i] {
                        smallest = Some(test.clone());
                        smallest_i = i;
                    }
                },
                Status::Failed => {
                    println!(" \x1b[1;31mFailed\x1b[0m ({}%)", calc_over_under(count, test.node_count[i]));
                    results[2] += 1;

                    if smallest.is_none() == true {
                        smallest = Some(test.clone());
                        smallest_i = i;
                    } else if test.node_count[i] < smallest.clone().unwrap().node_count[smallest_i] {
                        smallest = Some(test.clone());
                        smallest_i = i;
                    }
                }
            }

            println!("      {} of {}", count, test.node_count[i]);

            //println!();
            test_done += 1;
            i += 1;
        }

        println!("Time: {:?}(s)", test_start.elapsed().as_secs_f64());
        println!();
    }

    println!("\n\n-----------------------------------------------------");
    println!("Finished Perft Testing Suite\n");
    println!("Total Test Done: {:?}", test_done);
    println!("Total Time: {:?}(s)\n", full_start.elapsed().as_secs_f64());
    println!("\x1b[1;32mPassed\x1b[0m: {} ({}%)", results[0], calc_perc(results[0], test_done));
    println!("\x1b[1;33mAcceptable\x1b[0m: {} ({}%)", results[1], calc_perc(results[1], test_done));
    println!("\x1b[1;31mFailed\x1b[0m: {} ({}%)\n", results[2], calc_perc(results[2], test_done));
    println!("Smallest: |{}|", smallest.clone().unwrap().fen);
    println!("  Depth: {}", smallest_i);
    println!("  Expected: {}", smallest.unwrap().node_count[smallest_i]);
    println!("\n\n-----------------------------------------------------");


    if (results[2] == 0) {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }

}

//fn calc_percent

fn calc_perc(ammount: usize, total: usize) -> f32 {

    (ammount as f32 / total as f32) * 100.0
}

fn calc_over_under(res: usize, expt: usize) -> f32 {
    let expt_f = expt as f32;
    let res_f = res as f32;

    ((expt_f - res_f) / expt_f) * 100.0
}

fn check_outcome(res: usize, expt: usize) -> Status {
    if res == expt {
        return Status::Passed;
    } else if res < expt {
        return Status::Acceptable;
    } else {
        return Status::Failed;
    }
}

fn read_line(prompt: &str) -> LineInfo {
    let by_part: Vec<&str> = prompt.trim().split(';').collect();

    let mut fen = String::from(by_part[0]);
    fen = fen.trim().to_string();

    let mut node_count: Vec<usize> = vec!(1);
    let mut i = 1;

    while i < by_part.len() {
        let full = by_part[i].trim();
        let parts: Vec<&str> = full.split(' ').collect();
        if parts.len() == 2 && parts[0] == format!("D{}", node_count.len()) {
            node_count.push(parts[1].parse().unwrap());
        }
        i += 1;
    }



    LineInfo {
        fen: String::from(fen),
        node_count: node_count.clone(),
    }
}