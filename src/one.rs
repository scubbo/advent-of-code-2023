use std::{fs, collections::HashMap};

pub fn solve_one() -> String {
    // I tried a regex `^\D*?(\d*).*(\d*)\D*$` for this, but it didn't work - I guess the intermediate
    // character-matching was _too_ greedy, and overrode any further digits it encountered.
    // UPDATE: And in fact that doesn't even solve the problem anyway (I initially misread it as having to find the
    // first and last _numbers_ (i.e. digit-strings) and add them, not the first and last digits)
    let input = fs::read_to_string("inputs/1.txt")
        .expect("Should have been able to read the file");
    return input.split("\n").into_iter()
        .filter(|line| !line.is_empty())
        .inspect(|line| println!("DEBUG - line is {line}"))
        .map(|line| extract_numbers_from_line(line))
        .map(|(x, y)| (10*x)+y)
        .inspect(|val| println!("DEBUG - calibration value is {val}"))
        .reduce(|accum, elem| accum + elem).unwrap().to_string();
}

pub fn solve_one_two() -> String {
    let input = fs::read_to_string("inputs/1.txt")
        .expect("Should have been able to read the file");
    return input.split("\n").into_iter()
        .filter(|line| !line.is_empty())
        .inspect(|line| println!("line is {line}"))
        .map(|line| (find_first_digit_or_number_word(line), find_last_digit_of_number_word(line)))
        .inspect(|nums| println!("First calibration sub-number is {} and last sub-number is {}", nums.0, nums.1))
        .map(|nums| (10*nums.0) + nums.1)
        .reduce(|accum, elem| accum + elem).unwrap().to_string();
}

fn find_first_digit_or_number_word(line: &str) -> i32 {
    let number_words: HashMap<&str, i32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ]);

    let line_length = line.len();
    for idx in 0..line_length {
        let char_at_idx = line.chars().nth(idx).unwrap();
        if char_at_idx.is_numeric() {
            return char_at_idx.to_string().parse::<i32>().unwrap();
        }
        for number_word_pair in &number_words {
            if (idx + number_word_pair.0.len() <= line_length) && (&&line[idx..idx+number_word_pair.0.len()] == number_word_pair.0) {
                return *number_word_pair.1;
            }
        }
        // No match, continue
    }
    panic!("Expected to find a digit in line {line}")
}

fn find_last_digit_of_number_word(line: &str) -> i32 {
    // Ugh, _lots_ of repetition here, but what the hell - this is a race, not a maintainable-code competition! :P
    let number_words: HashMap<&str, i32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ]);

    let line_length = line.len();
    for idx in (0..line_length).rev() {
        let char_at_idx = line.chars().nth(idx).unwrap();
        if char_at_idx.is_numeric() {
            return char_at_idx.to_string().parse::<i32>().unwrap();
        }
        for number_word_pair in &number_words {
            if (idx + number_word_pair.0.len() <= line_length) && (&&line[idx..idx+number_word_pair.0.len()] == number_word_pair.0) {
                return *number_word_pair.1;
            }
        }
        // No match, continue
    }
    panic!("Expected to find a digit backwards in line {line}")
}


fn extract_numbers_from_line(line: &str) -> (i32, i32) {
    let mut num1 = None;
    let mut num2 = None;
    for i in line.chars() {
        if i.is_numeric() {
            if num1.is_none() {
                num1 = Some(i.to_string().parse::<i32>().unwrap());
            } else {
                num2 = Some(i.to_string().parse::<i32>().unwrap());
            }
        }
    }

    // In case there was only a single digit-string found in the overall string
    if num2 == None {
        num2 = num1
    }
    return (num1.unwrap(), num2.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve_one(), "55208");
        assert_eq!(solve_one_two(), "54578");
    }
}