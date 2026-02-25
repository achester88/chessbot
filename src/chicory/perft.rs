use std::sync::mpsc;
use std::thread;
use crate::chicory::board::Board;
use crate::chicory::engine::Engine;

pub fn perft(eng: &Engine, board: Board, depth: usize) -> usize {
    //println!("--------------------[ Perft depth: {} ]---------------------", depth);
    let mut count = 0;

    if depth == 0 {
        return 1;
    }

    let moves = eng.gen_moves(board);
    //println!("moves count: {:?}", moves);
    for m in moves {
        let (_, _, new_board, _) = m;
        count += perft(eng, new_board, depth - 1);
    }

    count
}

pub fn multi_perft(eng: &Engine, board: Board, depth: usize, thread_count: usize) -> usize {
    if depth == 0 {
        return 1;
    }

    let moves = eng.gen_moves(board);

    if depth == 1 {
        return moves.len();
    }

    let mut chunks = vec![vec!(); thread_count];

    let group = moves.len() / thread_count;
    let group_r = moves.len() % thread_count;

    for i in 0..chunks.len() {
        chunks[i] = moves[(group*i)..(group*(i+1))].to_vec();
    }

    if group_r != 0 {
        chunks[thread_count-1] = moves[(group*(chunks.len()-1))..((group*(chunks.len()))+group_r)].to_vec();
    }

    //let mut handles = vec![];
    let (tx, rx) = mpsc::channel();

    thread::scope(|s| {
        for n in 0..thread_count {
            let index = n.clone();
            let set = chunks[n].clone();
            let ctx = tx.clone();
            s.spawn(move || {
                //let ind = index.clone();
                let mut count = 0;
                for (_, _, b, _) in set {
                    count += perft(eng, b, depth - 1);
                }
                //println!("{:?}\n\n", set);
                ctx.send(count).unwrap();
            });
            //handles.push(handle);
        }
    });


    //for handle in handles {
    //    handle.join().unwrap();
    //}

    let mut sum = 0;
    let mut rec_count = 0;
    for received in rx {
        //println!("Got: {received}");
        sum += received;
        rec_count += 1;

        if rec_count >= thread_count {
            break;
        }
    }

    return sum;
}
