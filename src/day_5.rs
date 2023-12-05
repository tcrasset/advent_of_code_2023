use regex::Regex;
use std::{cmp::min, collections::HashMap, fs, ops::Range};

pub fn solve(input_file: String) {
    println!("### Solving Day 5  puzzle...###");

    let input = fs::read_to_string(input_file).unwrap();

    let closest_location = part_1(&input);

    println!("Part 1 Result: {closest_location}");
    // let closest_location_in_seed_ranges = part_2(&input);
    // println!("Part 2 Result: {closest_location_in_seed_ranges}");
}

#[derive(Debug)]
struct MapElement {
    destination_range_start: i64,
    source_range_start: i64,
    range_length: i64,
}

#[derive(Eq, PartialEq, Hash)]

enum MapKind {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

fn part_1(input: &String) -> i64 {
    let starter_seeds = input
        .lines()
        .next()
        .unwrap()
        .split(" ")
        .collect::<Vec<&str>>()
        .as_slice()[1..]
        .to_vec();

    let almanac: HashMap<MapKind, Vec<MapElement>> = get_almanac(&input);

    let mut locations: Vec<i64> = Vec::new();
    for seed in starter_seeds.iter().map(|x| x.parse::<i64>().unwrap()) {
        let location = get_location_from_seed(&almanac, seed);
        locations.push(location);
    }

    return locations.iter().min().unwrap().to_owned();
}

fn _part_2(input: &String) -> i64 {
    let seed_ranges = _get_seed_ranges(input);
    let almanac: HashMap<MapKind, Vec<MapElement>> = get_almanac(&input);

    let mut current_closest_location = i64::MAX;

    for range in seed_ranges {
        println!("Current range: {:?}", range);
        let end = range.end;
        for seed in range {
            if seed % 100000 == 0 {
                println!("Current milestone: [{:?}/{:?}]", seed, end);
            }

            let location: i64 = get_location_from_seed(&almanac, seed);
            current_closest_location = min(location, current_closest_location);
        }
    }

    return current_closest_location;
}

fn _get_seed_ranges(input: &String) -> Vec<Range<i64>> {
    let binding = input
        .lines()
        .next()
        .unwrap()
        .split(" ")
        .collect::<Vec<&str>>()
        .as_slice()[1..]
        .to_vec();

    let mut seed_starters_and_ranges = binding.iter().peekable();

    let mut seed_ranges: Vec<Range<i64>> = Vec::new();
    while seed_starters_and_ranges.peek().is_some() {
        println!("{:?}", seed_starters_and_ranges.peek());
        let start = seed_starters_and_ranges
            .next()
            .unwrap()
            .parse::<i64>()
            .unwrap();
        println!("{:?}", seed_starters_and_ranges.peek());
        let length = seed_starters_and_ranges
            .next()
            .unwrap()
            .parse::<i64>()
            .unwrap();

        seed_ranges.push(Range {
            start: start,
            end: start + length,
        });
    }

    return seed_ranges;
}

fn get_location_from_seed(almanac: &HashMap<MapKind, Vec<MapElement>>, seed: i64) -> i64 {
    let soil: i64 = get_destination_from_map(&almanac[&MapKind::SeedToSoil], &seed);
    let fertilizer: i64 = get_destination_from_map(&almanac[&MapKind::SoilToFertilizer], &soil);
    let water: i64 = get_destination_from_map(&almanac[&MapKind::FertilizerToWater], &fertilizer);
    let ligth: i64 = get_destination_from_map(&almanac[&MapKind::WaterToLight], &water);
    let temperature: i64 = get_destination_from_map(&almanac[&MapKind::LightToTemperature], &ligth);
    let humidity: i64 =
        get_destination_from_map(&almanac[&MapKind::TemperatureToHumidity], &temperature);
    let location: i64 = get_destination_from_map(&almanac[&MapKind::HumidityToLocation], &humidity);

    return location;
}

fn get_almanac(input: &String) -> HashMap<MapKind, Vec<MapElement>> {
    let map_kinds: HashMap<&str, MapKind> = get_map_kinds();
    let mut almanac: HashMap<MapKind, Vec<MapElement>> = HashMap::new();

    for (key, kind) in map_kinds {
        let map = extract_map(input, key);
        almanac.insert(kind, map);
    }

    return almanac;
}
fn get_map_kinds() -> HashMap<&'static str, MapKind> {
    return HashMap::from([
        ("seed-to-soil", MapKind::SeedToSoil),
        ("soil-to-fertilizer", MapKind::SoilToFertilizer),
        ("fertilizer-to-water", MapKind::FertilizerToWater),
        ("water-to-light", MapKind::WaterToLight),
        ("light-to-temperature", MapKind::LightToTemperature),
        ("temperature-to-humidity", MapKind::TemperatureToHumidity),
        ("humidity-to-location", MapKind::HumidityToLocation),
    ]);
}

fn extract_map_element(line: &String) -> MapElement {
    let re = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();
    let (_, [destination_range_start, source_range_start, range_length]) =
        re.captures(line).unwrap().extract();

    return MapElement {
        destination_range_start: destination_range_start.parse::<i64>().unwrap(),
        source_range_start: source_range_start.parse::<i64>().unwrap(),
        range_length: range_length.parse::<i64>().unwrap(),
    };
}

fn is_contained_in(element: &MapElement, source: &i64) -> bool {
    return (element.source_range_start..element.source_range_start + element.range_length)
        .contains(&source);
}

fn get_destination(element: &MapElement, source: &i64) -> i64 {
    // We know that the destination and source all increment by one, for {range} iterations.
    // If we figure out the difference between the destination and the source,
    // the destination to the corresponding source will always {difference} iterations away.
    let difference = element.destination_range_start - element.source_range_start;

    return source + difference;
}

fn get_destination_from_map(map: &Vec<MapElement>, source: &i64) -> i64 {
    for element in map {
        if is_contained_in(&element, source) {
            return get_destination(&element, source);
        }
    }

    // Any source numbers that aren't mapped correspond to the same destination number.
    // So, seed number 10 corresponds to soil number 10.
    return *source;
}

fn extract_map(input: &String, map_kind: &str) -> Vec<MapElement> {
    let mut buffer = String::new();
    let mut found_map = false;
    let mut map_elements: Vec<MapElement> = Vec::new();
    for line in input.lines() {
        if found_map {
            let is_empty_line = line == "";
            if is_empty_line {
                // Once we land on a line break, we are at the end of a section.
                break;
            }
            // We push every line after the header into a buffer.
            buffer.push_str((line.to_owned() + "\n").as_str());
            map_elements.push(extract_map_element(&line.to_string()))
        }
        if line.contains(map_kind) {
            found_map = true;
        }
        // We continue until we find the section we want
    }
    // println!("Extracted map \n{:?}", buffer);
    return map_elements;
}
