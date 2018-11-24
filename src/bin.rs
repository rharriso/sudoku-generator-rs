extern crate time;

mod sudoku_generator;

fn main() {
    let start = time::precise_time_ns();
    let args: Vec<_> = std::env::args().collect();

    //let board_size: usize = args[2].parse().unwrap();
    let board_size = 3;

    let board_count: usize = if args.len() > 1 { args[1].parse().unwrap() } else { 1000 };
    let all_neighbors  = if args.len() > 2 {args[2] == "--all-neighbors"} else { false };

    sudoku_generator::generate_and_fill_boards(board_count, board_size, all_neighbors);

    let duration = time::precise_time_ns() - start;
    let seconds = duration as f64 / 1000000000f64;
    println!("time: {} s", seconds);
    // println!("result size count: {}", result.len());
    println!("board count {}", board_count);
    println!("boards per second {}", board_count as f64 / seconds);
}
