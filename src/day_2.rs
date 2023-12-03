use std::{cmp::max, collections::HashSet, fs};

use regex::Regex;

pub fn solve(input_file: String) {
    println!("### Solving Day 2 puzzle...###");

    let input = fs::read_to_string(input_file).unwrap();

    let current_configuration = Configuration {
        nb_reds: 12,
        nb_blues: 14,
        nb_greens: 13,
    };
    let valid_identifiers = part_1(input.to_owned(), current_configuration);
    let cube_powers = part_2(input.to_owned());

    let total_identifier_sum: i32 = valid_identifiers.iter().sum();
    let total_cube_power_sum: i32 = cube_powers.iter().sum();

    println!("Part 1 Result: {total_identifier_sum}");
    println!("Part 2 Result: {total_cube_power_sum}");
}

fn part_1(input: String, current_configuration: Configuration) -> HashSet<i32> {
    let id_regex = Regex::new(r"Game (\d+):(.*)").unwrap();

    let mut invalid_game_identifiers: HashSet<i32> = HashSet::new();
    let mut game_identifiers: HashSet<i32> = HashSet::new();
    for line in input.lines() {
        // println!("{line}");
        let (_, [game_identifier, game]) = id_regex.captures(line).unwrap().extract();

        let sets: Vec<&str> = game.split(';').map(|x| x.trim()).collect();
        game_identifiers.insert(game_identifier.parse::<i32>().unwrap());

        let configurations_per_game = get_configurations_per_game(sets);
        // println!("{:?}", configurations_per_game);

        for possible_configuration in configurations_per_game.iter() {
            if !is_possible(current_configuration, *possible_configuration) {
                invalid_game_identifiers.insert(game_identifier.parse::<i32>().unwrap());
            }
        }

        // println!("{:?}", invalid_game_identifiers);
    }
    let valid_game_identifiers: HashSet<_> = game_identifiers
        .difference(&invalid_game_identifiers)
        .map(|x| *x)
        .collect();
    return valid_game_identifiers;
}

fn part_2(input: String) -> Vec<i32> {
    let id_regex = Regex::new(r"Game (\d+):(.*)").unwrap();

    let mut minimal_cube_powers: Vec<i32> = Vec::new();
    for line in input.lines() {
        // println!("{line}");
        let (_, [__, game]) = id_regex.captures(line).unwrap().extract();

        let sets: Vec<&str> = game.split(';').map(|x| x.trim()).collect();

        let configurations_per_game = get_configurations_per_game(sets);

        let minimal_configuration = get_minimal_working_configuration(configurations_per_game);
        let cube_power = minimal_configuration.nb_blues
            * minimal_configuration.nb_reds
            * minimal_configuration.nb_greens;
        minimal_cube_powers.push(cube_power)
    }
    return minimal_cube_powers;
}

fn get_configurations_per_game(sets: Vec<&str>) -> Vec<Configuration> {
    let configuration_regex = Regex::new(r"(\d+ \w+)+").unwrap();
    let mut configurations_per_game: Vec<Configuration> = Vec::new();

    for set in sets.iter() {
        let matches: Vec<_> = configuration_regex
            .find_iter(set)
            .map(|configurations| configurations.as_str())
            .collect();

        let configuration: Configuration = get_configuration(matches);
        configurations_per_game.push(configuration);
    }

    return configurations_per_game;
}

fn get_minimal_working_configuration(configurations_per_game: Vec<Configuration>) -> Configuration {
    let mut max_reds = 0;
    let mut max_greens = 0;
    let mut max_blues = 0;
    for possible_configuration in configurations_per_game.iter() {
        max_reds = max(max_reds, possible_configuration.nb_reds);
        max_greens = max(max_greens, possible_configuration.nb_greens);
        max_blues = max(max_blues, possible_configuration.nb_blues);
    }

    return Configuration {
        nb_reds: max_reds,
        nb_greens: max_greens,
        nb_blues: max_blues,
    };
}
#[derive(Debug, Copy, Clone)]
struct Configuration {
    nb_reds: i32,
    nb_blues: i32,
    nb_greens: i32,
}

fn get_configuration(set_inputs: Vec<&str>) -> Configuration {
    let nb_color_regex = Regex::new(r"(\d+) (\w+)").unwrap();

    let mut nb_reds = 0;
    let mut nb_greens = 0;
    let mut nb_blues = 0;

    for input in set_inputs {
        let (full, [number, color]) = nb_color_regex.captures(input).unwrap().extract();

        match color {
            "red" => nb_reds += number.parse::<i32>().unwrap(),
            "green" => nb_greens += number.parse::<i32>().unwrap(),
            "blue" => nb_blues += number.parse::<i32>().unwrap(),
            _ => eprintln!("Problem parsing arguments."),
        }
    }

    let configuration = Configuration {
        nb_reds: nb_reds,
        nb_blues: nb_blues,
        nb_greens: nb_greens,
    };

    return configuration;
    // println!("{:? :?}", number, color);
}

fn is_possible(
    current_configuration: Configuration,
    potential_configuration: Configuration,
) -> bool {
    return current_configuration.nb_blues >= potential_configuration.nb_blues
        && current_configuration.nb_greens >= potential_configuration.nb_greens
        && current_configuration.nb_reds >= potential_configuration.nb_reds;
}
