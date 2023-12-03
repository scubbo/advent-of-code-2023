use crate::one::{solve_one, solve_one_two};
use crate::two::{solve_two, solve_two_two};

pub mod one;
pub mod two;


fn main() {
    let problem_to_solve = std::env::args().nth(1).expect("No problem number given");
    // TODO - would love to know how to make an array of functions that can be indexed based on this parameter, but
    // :shrug: not during a race!
    let output = match problem_to_solve.as_str() {
        "1" => solve_one(),
        "1-2" => solve_one_two(),
        "2" => solve_two(),
        "2-2" => solve_two_two(),
        x => format!("Illegal problem number {x}")
    };
    println!("{}", output)
}
