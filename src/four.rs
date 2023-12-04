use std::fs;
use std::collections::HashSet;

use itertools::Itertools;

pub fn solve_four() -> String {
    return process_four("inputs/4.txt");
}

fn process_four(file_name: &str) -> String {
    let input = fs::read_to_string(file_name).expect("Should have been able to read file");
    let lines = input.lines().filter(|line| !line.is_empty());
    let cards = lines.map(Scratchcard::from_line);
    return cards.map(|card| card.score()).sum::<usize>().to_string();
}

// Intentionally leaning into a more OO-based approach for this one - trying to get familiarity with more of Rust!
struct Scratchcard {
    wants: HashSet<i32>,
    haves: HashSet<i32>
}

impl Scratchcard {
    pub fn from_line(line: &str) -> Scratchcard {
        let numbers = line.split(": ").nth(1).unwrap();
        let mut split_numbers = numbers.split(" | ");

        let wants_iter = split_numbers.nth(0).unwrap()
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i32>().unwrap());
        // `nth(0)` is correct because `Split` is an iterator (which has already been advanced by the previous `nth(0)` call)
        let haves_iter = split_numbers.nth(0).unwrap()
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i32>().unwrap());
        return Scratchcard {
            wants: HashSet::from_iter(wants_iter),
            haves: HashSet::from_iter(haves_iter)
        }
    }

    // Thinking about the signature, I _think_ it's ok for this to take `self` rather than `&self` (that is, for the
    // object to be consumed by the method), because we won't want to do anything else with this object after getting
    // its score.
    pub fn score(self) -> usize {
        println!("DEBUG - calling score on");
        println!("{:?}", self.wants);
        println!("{:?}", self.haves);
        let mut inters = self.wants.intersection(&self.haves);
        // Cannot use `inters.try_len()` because that returns an inaccurate value - e.g. `5` for the first line of the
        // test-case (when it should be 4). Why?
        // (See `size_of_intersection_behaves_unexpectedly`, below)
        let mut size_of_intersection = 0;
        loop {
            match inters.next() {
                Some(_) => size_of_intersection += 1,
                None => break
            }
        }

        
        return if size_of_intersection > 0 {2_usize.pow(size_of_intersection - 1)} else {0};
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(process_four("inputs/4-test.txt"), "13");
        assert_eq!(process_four("inputs/4.txt"), "25651")
    }

    #[test]
    fn size_of_intersection_behaves_unexpectedly() {
        let set1 = HashSet::from([17, 41, 83, 48, 86]);

        let set2 = HashSet::from([86, 17, 9, 31, 53, 48, 83, 6]);

        // Intersection is {17, 83, 48, 86} - i.e. has size 4
        let intersection = set1.intersection(&set2);
        let intersection_size = intersection.try_len();
        // We would expect `intersection_size` to be 4, but it's actually `Err((0, Some(5))`
        assert_eq!(intersection_size.unwrap_err().1.unwrap(), 5);

        // What happens if we check the intersection in the other direction?
        let second_intersection = set2.intersection(&set1);
        assert_eq!(second_intersection.try_len().unwrap_err().1.unwrap(), 5);
    }
}
