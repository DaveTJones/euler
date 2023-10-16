use itertools::Itertools;

const UPPER_BOUND: i32 = 1000;

fn main() {
    // list all multiples of 3
    let threes = (0..UPPER_BOUND).step_by(3);
    // list all multiples of 5
    let fives = (0..UPPER_BOUND).step_by(5);
    // combine, deduplicate and sum
    let threes_and_fives: i32 = threes.chain(fives).into_iter().unique().sum();
    println!("Sum is {threes_and_fives}");
}
