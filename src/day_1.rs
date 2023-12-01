use regex::Regex;
use std::fs;

pub fn day_1(input_file: String) -> i32 {
    print!("Solving Day 1 puzzle.. \t");

    let input = fs::read_to_string(input_file).unwrap();

    let re = Regex::new(r"\d").unwrap();
    let mut total_sum = 0;
    for line in input.lines() {

        let matches: Vec<_> = re.find_iter(line).map(|numbers| numbers.as_str()).collect();

        if matches.len() == 0 {
            continue;
        }

        let mut number = matches[0].to_string();

        number = number + matches[matches.len() - 1];
        

        // println!("{line} --> {number}");

        total_sum += number.parse::<i32>().unwrap();
    }

    total_sum
}
