use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("./input").expect("Unable to open file");
    let f = BufReader::new(f);

    let mut calories = vec![];
    let mut current_calories = 0;

    for line in f.lines() {

        let line = line.expect("Unable to read line");

        if !line.is_empty() {
            let calories = line.parse::<i32>().unwrap();
            current_calories += calories;
        } else {
            calories.push(current_calories);
            current_calories = 0;
        }
    }

    println!("{:?}", calories);
    calories.sort();
    calories.reverse();

    let top_three_total: i32 = calories.iter().take(3).sum();

    println!("{:?}", top_three_total)
}
