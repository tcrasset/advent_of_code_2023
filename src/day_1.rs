use regex::Regex;
use std::{collections::HashMap, fs};

pub fn solve(input_file: String) {
    println!("### Solving Day 1  puzzle...###");

    let input = fs::read_to_string(input_file).unwrap();

    println!("Part 1 Result: {:?}", part_1(input.to_owned()));
    println!("Part 2 Result: {:?}", part_2(input.to_owned()));
}

fn part_1(input: String) -> i32 {
    return get_computed_sum(input);
}

fn part_2(input: String) -> i32 {
    return get_computed_sum(get_replaced_spelled_out_digits(input));
}

fn get_computed_sum(input: String) -> i32 {
    let re = Regex::new(r"\d").unwrap();
    let mut total_sum: i32 = 0;

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

    return total_sum;
}
fn get_replaced_spelled_out_digits(input: String) -> String {
    let name_to_int = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("si", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let mut as_letter_to_replacement_pattern = HashMap::new();

    for (as_letter, as_value) in name_to_int {
        as_letter_to_replacement_pattern.insert(
            as_letter.to_string(),
            as_letter.to_owned() + as_value + as_letter,
        );
    }

    let mut sanitized_input = input.to_owned();
    for (from, to) in &as_letter_to_replacement_pattern {
        sanitized_input = sanitized_input.replace(from, to);
    }

    return sanitized_input;
}
