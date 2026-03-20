use std::collections::HashMap;
use chicory::chicory::board::Board;
use chicory::chicory::engine::Engine;
use chicory::chicory::perft::*;

const THREAD_COUNT: usize = 8;

fn perft_from_string(eng: &Engine, board: Board, list:  String, depth: usize) -> bool {

    let lines: Vec<&str> = list.split("\n").collect();

    let mut expected: HashMap<String, usize> = HashMap::new();
    let mut results: HashMap<String, usize> = HashMap::new();

    for line in lines {
        let parts: Vec<&str> = line.split(": ").collect();
        //println!("|{} {}|", parts[0], parts[1]);
        expected.insert(parts[0].to_string(), parts[1].parse::<usize>().unwrap());
    }

    //let mut count = 0;

    let moves = eng.gen_moves(board);
    //println!("moves count: {:?}", moves);
    for m in moves {
        let (from, to, new_board, _) = m;
        //new_board.print_board();
        //count += perft(&eng, new_board, 1);
        results.insert(Board::move_to_lan(&m) , multi_perft(&eng, new_board, depth-1, THREAD_COUNT));
    }

    let mut pass = true;

    for key in results.keys() {
        let result = results.get(key).unwrap();

        match expected.get(key) {
            Some(x) => {
                if result == x {
                    println!("{} | ==", key);
                } else {
                    pass = false;
                    println!("{} | E: {}, R: {}", key, x, result);
                }
                expected.remove(key);
            },
            None => {
                pass = false;
                println!("{} | Additional", key);
            }
        }

    }

    for key in expected.keys() {
        pass = false;
        print!("{} | Missing ", key);
    }

    return pass;
}

#[test]
fn ip_one() {
    let engine = Engine::new();
    let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &engine);
    let count = multi_perft(&engine, board, 1, THREAD_COUNT);

    assert_eq!(count, 20);
}

#[test]
fn ip_two() {
    let engine = Engine::new();
    let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &engine);
    let count = multi_perft(&engine, board, 2, THREAD_COUNT);

    assert_eq!(count, 400);
}

#[test]
fn ip_three() {
    let engine = Engine::new();
    let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &engine);
    let count = multi_perft(&engine, board, 3, THREAD_COUNT);

    assert_eq!(count, 8902);
}

#[test]
fn ip_four() {
    let engine = Engine::new();
    let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &engine);
    let count = multi_perft(&engine, board, 4, THREAD_COUNT);

    assert_eq!(count, 197281);
}

#[test]
fn ip_five() {
    let engine = Engine::new();
    let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &engine);
    let count = multi_perft(&engine, board, 5, THREAD_COUNT);

    assert_eq!(count, 4865609);
}

#[test]
fn p2_one() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 1 1", &engine);
    let count = multi_perft(&engine, board, 1, THREAD_COUNT);

    assert_eq!(count, 48);
}

#[test]
fn p2_two() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 1 1", &engine);
    let count = multi_perft(&engine, board, 2, THREAD_COUNT);

    assert_eq!(count, 2039);
}

#[test]
fn p2_three() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 1 1", &engine);
    let count = multi_perft(&engine, board, 3, THREAD_COUNT);

    assert_eq!(count, 97862);
}

#[test]
fn p3_one() {
    let engine = Engine::new();
    let board = Board::new("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", &engine);
    let count = multi_perft(&engine, board, 1, THREAD_COUNT);

    assert_eq!(count, 14);
}

#[test]
fn p3_two() {
    let engine = Engine::new();
    let board = Board::new("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", &engine);
    let count = multi_perft(&engine, board, 2, THREAD_COUNT);

    assert_eq!(count, 191);
}

#[test]
fn p3_three() { //E.P?
    let engine = Engine::new();
    let board = Board::new("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", &engine);
    let count = multi_perft(&engine, board, 3, THREAD_COUNT);

    assert_eq!(count, 2812);
}

#[test]
fn p4_one() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", &engine);
    let count = multi_perft(&engine, board, 1, THREAD_COUNT);

    assert_eq!(count, 6);
}

#[test]
fn p4_two() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", &engine);
    let count = multi_perft(&engine, board, 2, THREAD_COUNT);

    assert_eq!(count, 264);
}

#[test]
fn p4_three() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", &engine);
    let count = multi_perft(&engine, board, 3, THREAD_COUNT);

    assert_eq!(count, 9467);
}

#[test]
fn p5_one() {
    let engine = Engine::new();
    let board = Board::new("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", &engine);
    let count = multi_perft(&engine, board, 1, THREAD_COUNT);

    assert_eq!(count, 44);
}

#[test]
fn p5_two() {
    let engine = Engine::new();
    let board = Board::new("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", &engine);
    let count = multi_perft(&engine, board, 2, THREAD_COUNT);

    assert_eq!(count, 1486);
}

#[test]
fn p5_three() {
    let engine = Engine::new();
    let board = Board::new("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", &engine);
    let count = multi_perft(&engine, board, 3, THREAD_COUNT);

    assert_eq!(count, 62379);
}

#[test]
fn p6_one() {
    let engine = Engine::new();
    let board = Board::new("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", &engine);
    let count = multi_perft(&engine, board, 1, THREAD_COUNT);

    assert_eq!(count, 46);
}

#[test]
fn p6_two() {
    let engine = Engine::new();
    let board = Board::new("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", &engine);
    let count = multi_perft(&engine, board, 2, THREAD_COUNT);

    assert_eq!(count, 2079);
}

#[test]
fn p6_three() {
    let engine = Engine::new();
    let board = Board::new("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", &engine);
    let count = multi_perft(&engine, board, 3, THREAD_COUNT);

    assert_eq!(count, 89890);
}

#[test]
fn p6_four() {
    let engine = Engine::new();
    let board = Board::new("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", &engine);
    let count = multi_perft(&engine, board, 4, THREAD_COUNT);

    assert_eq!(count, 3894594);
}

#[test]
fn custom() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/8/8/8/8/8/8/2R1K2R w Kkq - 0 1", &engine);
    let count = perft(&engine, board, 4);

    assert_eq!(count, 312835);
}

#[test]
fn full_hash() {

    //TODO FIX BLOCKED CASLING ATTACKS

    let input = String::from("c1a1: 13231
c1b1: 14252
c1d1: 11317
c1c2: 15278
c1c3: 15119
c1c4: 15061
c1c5: 15011
c1c6: 14836
c1c7: 11599
c1c8: 2318
h1f1: 11144
h1g1: 13435
h1h2: 15298
h1h3: 14515
h1h4: 13650
h1h5: 12791
h1h6: 11793
h1h7: 8164
h1h8: 1377
e1d1: 11797
e1f1: 12007
e1d2: 15926
e1e2: 17573
e1f2: 15991
e1g1: 9352");

    let eng = Engine::new();
    let board = Board::new("r3k2r/8/8/8/8/8/8/2R1K2R w Kkq - 0 1", &eng);

    assert_eq!(perft_from_string(&eng, board, input, 4), true);
}

#[test]
fn full_hash_2() {

    let input = String::from("a8a1: 656
a8a2: 169
a8a3: 672
a8a4: 759
a8a5: 759
a8a6: 759
a8a7: 759
a8b8: 655
a8c8: 583
a8d8: 120
h8h1: 466
h8h2: 161
h8h3: 576
h8h4: 691
h8h5: 717
h8h6: 742
h8h7: 763
h8f8: 646
h8g8: 674
e8d7: 889
e8e7: 893
e8f7: 887
e8d8: 632
e8f8: 628
e8g8: 670");

    let eng = Engine::new();
    let board = Board::new("r3k2r/8/8/8/8/8/3K4/2R4R b kq - 0 1", &eng);
    //println!("{:08b}", board.casling);
    assert_eq!(perft_from_string(&eng, board, input, 3), true);
    //assert_eq!(false, true);
}

#[test]
fn full_hash_2_c() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/8/8/8/8/8/3K4/2R4R b kq - 0 1", &engine);
    let count = perft(&engine, board, 3);

    assert_eq!(count, 15926);
}

#[test]
fn full_hash_1() {

    let input = String::from("h1h2: 15
d2d1: 29
d2e1: 29
d2e3: 29
d2d3: 29
d2c3: 30");

    let eng = Engine::new();
    let board = Board::new("r3k3/8/8/8/8/8/3K3r/2R4R w q - 0 1", &eng);

    assert_eq!(perft_from_string(&eng, board, input, 2), true);
}

#[test]
fn full_hash_1_c() {
    let engine = Engine::new();
    let board = Board::new("r3k3/8/8/8/8/8/3K3r/2R4R w q - 0 1", &engine);
    let count = perft(&engine, board, 2);

    assert_eq!(count, 161);
}

#[test]
fn custom_c() {
    let engine = Engine::new();
    let board_init =  Board::new("r3k3/8/8/8/8/8/8/4K3 b q - 0 1", &engine);

    //let moves = engine.gen_moves(board_init);

   // let moves_2 = engine.gen_moves( moves[16].2);

    //let board = moves_2[37].2;

    //let mut i = 0;

    //while i < moves_2.len() {
    //    println!("{}: {}", i, Board::move_to_lan(&moves_2[i]));
    //    i += 1;
    //}

    //println!("{:?}", moves_2[37]);

    //println!("{}", Board::move_to_lan(&moves[37]));

    let input = String::from("a8a1: 1
a8a2: 1
a8a3: 1
a8a4: 1
a8a5: 1
a8a6: 1
a8a7: 1
a8b8: 1
a8c8: 1
a8d8: 1
e8d7: 1
e8e7: 1
e8f7: 1
e8d8: 1
e8f8: 1
e8c8: 1");

    let eng = Engine::new();

    assert_eq!(perft_from_string(&eng, board_init, input, 1), true);
    //assert_eq!(false, true);
}

//Rook Capture, respawn when calsing

/*
#[test]
fn p6_five() {
    let engine = Engine::new();
    let board = Board::new("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", &engine);
    let count = perft(board, 5);

    assert_eq!(count, 164075551);
}
*/
//https://www.chessprogramming.org/Perft_Results
//http://www.rocechess.ch/perft.html