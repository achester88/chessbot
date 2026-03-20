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
    let board = Board::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1", &engine);
    let count = multi_perft(&engine, board, 4, THREAD_COUNT);

    assert_eq!(count, 4085603);
}

#[test]
fn full_hash() {

    //TODO FIX BLOCKED CASLING ATTACKS

    let input = String::from("a2a3: 94405
b2b3: 81066
g2g3: 77468
d5d6: 79551
a2a4: 90978
g2g4: 75677
g2h3: 82759
d5e6: 97464
c3b1: 84773
c3d1: 84782
c3a4: 91447
c3b5: 81498
e5d3: 77431
e5c4: 77752
e5g4: 79912
e5c6: 83885
e5g6: 83866
e5d7: 93913
e5f7: 88799
d2c1: 83037
d2e3: 90274
d2f4: 84869
d2g5: 87951
d2h6: 82323
e2d1: 74963
e2f1: 88728
e2d3: 85119
e2c4: 84835
e2b5: 79739
e2a6: 69334
a1b1: 83348
a1c1: 83263
a1d1: 79695
h1f1: 81563
h1g1: 84876
f3d3: 83727
f3e3: 92505
f3g3: 94461
f3h3: 98524
f3f4: 90488
f3g4: 92037
f3f5: 104992
f3h5: 95034
f3f6: 77838
e1d1: 79989
e1f1: 77887
e1g1: 86975
e1c1: 79803");

    let eng = Engine::new();
    let board = Board::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1", &eng);

    assert_eq!(perft_from_string(&eng, board, input, 4), true);
}

#[test]
fn full_hash_2() {

    let input = String::from("b4b3: 2019
g6g5: 1937
c7c6: 1848
c7c5: 1844
h3g2: 2301
b4c3: 1932
c7d6: 1803
b6a4: 1931
b6c4: 1946
b6d5: 1965
b6c8: 1690
f6e4: 2508
f6g4: 2220
f6d5: 2220
f6h5: 2081
f6h7: 1985
f6g8: 1988
a6e2: 1821
a6d3: 1983
a6c4: 1993
a6b5: 2035
a6b7: 2040
a6c8: 1705
g7h6: 2012
g7f8: 1789
a8b8: 2032
a8c8: 1887
a8d8: 1891
h8h4: 2020
h8h5: 2002
h8h6: 1834
h8h7: 1835
h8f8: 1644
h8g8: 1738
e7d6: 2197
e7d8: 1805
e7f8: 1751
e8d8: 1789
e8f8: 1783
e8g8: 1840
e8c8: 1907");

    let eng = Engine::new();
    let board = Board::new("r3k2r/p1ppqpb1/bn1Ppnp1/4N3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQkq - 0 1", &eng);
    //println!("{:08b}", board.casling);
    assert_eq!(perft_from_string(&eng, board, input, 3), true);
    //assert_eq!(false, true);
}

#[test]
fn full_hash_2_c() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/p1ppqpb1/bn1Ppnp1/4N3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQkq - 0 1", &engine);
    let count = perft(&engine, board, 3);

    assert_eq!(count, 79551);
}

#[test]
fn full_hash_1() {

    let input = String::from("a2a3: 35
b2b3: 33
g2g3: 33
a2a4: 35
g2g4: 33
g2h3: 33
d6e7: 32
d6c7: 32
c3b1: 33
c3d1: 33
c3a4: 33
c3b5: 30
c3d5: 34
e5d3: 34
e5c4: 33
e5g4: 35
e5c6: 32
e5g6: 35
e5d7: 39
e5f7: 36
d2c1: 34
d2e3: 34
d2f4: 34
d2g5: 33
d2h6: 34
e2d1: 35
e2f1: 35
e2d3: 33
e2c4: 32
e2b5: 31
e2a6: 27
a1b1: 34
a1c1: 34
a1d1: 34
h1f1: 34
h1g1: 34
f3d3: 33
f3e3: 34
f3g3: 34
f3h3: 33
f3f4: 34
f3g4: 34
f3f5: 36
f3h5: 35
f3f6: 30
e1d1: 34
e1f1: 34
e1g1: 34
e1c1: 34");

    let eng = Engine::new();
    let board = Board::new("r3kr2/p1ppqpb1/bn1Ppnp1/4N3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQq - 0 1", &eng);

    assert_eq!(perft_from_string(&eng, board, input, 2), true);
}

#[test]
fn full_hash_1_c() {
    let engine = Engine::new();
    let board = Board::new("r3kr2/p1ppPpb1/bn2pnp1/4N3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQq - 0 1", &engine);
    let count = perft(&engine, board, 1);

    assert_eq!(count, 32);
}

#[test]
fn custom_c() {
    let engine = Engine::new();
    let board_init =  Board::new("r3kr2/p1ppPpb1/bn2pnp1/4N3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQq - 0 1", &engine);

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

    let input = String::from("b4b3: 1
g6g5: 1
c7c6: 1
d7d6: 1
c7c5: 1
d7d5: 1
h3g2: 1
b4c3: 1
b6a4: 1
b6c4: 1
b6d5: 1
b6c8: 1
f6e4: 1
f6g4: 1
f6d5: 1
f6h5: 1
f6h7: 1
f6g8: 1
a6e2: 1
a6d3: 1
a6c4: 1
a6b5: 1
a6b7: 1
a6c8: 1
g7h6: 1
g7h8: 1
a8b8: 1
a8c8: 1
a8d8: 1
f8g8: 1
f8h8: 1
e8e7: 1");

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