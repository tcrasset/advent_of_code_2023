use std::{fs, iter::zip};

pub fn solve(input_file: String) {
    println!("########## Solving Day 6  puzzle...##########\n");

    let input = fs::read_to_string(input_file).unwrap();

    let output_part_1 = part_1(&input);

    println!("Part 1 Result: {output_part_1}\n");
    let output_part_2 = part_2(&input);

    println!("\nPart 2 Result: {output_part_2}");
}

fn part_1(input: &String) -> i64 {
    let mut lines = input.lines();
    let race_times: Vec<i64> = lines
        .next()
        .unwrap()
        .split(":")
        .collect::<Vec<&str>>()
        .as_slice()[1]
        .split(" ")
        .map(|x| x.parse::<i64>().ok())
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();

    let record_distances: Vec<i64> = lines
        .next()
        .unwrap()
        .split(":")
        .collect::<Vec<&str>>()
        .as_slice()[1]
        .split(" ")
        .map(|x| x.parse::<i64>().ok())
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();

    // println!("{:?}", race_times);
    // println!("{:?}", record_distances);

    let mut result = 1;

    for (race_time, record_distance) in zip(race_times, record_distances) {
        // let nb_ways_optimized = get_numbers_of_ways_one_can_beat_the_record_optimized(
        //     race_time as i64,
        //     record_distance as i64,
        // );
        let _nb_ways_naive =
            _get_numbers_of_ways_one_can_beat_the_record_naive(race_time, record_distance as i64);
        // println!("nb_ways_optimized: {nb_ways_optimized}");
        // println!("nb_ways_naive: {nb_ways_naive}");
        result *= _nb_ways_naive;

    }

    return result as i64;
}

fn part_2(input: &String) -> i64 {
    let mut lines = input.lines();
    // let time: Vec<i64> = lines
    let race_times: Vec<i64> = lines
        .next()
        .unwrap()
        .split(":")
        .collect::<Vec<&str>>()
        .as_slice()[1]
        .split(" ")
        .map(|x| x.parse::<i64>().ok())
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();

    let record_distances: Vec<i64> = lines
        .next()
        .unwrap()
        .split(":")
        .collect::<Vec<&str>>()
        .as_slice()[1]
        .split(" ")
        .map(|x| x.parse::<i64>().ok())
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();

    let race_time_without_kerning = race_times
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<i64>()
        .unwrap();
    let record_distance_without_kerning = record_distances
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<i64>()
        .unwrap();

    println!("race_time_without_kerning: {:?}", race_time_without_kerning);
    println!(
        "record_distance_without_kerning: {:?}",
        record_distance_without_kerning
    );

    return _get_numbers_of_ways_one_can_beat_the_record_naive(
        race_time_without_kerning as i64,
        record_distance_without_kerning,
    );
}
fn _get_numbers_of_ways_one_can_beat_the_record_naive(race_time: i64, distance_record: i64) -> i64 {
    let mut distances_higher_than_records: Vec<i64> = Vec::new();
    for time_spent_pushing_button in 1..race_time - 1 {
        let time_remaining_to_move = race_time - time_spent_pushing_button;
        let speed = time_spent_pushing_button;
        let distance_travelled = time_remaining_to_move * speed;
        if distance_travelled > distance_record {
            distances_higher_than_records.push(distance_travelled);
        }
    }

    // println!(
    //     "distances_higher_than_records: {:?}",
    //     distances_higher_than_records
    // );
    let number_of_ways_one_can_beat_the_record = distances_higher_than_records.len();

    return number_of_ways_one_can_beat_the_record as i64;
}

fn get_numbers_of_ways_one_can_beat_the_record_optimized(
    race_time: i64,
    distance_record: i64,
) -> i64 {
    let mut start_index = 1;
    let mut stop_index = race_time - 1;

    let mut distance_travelled;
    let mut pivot = 0;
    let mut is_last_operation_stop = false;
    // Let's do binary search to find the lowest time_spent_pushing_button (pivot) that beats the record
    while start_index <= stop_index {
        pivot = (start_index + stop_index) / 2;
        let time_remaining_to_move = race_time - pivot;
        let speed: i64 = pivot;
        distance_travelled = (time_remaining_to_move as i64 * speed as i64) as i64;

        if distance_travelled > distance_record {
            stop_index = pivot - 1;
            is_last_operation_stop = true;
        } else {
            start_index = pivot + 1;
            is_last_operation_stop = false;
        }
    }

    let final_pivot = if is_last_operation_stop {
        pivot
    } else {
        pivot + 1
    };
    let first_time_spent_pushing_button_that_exceeds_record = final_pivot;
    let first_distance_record = (race_time - first_time_spent_pushing_button_that_exceeds_record)
        * first_time_spent_pushing_button_that_exceeds_record;

    let last_time_spent_pushing_button_that_exceeds_record = race_time - (final_pivot);
    let last_distance_record = (race_time - last_time_spent_pushing_button_that_exceeds_record)
        * last_time_spent_pushing_button_that_exceeds_record;
    let nb_ways = last_time_spent_pushing_button_that_exceeds_record
        - first_time_spent_pushing_button_that_exceeds_record
        + 1;
    println!("The first time we beat the record is by pushing the button for {first_time_spent_pushing_button_that_exceeds_record} seconds.");
    println!(
        "We thus have {:?} seconds to run.",
        race_time - first_time_spent_pushing_button_that_exceeds_record
    );
    println!("That gives us a total distance travelled of {first_distance_record}, the record is {distance_record}");
    println!("The max time we can push the button to still have the record is {:?}, which is the previous time that we could run.", last_time_spent_pushing_button_that_exceeds_record);
    println!(
        "That gives us a total distance travelled of {:?}",
        last_distance_record
    );
    println!(
        "We just have to sum up all the ways, which is {:?} ways",
        nb_ways
    );

    return nb_ways;
}
