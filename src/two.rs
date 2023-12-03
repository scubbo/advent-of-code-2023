use std::{fs, str::Split, cmp::max};

use itertools::Itertools;

pub fn solve_two() -> String {
    let input = fs::read_to_string("inputs/2.txt")
        .expect("Should have been able to read file");
    return input.split("\n").into_iter()
        .filter(|line| !line.is_empty())
        .inspect(|line| println!("DEBUG - operating on {}", line))
        .map(|line| line.split(": "))
        // `clone` because "Cannot borrow split as mutable, as it is not declared as mutable" - but I'm not _trying_ to mutate it!?
        // (oh, but calling `nth` _does_ mutate it)
        .map(|split| (split.clone().nth(0).unwrap(), split.clone().nth(1).unwrap()))
        .map(|(game_id_as_long_string, game_info)| (game_id_as_long_string[5..].parse::<i32>().unwrap(), game_info.split("; ")))
        // This _works_, but I don't see why I have to `clone` in order to iterate over a `Split`? 
        .filter(|(_, split_game_info)| is_game_legal(split_game_info.clone()))
        .map(|(game_id, _)| game_id)
        .sum::<i32>().to_string();
}

pub fn solve_two_two() -> String {
    let input = fs::read_to_string("inputs/2.txt")
        .expect("Should have been able to read file");
    return input.split("\n").into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.split(": ").collect_vec())
        .map(|split_vec| split_vec[1])
        .map(|game_info| find_min_cubes_for_game(game_info))
        .map(|min_cubes| min_cubes.0 * min_cubes.1 * min_cubes.2)
        .sum::<i32>().to_string()
}

// A `game_info` looks like `["3 green, 1 blue, 3 red", "3 blue, 1 green, 3 red", "; "2 red, 12 green, 7 blue", "1 red, 4 blue, 5 green", "7 green, 2 blue, 2 red"]``
fn is_game_legal(game_info: Split<&str>) -> bool {
    // Initially had `game_info.all(is_game_instance_legal)`, but that gave an error
    // `cannot mutate immutable variable game_info` - which is unexpected, because I'm _not_ mutating it, I'm just
    // calling a method on each of its elements. Maybe I need to mark `is_game_instance_legal` as non-mutating somehow?
    //
    // return game_info.all(is_game_instance_legal);

    // And then tried `for elem in game_info`, which said `&std::str::Split<'_, &str>` is not an iterator`
    for game_instance in game_info {
        if !is_game_instance_legal(game_instance) {
            return false
        }
    }
    return true
}

// A `game_instance` looks like `"3 green, 1 blue, 3 red"`
fn is_game_instance_legal(game_instance_info: &str) -> bool {
    return game_instance_info.split(", ").into_iter()
        .inspect(|colour_instance| println!("DEBUG - colour_instance is {colour_instance}"))
        .map(|colour_instance| colour_instance.split(" "))
        .map(|mut split| {
            // Yes, repetition of `.nth(0)` is correct - `split` is an iterator, so calling `nth` progresses the iteration.
            let (num, col) = (split.nth(0).unwrap().parse::<i32>().unwrap(), split.nth(0).unwrap());
            match col {
                "red" => num <= 12,
                "green" => num <= 13,
                "blue" => num <= 14,
                _ => panic!("Encountered unexpected colour")
            }
        })
        .reduce(|accum, elem| accum && elem).unwrap();
}

fn find_min_cubes_for_game(game_info: &str) -> (i32, i32, i32) {
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;
    let game_instances = game_info.split("; ");
    for game_instance in game_instances {
        let colour_instances = game_instance.split(", ");
        for colour_instance in colour_instances {
            let split_colour_instance = colour_instance.split(" ").collect_vec();
            let (num, col) = (split_colour_instance[0].parse::<i32>().unwrap(), split_colour_instance[1]);
            match col {
                "red" => min_red = max(min_red, num),
                "green" => min_green = max(min_green, num),
                "blue" => min_blue = max(min_blue, num),
                _ => panic!("Unexpected colour")
            }
        }
    }
    return (min_red, min_green, min_blue);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve_two(), "1853")
    }
}