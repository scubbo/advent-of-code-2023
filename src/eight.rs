use std::{fs::{self, OpenOptions}, collections::HashMap};
use itertools::Itertools;
use regex::Regex;
use serde::{Serialize, Deserialize};
use std::io::Write;

pub fn solve_eight() -> String {
    return process_eight("inputs/8.txt");
}

fn process_eight(input_file: &str) -> String {
    let file_as_string = fs::read_to_string(input_file).unwrap();
    let mut lines = file_as_string.lines();
    let first_line_chars = lines.next().unwrap().chars().collect_vec();
    lines.next();

    let re = Regex::new(r"^(...) = \((...), (...)\)$").unwrap();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    lines.for_each(|line| {
        let captures = re.captures(line).unwrap();
        map.insert(captures.get(1).unwrap().into(), (captures.get(2).unwrap().into(), captures.get(3).unwrap().into()));
    });
    println!("{:?}", map);

    let mut jumps = 0;
    let mut current_place = "AAA";
    while current_place != "ZZZ" {
        let direction = first_line_chars.get(jumps % first_line_chars.len()).unwrap();
        println!("Jumping {} of {}", direction, current_place);
        current_place = match direction {
            'L' => map.get(current_place).map(|pair| pair.0).unwrap(),
            'R' => map.get(current_place).map(|pair| pair.1).unwrap(),
            _ => panic!("Unparsable direction")
        };
        jumps += 1;
    }

    return jumps.to_string();
}

pub fn solve_eight_two() -> String {
    return process_eight_two("inputs/8.txt");
}

// Plan for this:
// * For each ghost, find the lollipop (https://en.wikipedia.org/wiki/Lollipop_graph) that its path follows
// * Use the parameters of the lollipop (time until entering, loop length, indices along the way of terminal nodes) to
//    generate an infinite expression for indices when the ghost is on a terminal
// * ...?
// * Solve? :D
fn process_eight_two(input_file: &str) -> String {
    let file_as_string = fs::read_to_string(input_file).unwrap();
    let mut lines = file_as_string.lines();
    let directions = lines.next().unwrap().chars().collect_vec();
    lines.next();

    let re = Regex::new(r"^(...) = \((...), (...)\)$").unwrap();
    let mut direction_map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut ghosts: Vec<&str> = Vec::new();
    lines.for_each(|line| {
        let captures = re.captures(line).unwrap();
        let start_node = captures.get(1).unwrap().into();
        direction_map.insert(start_node, (captures.get(2).unwrap().into(), captures.get(3).unwrap().into()));
        if start_node.ends_with('A') {
            ghosts.push(start_node);
        }
    });

    

    println!("There are {} ghosts", ghosts.len());
    // Use an intermediate file because tests will shut down if running for more than 60 seconds and I don't want to lose information.
    let intermediate_info_so_far = fs::read_to_string("intermediate-files/8-info.txt").unwrap().lines()
        .map(|l| {
            println!("{}", l);
            let li: LoopInfo = serde_json::from_str(l).unwrap();
            return li.starting_node;
        })
        .collect_vec();
    // https://stackoverflow.com/questions/30684624/what-is-the-best-variant-for-appending-a-new-line-in-a-text-file
    let mut out_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("intermediate-files/8-info.txt")
        .unwrap();
    ghosts.iter()
        .filter(|ghost| !intermediate_info_so_far.contains(&(*ghost).to_string()))
        .for_each(|ghost| {
            let loop_info = find_loop_info(ghost, &direction_map, &directions);
            if let Err(e) = writeln!(out_file, "{}", serde_json::to_string(&loop_info).unwrap()) {
                eprintln!("Couldn't write to file: {}", e);
            }
        });

    // Parse the intermediate file
    // Thankfully, each of the 6 starting nodes only has a single terminal node in their path, otherwise this would have
    // been even worse to calculate!
    let information = fs::read_to_string("intermediate-files/8-info.txt").unwrap();
    information.lines().map(|line| {
        let li: LoopInfo = serde_json::from_str(line).unwrap();
        return li;
        // TODO - can't find how to do this as a one-liner, compiler insists that a comma is missing from
        // `serde_json::<LoopInfo>from_str(line)`
    }).map(|li|  (li.length_of_loop, (li.jumps_after_entering_that_are_terminal[0] + li.jumps_until_entering_loop)))
    // We have a series of expressions of the form a_i * n_i + b_i, and we want to find the lowest possible value x
    // such that, for all i, there exists an n_i such that a_i * n_i + b_i = x
    //
    // I'm fairly sure we can use the Chinese Remainder Theorem to do this, but I've been at this a while and need a break :P
    .collect_vec();
    return "Hello World!".to_string();
}

#[derive(Debug, Serialize, Deserialize)]
struct LoopInfo {
    starting_node: String,
    jumps_until_entering_loop: u32,
    node_on_entering_loop: String,
    length_of_loop: u32,
    jumps_after_entering_that_are_terminal: Vec<u32>
}

fn find_loop_info(starting_node: &str, direction_map: &HashMap<&str, (&str, &str)>, directions: &Vec<char>) -> LoopInfo {
    // `seen_so_far` needs to record not only the node, but which index we're in in the instructions, because being at
    // the same node in a _different_ point in the instructions does not guarantee you'll take the same steps!
    let mut seen_so_far = vec![(starting_node, 0)];
    let mut current_node = next_node(starting_node, &direction_map, &directions, 0);
    let mut jumps = 1;
    let mut jump_counts_of_terminals = Vec::new();
    let length_of_instructions = directions.len();
    while !seen_so_far.contains(&(current_node, jumps % length_of_instructions)) {
        if current_node.ends_with('Z') {
            jump_counts_of_terminals.push(jumps);
        }
        seen_so_far.push((current_node, jumps % length_of_instructions));
        current_node = next_node(current_node, &direction_map, &directions, jumps.try_into().unwrap());
        jumps += 1;
    }
    // When this breaks, `current_node` is a value that's already been visited before - i.e. it is the node-entry point.

    let jumps_until_entering_loop = seen_so_far.iter().position(|n| (n.0) == current_node).unwrap();
    let length_of_loop = seen_so_far.len() - jumps_until_entering_loop - 1;
    return LoopInfo{
        starting_node: starting_node.to_string(),
        jumps_until_entering_loop: jumps_until_entering_loop as u32, 
        node_on_entering_loop: current_node.to_string(),
        jumps_after_entering_that_are_terminal: jump_counts_of_terminals.iter().map(|jc| (jc-jumps_until_entering_loop) as u32).collect_vec(),
        length_of_loop: length_of_loop as u32
    }

}

fn next_node<'a>(current_node: &'a str, direction_map: &HashMap<&'a str, (&'a str, &'a str)>, directions: &Vec<char>, jump_number: u32) -> &'a str {
    let direction = directions[(jump_number as usize) % directions.len()];
    match direction {
        'L' => direction_map.get(current_node).map(|pair| pair.0).unwrap(),
        'R' => direction_map.get(current_node).map(|pair| pair.1).unwrap(),
        _ => panic!("Unparsable direction")
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eight() {
        // assert_eq!(process_eight("inputs/8-test-1.txt"), "2");
        // assert_eq!(process_eight("inputs/8-test-2.txt"), "6");
        // assert_eq!(process_eight_two("inputs/8-test-3.txt"), "6");
        assert_eq!(process_eight_two("inputs/8.txt"), "something-to-make-println-happen");
    }
}
