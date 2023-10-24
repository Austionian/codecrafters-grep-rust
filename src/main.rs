use grep_starter_rust::matches::MatchQueue;
use std::env;
use std::io;
use std::process;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();

    let match_queue = MatchQueue::from(&pattern);
    let res = match_queue.check(&input_line);

    if res {
        println!("pass");
        process::exit(0);
    } else {
        println!("fail");
        process::exit(1);
    }
}
