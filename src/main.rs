mod memory;
use memory::*;

extern crate rand;

fn main() {
    println!("Hello, world!");
    let (fifo_result, fifo_outcome) = fifo(3, String::from("1 3 0 3 5 6 3"));
    println!("{}", fifo_result);
    println!("{}", fifo_outcome);

    let (fifo_result, fifo_outcome) = fifo(4, String::from("3 5 6 3 2 6 5 7 8 6 9 1 2 4 5 4 6 5 7 9 8 6 7 6 5 6 5 6 5 3 2"));
    println!("{}", fifo_result);
    println!("{}", fifo_outcome);

    let (lru_result, lru_outcome) = lru(4, String::from("3 5 6 3 2 6 5 7 8 6 9 1 2 4 5 4 6 5 7 9 8 6 7 6 5 6 5 6 5 3 2"));
    println!("{}", lru_result);
    println!("{}", lru_outcome);

    let (opt_result, opt_outcome) = opt(4, String::from("3 5 6 3 2 6 5 7 8 6 9 1 2 4 5 4 6 5 7 9 8 6 7 6 5 6 5 6 5 3 2"));
    println!("{}", opt_result);
    println!("{}", opt_outcome);

    let (rand_result, rand_outcome) = rand(4, String::from("3 5 6 3 2 6 5 7 8 6 9 1 2 4 5 4 6 5 7 9 8 6 7 6 5 6 5 6 5 3 2"));
    println!("{}", rand_result);
    println!("{}", rand_outcome);
}
