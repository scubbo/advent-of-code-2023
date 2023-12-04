use std::fs;

use itertools::Itertools;

pub fn solve_three() -> String {
    return process_three("inputs/3.txt");
}

fn process_three(input_file: &str) -> String {
    let input = fs::read_to_string(input_file)
        .expect("Should have been able to read file");
    
    // TIL that `lines()` exists, didn't need to `.split("\n")` :)

    // (We're going to have to "process" one line multiple times, both as a source of part-numbers and of symbols, so an
    // iterator would make things tricky as it would have to go backwards)
    let mut lines_as_vec = input.lines().collect_vec();
    let length_of_line = lines_as_vec[0].len();
    let number_of_lines = lines_as_vec.len();
    println!("DEBUG - number_of_lines is {number_of_lines}");

    // We can simplify the logic by adding symbol-free lines before and after the input - that way we don't need special
    // logic for the first and last lines, we can treat all lines as internal (i.e. having neighbours above and below).
    let mut bracketed_lines: Vec<&str> = Vec::new();
    let fake_line = std::iter::repeat('.').take(length_of_line).collect::<String>();
    let mut bracketing_line_vec = vec!(fake_line.as_str());
    bracketed_lines.append(&mut bracketing_line_vec.clone()); // Need to clone because `append` will empty the source
    bracketed_lines.append(&mut lines_as_vec);
    bracketed_lines.append(&mut bracketing_line_vec);
    println!("DEBUG - bracketed_lines has length {}", bracketed_lines.len());

    return (0..number_of_lines).map(|line_num| {
        println!("DEBUG - operating on line_num {line_num}");
        let line_chunk = &bracketed_lines[line_num..line_num+3];
        return find_sum_of_part_numbers_in_line_chunk(line_chunk, length_of_line);
    }).sum::<i32>().to_string();
}

pub fn solve_three_two() -> String {
    return process_three_two("inputs/3.txt");
}

fn process_three_two(input_file: &str) -> String {
    println!("Running three-two");
    let input = fs::read_to_string(input_file)
        .expect("Should have been able to read file");
    let lines = input.lines().into_iter()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let num_lines = lines.len();
    let line_length = lines[0].len();

    let mut sum_of_gear_ratios = 0;
    for line_num in 0..num_lines {
        for char_idx in 0..line_length {
            if lines[line_num][char_idx] == '*' {
                println!("DEBUG - found a gear at {line_num}, {char_idx}");
                sum_of_gear_ratios += gear_ratio_at(&lines, line_num, char_idx);
            }
        }
    }
    return sum_of_gear_ratios.to_string();
}

fn find_sum_of_part_numbers_in_line_chunk(line_chunk: &[&str], length_of_line: usize) -> i32 {
    let mut idx: usize = 0;
    let mut sum = 0;
    let line_chunk_chars = line_chunk.into_iter().map(|line| line.chars().collect_vec()).collect_vec();
    while idx < length_of_line {
        println!("DEBUG - checking index {idx}");
        if line_chunk_chars[1][idx].is_numeric() {
            let processed = process_potential_part_num(&line_chunk_chars, idx);
            sum += processed.0.unwrap_or(0);
            idx += processed.1;
        } else {
            // Number is not numeric - continue to the next character.
            idx += 1;
        }

    }
    return sum;
}

// Returns:
// * the value of the number if it is a part-num (i.e. is adjacent to a symbol), or `None` if not a part-num.
// * the length of the processed number in characters
fn process_potential_part_num(line_chunk: &Vec<Vec<char>>, starting_index: usize) -> (Option<i32>, usize) {
    let mut is_part_num = false;
    
    // I dislike this special-case boundary behaviour, but it's harder to prepend to a `str` than to a `Vec` (as we did
    // when adding lines), so :shrug:
    if starting_index != 0 {
        // Use `is_part_num = is_part_num || ...` to short-circuit evaluation - no need to check indices if already
        // identified as a part_num 
        is_part_num = is_index_touching_symbol_above_or_below(&line_chunk, starting_index-1) ||
            is_symbol(line_chunk[1][starting_index-1]);
    }

    let mut offset = 0;
    let mut num_chars_as_vec: Vec<char> = Vec::new();
    let line_length = line_chunk[1].len();
    while starting_index + offset < line_length && line_chunk[1][starting_index + offset].is_numeric() {
        num_chars_as_vec.push(line_chunk[1][starting_index + offset]);
        is_part_num = is_part_num || is_index_touching_symbol_above_or_below(&line_chunk, starting_index + offset);
        offset += 1;
    }

    // Another awkward boundary-behaviour :'(
    if starting_index + offset < line_chunk[1].len() {
        is_part_num = is_part_num || 
            is_index_touching_symbol_above_or_below(&line_chunk, starting_index + offset) ||
            is_symbol(line_chunk[1][starting_index + offset])
    }

    return (if is_part_num {Some(num_chars_as_vec_to_i32(num_chars_as_vec))} else {None}, offset);
}

fn is_index_touching_symbol_above_or_below(line_chunk: &Vec<Vec<char>>, index: usize) -> bool {
    return is_symbol(line_chunk[0][index]) || is_symbol(line_chunk[2][index]);
}

fn is_symbol(c: char) -> bool {
    return !(c.is_numeric() || c == '.')
}

fn num_chars_as_vec_to_i32(num_chars_as_vec: Vec<char>) -> i32 {
    let length = num_chars_as_vec.len();
    return num_chars_as_vec.into_iter().enumerate()
        .map(|(i, char)| char.to_string().parse::<i32>().unwrap() * 10_i32.pow((length - (i + 1)).try_into().unwrap()))
        .sum();
}

// Returns 0 if this is not a gear
fn gear_ratio_at(lines: &Vec<Vec<char>>, line_num: usize, char_idx: usize) -> i32 {
    let mut neighbouring_numbers = Vec::new();
    // First, process top row, being careful of overlap.
    if line_num > 0 {
        neighbouring_numbers.append(&mut find_flood_fill_numbers_in_neighbouring_line(&lines[line_num - 1], char_idx));
    }

    // Then, find numbers to either side horizontally - this is easier, since they cannot overlap!
    if char_idx > 0 && lines[line_num][char_idx - 1].is_numeric() {
        neighbouring_numbers.push(find_number_by_flood_fill(&lines[line_num], char_idx - 1))
    }
    if char_idx < lines[line_num].len() && lines[line_num][char_idx + 1].is_numeric() {
        neighbouring_numbers.push(find_number_by_flood_fill(&lines[line_num], char_idx + 1))
    }
    // Then find numbers on row below - again careful of overlap
    if line_num < lines.len() - 1 {
        neighbouring_numbers.append(&mut find_flood_fill_numbers_in_neighbouring_line(&lines[line_num + 1], char_idx));
    }
    println!("Neighbouring numbers for gear at {}, {} are [{:?}]", line_num, char_idx, neighbouring_numbers);

    // Finally, check for valid gear ratio - i.e. adjacent to exactly two numbers.
    if neighbouring_numbers.len() == 2 {
        let response = neighbouring_numbers[0] * neighbouring_numbers[1];
        println!("Gear ratio at {}, {} is {}", line_num, char_idx, response);
        return response;
    } else {
        // Since we will be adding gear-ratios, this is the identity value.
        return 0
    }
}

fn find_flood_fill_numbers_in_neighbouring_line(neighbouring_line: &Vec<char>, char_idx: usize) -> Vec<i32> {
    let line_length = neighbouring_line.len();

    let mut neighbouring_numbers: Vec::<i32> = Vec::new();
    if char_idx > 0 && neighbouring_line[char_idx - 1].is_numeric() {
        println!("DEBUG - there's a number to the diagonal-left from {} in {:?}", char_idx, neighbouring_line);
        neighbouring_numbers.push(find_number_by_flood_fill(&neighbouring_line, char_idx - 1));
    }

    if char_idx < line_length - 1 && neighbouring_line[char_idx+1].is_numeric() && (
        char_idx == 0 ||
        (
            char_idx > 0 &&
            (
                neighbouring_line[char_idx - 1].is_numeric() &&
                !neighbouring_line[char_idx].is_numeric()
            ) ||
            (
                !neighbouring_line[char_idx - 1].is_numeric()
            )
        )
    ) {
        // I.e. if there is a number to the diagonal-right, _and_ that number doesn't "overflow" back to the diagonal-left
        println!("DEBUG - there's a (non-overlapping) number to the diagonal-right from {} in {:?}", char_idx, neighbouring_line);
        neighbouring_numbers.push(find_number_by_flood_fill(&neighbouring_line, char_idx + 1));
    }

    // Edge-case - single-digit number directly above the gear
    if char_idx > 0 && char_idx < line_length - 1 &&
            !neighbouring_line[char_idx-1].is_numeric() &&
            !neighbouring_line[char_idx + 1].is_numeric() &&
            neighbouring_line[char_idx].is_numeric() {
        println!("There's a single-digit number at {} in {:?}", char_idx, neighbouring_line);
        neighbouring_numbers.push(neighbouring_line[char_idx].to_string().parse::<i32>().unwrap());
    }

    return neighbouring_numbers;
}

fn find_number_by_flood_fill(line: &[char], char_idx: usize) -> i32 {
    let mut nums_as_vec_of_chars = vec![];
    let mut offset: usize = 0;
    let line_length = line.len();
    while char_idx + offset < line_length {
        let c = line[char_idx + offset];
        if c.is_numeric() {
            nums_as_vec_of_chars.push(c);
        } else {
            break;
        }
        offset += 1;
    }

    let mut nums_as_vec_going_backwards = vec![];
    let mut backwards_offset = 1;
    while backwards_offset <= char_idx {
        let c = line[char_idx - backwards_offset];
        if c.is_numeric() {
            nums_as_vec_going_backwards.push(c)
        } else {
            break
        }
        backwards_offset += 1;
    }

    nums_as_vec_going_backwards.reverse();
    nums_as_vec_going_backwards.append(&mut nums_as_vec_of_chars);
    let response = num_chars_as_vec_to_i32(nums_as_vec_going_backwards);
    println!("DEBUG - flood-filled number at {} in {:?} is {}", char_idx, line, response);
    return response;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(process_three("inputs/3-test.txt"), "4361");
        assert_eq!(process_three("inputs/3.txt"), "556367");
        assert_eq!(process_three_two("inputs/3-test.txt"), "467835");
    }
}