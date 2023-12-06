// I did consider "working backwards" in this - start with output number 0, then map it _back_ through
// the maps to find if it corresponds with a starting seed; if not, check for output number 1, and so on - but suspect
// this would end up being less efficient:
// * Might need to check _many_ values before finding one which hits an initial seed, whereas there's only 20 seeds to
//    check.
// * Could sidestep the iteration by checking ranges at once, but the ranges involved are huge (billions, in some
//      cases), and if they get split then I'd need to keep following the split ranges - could get very hairy.

use std::fs;
use std::str::Lines;

use itertools::Itertools;

pub fn solve_five() -> String {
    return process_five("inputs/5.txt");
}

fn process_five(input_file: &str) -> String {
    let (seeds, almanac_maps) = parse_file_to_sources_and_maps(input_file);

    let numbers = seeds.iter().map(|seed| {
        let mut val = *seed;
        for map in &almanac_maps {
            val = apply_almanac_map(map, val);
        }
        return val;
    });

    return numbers.min().unwrap().to_string();
    
}

fn parse_file_to_sources_and_maps(input_file: &str) -> (Vec<i64>, Vec<AlmanacMap>) {
    let binding = fs::read_to_string(input_file).unwrap();
    let mut lines = binding.lines();
    // First line is a special case - describes the seeds
    let first_line = lines.next().unwrap();
    let seeds = first_line.split(": ").nth(1).unwrap()
        .split(" ").into_iter()
        .map(|s| s.parse::<i64>().unwrap()).collect_vec();
    
    let mut maps = Vec::new();
    loop {
        let next_line = lines.next();
        match next_line {
            Some(s) => {
                if s.is_empty() {
                    continue;
                } else {
                    maps.push(AlmanacMap::from_lines(&mut lines));
                }
            },
            None => break
        }
    }

    return (seeds, maps);
}

struct AlmanacMap {
    // Tried making this a `Vec`, but got "cannot move out of `self.ranges` which is behind a shared reference"
    // And a `[AlmanacMapRange]` gives `the size for values of type `[AlmanacMapRange]` cannot be known at compilation time`
    // on the return of `parse_file_to_sources_and_maps`
    ranges: Box<[AlmanacMapRange]>
}

impl AlmanacMap {
    pub fn from_lines(lines: &mut Lines) -> AlmanacMap {
        let mut ranges = Vec::new();

        loop {
            let next_line_option = lines.next();
            match next_line_option {
                Some(val) => {
                    if val.is_empty() {
                        break;
                    } else {
                        ranges.push(AlmanacMapRange::from_line(val))
                    }
                },
                None => break
            }
        }

        return AlmanacMap { ranges: ranges.into() }

    }
}

fn apply_almanac_map(map: &AlmanacMap, source: i64) -> i64 {
    return map.ranges.into_iter()
        .map(|range| apply_almanac_map_range(range, source))
        .filter(|op| op.is_some())
        .next().unwrap_or(Some(source)).unwrap();
}


#[derive(Clone, Copy)]
struct AlmanacMapRange {
    destination_range_start: i64,
    source_range_start: i64,
    range_length: i64
}

impl AlmanacMapRange {
    pub fn from_line(line: &str) -> AlmanacMapRange {
        let mut line_iter = line.split(" ");
        return AlmanacMapRange {
            destination_range_start: line_iter.next().unwrap().parse::<i64>().unwrap(),
            source_range_start: line_iter.next().unwrap().parse::<i64>().unwrap(),
            range_length: line_iter.next().unwrap().parse::<i64>().unwrap()
        }
    }
}

// I initially tried making this an implementation _on_ `AlmanacMapRange`, but ran into borrowing/moving issues.
fn apply_almanac_map_range(range: &AlmanacMapRange, val: i64) -> Option<i64> {
    if val >= range.source_range_start && (val < range.source_range_start + range.range_length) {
        return Some(val + (range.destination_range_start - range.source_range_start))
    } else {
        return None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(process_five("inputs/5-test.txt"), "35")
    }

    #[test]
    fn test_apply_range() {
        let range = AlmanacMapRange{
            destination_range_start: 10,
            source_range_start: 3,
            range_length: 2,
        };
        assert_eq!(apply_almanac_map_range(&range, 1), None);
        assert_eq!(apply_almanac_map_range(&range, 2), None);
        assert_eq!(apply_almanac_map_range(&range, 3), Some(10));
        assert_eq!(apply_almanac_map_range(&range, 4), Some(11));
        assert_eq!(apply_almanac_map_range(&range, 5), None);
    }
}
