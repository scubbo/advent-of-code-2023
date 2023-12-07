use crate::one::{solve_one, solve_one_two};
use crate::two::{solve_two, solve_two_two};
use crate::three::{solve_three, solve_three_two};
use crate::four::{solve_four, solve_four_two};
use crate::five::{solve_five, solve_five_two};
use crate::six::solve_six;
use crate::seven::solve_seven;

pub mod one;
pub mod two;
pub mod three;
pub mod four;
pub mod five;
pub mod six;
pub mod seven;


fn main() {
    let problem_to_solve = std::env::args().nth(1).expect("No problem number given");
    // TODO - would love to know how to make an array of functions that can be indexed based on this parameter, but
    // :shrug: not during a race!
    let output = match problem_to_solve.as_str() {
        "1" => solve_one(),
        "1-2" => solve_one_two(),
        "2" => solve_two(),
        "2-2" => solve_two_two(),
        "3" => solve_three(),
        "3-2" => solve_three_two(),
        "4" => solve_four(),
        "4-2" => solve_four_two(),
        "5" => solve_five(),
        "5-2" => solve_five_two(),
        "6" => solve_six(),
        "7" => solve_seven(),
        x => format!("Illegal problem number {x}")
    };
    println!("{}", output)
}
