use std::{fs, iter::zip, num};

pub fn solve_six() -> String {
    return process_six("inputs/6.txt");
}

fn process_six(input_file: &str) -> String {
    let file_string = &fs::read_to_string(input_file).unwrap();
    let mut lines = file_string.lines();
    let mut times = lines.next().unwrap().split_whitespace().into_iter();
    let mut distances = lines.next().unwrap().split_whitespace().into_iter();
    times.next(); // Advance past the label
    distances.next(); // ""
    let time_distance = zip(times.map(|s| s.parse::<i32>().unwrap()), distances.map(|s| s.parse::<i32>().unwrap()));
    return time_distance.map(|(time, distance)| how_many_ways_can_distance_be_beaten(time.into(), distance.into()))
        .reduce(|accum, elem| {
            println!("DEBUG - accum is {accum} and elem is {elem}");
            return accum * elem;
        }).unwrap().to_string();

    // return "Hello world!".to_string();
}

fn how_many_ways_can_distance_be_beaten(time: i128, distance: i128) -> i128 {
    // For time t, if we hold for h, then the boat will travel `d(h) = (t-h)*h = -h^2+th`
    // We want to know how many integer values result in a value higher than `distance`
    // While we _could_ calculate this iteratively, this is simple mathematics - `-h^2 + th - distance` is a negative
    // quadratic, which will have (usually) two roots, so we find those and subtract them (and watch out for
    // a. off-by-one errors, and b. situations where there are single or no roots. Though, since the question asks us to
    // multiply the solutions together, I'm willing to gamble that this function always returns >0 for the data given...)
    let discriminant = time.pow(2) - 4 * distance;
    println!("DEBUG - discriminant for {time} and {distance} is {discriminant}");
    let rooted = (discriminant as f64).sqrt();
    println!("DEBUG - rooted, is {rooted}"); // I _really hope_ all the discriminants are perfect squares...(edit - nope)
    let rooted_i32 = rooted; // Maintain as a float so we can check boundary conditions
    let sol_high = ((time as f64) + rooted_i32) / (2 as f64);
    let sol_low = ((time as f64) - rooted_i32) / (2 as f64);
    println!("DEBUG - sol_high for {time} and {distance} is {sol_high}");
    println!("DEBUG - sol_low for {time} and {distance} is {sol_low}");
    let number_of_intermediate_values = (((sol_high.floor() - sol_low.ceil()) as i128) - 1) + {
        if sol_high > sol_high.floor() {
            1
        } else {
            0
        }
    } + {
        if sol_low < sol_low.ceil() {
            1
        } else {
            0
        }
    };
    println!("DEBUG - return_value is {number_of_intermediate_values}");
    return number_of_intermediate_values;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(how_many_ways_can_distance_be_beaten(7, 9),  4);
        assert_eq!(how_many_ways_can_distance_be_beaten(15, 40),  8);
        assert_eq!(process_six("inputs/6-test.txt"), "288");
        assert_eq!(how_many_ways_can_distance_be_beaten(61677571, 430103613071150), 45647654)
    }
}