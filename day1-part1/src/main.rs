use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("./input").expect("Unable to open file");
    let f = BufReader::new(f);

    let mut max_calories = 0;
    let mut current_calories = 0;

    for line in f.lines() {
        let line = line.expect("Unable to read line");

        if !line.is_empty() {
            let calories = line.parse::<i32>().unwrap();
            current_calories += calories;
        } else {
            if current_calories > max_calories {
                max_calories = current_calories;
            }
            current_calories = 0;
        }
    }
    println!("{}", max_calories)
}
