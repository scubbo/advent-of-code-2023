use std::{fs, cmp::Ordering};

use itertools::Itertools;

use crate::three;

pub fn solve_seven() -> String {
  return process_seven("inputs/7.txt");
}

fn process_seven(input_file: &str) -> String {
  let binding = fs::read_to_string(input_file).unwrap();
  let lines = binding.lines();
  return lines.into_iter()
    .map(|line| line.split_whitespace())
    .map(|mut split_line| (split_line.next().unwrap(), split_line.next().unwrap()))
    .map(|(hand_as_str, bid_as_str)| (Hand::new(hand_as_str), bid_as_str.parse::<u32>().unwrap()))
    // TODO - check that this sorting uses ascending, not descending!
    .sorted_by(|a, b| PartialOrd::partial_cmp(&a.0, &b.0).unwrap())
    .enumerate()
    .inspect(|(idx, (hand, _))| println!("DEBUG - the {idx}th hand is {}", hand.initial_string))
    .map(|(idx, (_, bid))| ((idx+1) as u32) * bid)
    .sum::<u32>().to_string();
}

#[derive(PartialEq)]
struct Hand {
  primary_score: u32,
  secondary_scores: Vec<u32>,
  // Don't actually need this for calculation, but it's helpful for debugging!
  initial_string: String
}

impl Hand {
  // Actually it can consume the string, but I suspect that would cause problems because Rust seems to prefer
  // to always deal with `&str`, so...not sure what the idiomatic way to do this would be?
  pub fn new(as_string: &str) -> Hand {

    let binding = as_string.chars().counts();
    let counts: Vec<&usize> = binding.values().sorted().rev().collect_vec();

    // Cannot define these within a match clause, which honestly is fair enough.
    let five_of_a_kind = vec![&5_usize];
    let four_of_a_kind = vec![&4_usize, &1_usize];
    let full_house = vec![&3_usize, &2_usize];
    let three_of_a_kind = vec![&3_usize, &1_usize, &1_usize];
    let two_pair = vec![&2_usize, &2_usize, &1_usize];
    let one_pair = vec![&2_usize, &1_usize, &1_usize, &1_usize];
    let high_card = vec![&1_usize, &1_usize, &1_usize, &1_usize, &1_usize];

    let primary_score = match counts {
      x if x == five_of_a_kind => 7,
      x if x == four_of_a_kind => 6,
      x if x == full_house => 5,
      x if x == three_of_a_kind => 4,
      x if x == two_pair => 3,
      x if x == one_pair => 2,
      x if x == high_card => 1,
      _ => panic!("Unexpected hand distribution: {:?}", counts)
    };

    let secondary_scores = as_string.chars().map(|c| match c {
      n if n.is_numeric() => n.to_digit(10).unwrap() - 1,
      'T' => 9,
      'J' => 10,
      'Q' => 11,
      'K' => 12,
      'A' => 13,
      _ => panic!("Unexpected character")
    }).collect_vec();

    println!("Primary score for {} is {primary_score}", as_string);
    println!("Secondary scores for {} are {}", as_string, secondary_scores.iter().join(","));

    return Hand { primary_score: primary_score, secondary_scores: secondary_scores, initial_string: as_string.to_string() }
    
  }


}

impl PartialOrd for Hand {
  fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
    if self.primary_score == other.primary_score && self.secondary_scores == other.secondary_scores {
      return Some(Ordering::Equal);
    }

    // This is referred to in two branches below, and it's a little involed to calculate, so I extracted it
    let first_secondary_score_difference = self.secondary_scores.iter().zip(other.secondary_scores.iter())
      .map(|(first, second)| {
        return first.partial_cmp(second).unwrap();
      })
      .filter(|o| o.is_ne())
      .next().unwrap_or(Ordering::Equal);
      

    if self.primary_score > other.primary_score || (self.primary_score == other.primary_score && first_secondary_score_difference == Ordering::Greater) {
      return Some(Ordering::Greater);
    }

    if self.primary_score < other.primary_score || (self.primary_score == other.primary_score && first_secondary_score_difference == Ordering::Less) {
      return Some(Ordering::Less);
    }

    panic!("Could not compare these hands");
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(process_seven("inputs/7-test.txt"), "6440");
  }
}