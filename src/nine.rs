use std::fs;

use itertools::Itertools;

pub fn solve_nine() -> String {
    return process_nine("inputs/9.txt");
}

fn process_nine(input_file: &str) -> String {
    let binding = fs::read_to_string(input_file).unwrap();
    let lines = binding.lines().filter(|line| !line.is_empty());
    return lines
        //TODO - does the split line actually need to be a vector? Maybe just an iterator?
        .map(|line| line.split_whitespace().map(|num| num.parse::<i32>().unwrap()).collect_vec())
        .map(|line| find_last_number_of_sequence(line)).sum::<i32>().to_string();
}

fn find_last_number_of_sequence(seq: Vec<i32>) -> i32 {
    let mut diffs = Vec::new();
    let mut are_all_diffs_zero = true;
    for idx in 0..seq.len()-1 {
        let diff = seq[idx+1] - seq[idx];
        if diff != 0 {
            are_all_diffs_zero = false;
        }
        diffs.push(diff);
    }
    return seq.last().unwrap() + if are_all_diffs_zero {0} else {find_last_number_of_sequence(diffs)};
}

pub fn solve_nine_two() -> String {
    return process_nine_two("inputs/9.txt");
}

fn process_nine_two(input_file: &str) -> String {
    let binding = fs::read_to_string(input_file).unwrap();
    let lines = binding.lines().filter(|line| !line.is_empty());
    return lines
        //TODO - does the split line actually need to be a vector? Maybe just an iterator?
        .map(|line| line.split_whitespace().map(|num| num.parse::<i32>().unwrap()).collect_vec())
        .map(|line| find_first_number_of_sequence(line)).sum::<i32>().to_string();
}

fn find_first_number_of_sequence(seq: Vec<i32>) -> i32 {
    let mut diffs = Vec::new();
    let mut are_all_diffs_zero = true;
    for idx in 0..seq.len()-1 {
        let diff = seq[idx+1] - seq[idx];
        if diff != 0 {
            are_all_diffs_zero = false;
        }
        diffs.push(diff);
    }
    return seq.first().unwrap() - if are_all_diffs_zero {0} else {find_first_number_of_sequence(diffs)};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(process_nine("inputs/9-test.txt"), "114");
        assert_eq!(process_nine_two("inputs/9-test.txt"), "2");
    }
}