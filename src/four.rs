use std::fs;
use std::collections::{HashSet, HashMap};

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
        
        let size_of_intersection = self.number_of_matches();

        return if size_of_intersection > 0 {2_usize.pow((size_of_intersection - 1).try_into().unwrap())} else {0};
    }

    pub fn number_of_matches(self) -> usize {
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
        return size_of_intersection;
    }
}

pub fn solve_four_two() -> String {
    return process_four_two("inputs/4.txt");
}

fn process_four_two(file_name: &str) -> String {
    let input = fs::read_to_string(file_name).expect("Should have been able to read file");
    let lines = input.lines().filter(|line| !line.is_empty());

    let mut extra_copies: HashMap<usize, i32> = HashMap::new();
    // A strange condition - we want to know how many original cards there are (because we cannot win extra copies of
    // cards beyond that limit), but we can't find the size of `lines` without consuming the iterator.
    // There might be a better way to do this. but my approach is to just consume the iterator, naïvely updating
    // `extra_copies` without limitation, and then reference the final `idx` after consuming the iterator to figure out
    // where to limit.
    let mut number_of_cards = 0;
    for (idx, line) in lines.enumerate() {
        let scratchcard = Scratchcard::from_line(line);
        let score = scratchcard.number_of_matches();
        println!("DEBUG - score for {idx} is {score}");
        (1..score+1).for_each(|increment| {
            let index_earning_extra_copies = idx + increment;
            let total_copies_of_current_card = 1+extra_copies.get(&idx).unwrap_or(&0);
            let number_of_extra_copies_to_create = total_copies_of_current_card;
            println!("DEBUG - adding {number_of_extra_copies_to_create} copies of {index_earning_extra_copies} due to the {total_copies_of_current_card} copies of the current card {idx}");
            extra_copies.insert(
                index_earning_extra_copies, 
                extra_copies.get(&index_earning_extra_copies).unwrap_or(&0) + number_of_extra_copies_to_create);
        });
        number_of_cards = idx;
    }
    println!("DEBUG - there are {number_of_cards} cards");
    println!("DEBUG - extra_copies is {:?}", extra_copies);
    let total_number_of_copies_of_cards = (0..number_of_cards+1)
        .map(|card_num| extra_copies.get(&card_num).unwrap_or(&0) + 1)
        .sum::<i32>();
    return total_number_of_copies_of_cards.to_string();
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn it_works() {
        // assert_eq!(process_four("inputs/4-test.txt"), "13");
        // assert_eq!(process_four("inputs/4.txt"), "25651");
        assert_eq!(process_four_two("inputs/4-test.txt"), "30")
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
