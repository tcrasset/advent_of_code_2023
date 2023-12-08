use std::{cmp::Ordering, collections::HashMap, fs, iter::zip};

pub fn solve(input_file: String) {
    println!("########## Solving Day 7  puzzle...##########\n");

    let input = fs::read_to_string(input_file).unwrap();

    let output_part_1 = part_1(&input);

    println!("Part 1 Result: {output_part_1}\n");
    let output_part_2: i64 = part_2(&input);

    println!("\nPart 2 Result: {output_part_2}");
}

fn part_1(input: &String) -> i64 {
    let hands = input
        .lines()
        .map(|line| line.split(" ").nth(0).unwrap())
        .collect::<Vec<&str>>();
    let bids = input
        .lines()
        .map(|line| line.split(" ").nth(1).unwrap())
        .map(|bid| bid.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    // We are going to sort the hands by their rank, from weakest (rank 1) to strongest (rank N)
    let compare_ordering = |hand_1: &str, hand_2: &str| -> Ordering {
        match compare_hands_part_1(hand_1, hand_2) {
            -1 => Ordering::Greater,
            0 => Ordering::Equal,
            1 => Ordering::Less,
            _ => !panic!(),
        }
    };

    let hands_to_bids: HashMap<&str, i32> =
        hands.iter().cloned().zip(bids.iter().cloned()).collect();

    let mut sorted_hands = hands.to_owned();
    sorted_hands.sort_by(|hand_1, hand_2| compare_ordering(hand_1, hand_2));

    for (h, h_sort) in zip(hands, &sorted_hands) {
        println!("{:?}, {:?}", h, h_sort);
    }

    let mut total_winnings = 0 as i64;

    for (index, hand) in sorted_hands.iter().enumerate() {
        let bid = hands_to_bids.get(hand).unwrap();
        let rank = index + 1;
        let amount_won = (rank as i32 * bid) as i64;
        println!("Total winning of {hand} is {amount_won} ({rank} * {bid})");
        total_winnings += amount_won;
    }

    return total_winnings;
}

fn part_2(input: &String) -> i64 {
    let hands = input
        .lines()
        .map(|line| line.split(" ").nth(0).unwrap())
        .collect::<Vec<&str>>();
    let bids = input
        .lines()
        .map(|line| line.split(" ").nth(1).unwrap())
        .map(|bid| bid.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    // We are going to sort the hands by their rank, from weakest (rank 1) to strongest (rank N)
    let compare_ordering = |hand_1: &str, hand_2: &str| -> Ordering {
        match compare_hands_part_2(hand_1, hand_2) {
            -1 => Ordering::Greater,
            0 => Ordering::Equal,
            1 => Ordering::Less,
            _ => !panic!(),
        }
    };

    let hands_to_bids: HashMap<&str, i32> =
        hands.iter().cloned().zip(bids.iter().cloned()).collect();

    let mut sorted_hands = hands.to_owned();
    sorted_hands.sort_by(|hand_1, hand_2| compare_ordering(hand_1, hand_2));

    for (h, h_sort) in zip(hands, &sorted_hands) {
        println!("{:?}, {:?}", h, h_sort);
    }

    let mut total_winnings = 0 as i64;

    for (index, hand) in sorted_hands.iter().enumerate() {
        let bid = hands_to_bids.get(hand).unwrap();
        let rank = index + 1;
        let amount_won = (rank as i32 * bid) as i64;
        println!("Total winning of {hand} is {amount_won} ({rank} * {bid})");
        total_winnings += amount_won;
    }

    return total_winnings;
}

#[derive(PartialEq, PartialOrd, Debug)]
// When derived on enums, variants are ordered by their discriminants.
// By default, the discriminant is smallest for variants at the top, and largest for variants at the bottom. Here’s an example:
enum HandType {
    HighCard,
    SinglePair,
    DoublePair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn compare_hands_part_1(hand_1: &str, hand_2: &str) -> i8 {
    // Returns -1 if hand_1 is stronger, 0 if equal, 1 if card_2 is stronger
    let hand_1_type = get_hand_type(hand_1);
    let hand_2_type = get_hand_type(hand_2);

    let type_comparison = compare_types(&hand_1_type, &hand_2_type);

    if type_comparison != 0 {
        // Either hand_1 is a different type, or hand_2.
        // We return the result of the comparison, which is the same
        // as the current function.
        return type_comparison;
    } else {
        // If two hands have the same type, a second ordering rule takes effect.
        // Start by comparing the first card in each hand.
        // If these cards are different, the hand with the stronger first card is considered stronger.
        // If the first card in each hand have the same label, however, then move on to considering the second card in each hand.
        // If they differ, the hand with the higher second card wins; otherwise,
        // continue with the third card in each hand, then the fourth, then the fifth.
        for (hand_1_card, hand_2_card) in zip(hand_1.chars(), hand_2.chars()) {
            let card_comparison = compare_card_part_1(hand_1_card, hand_2_card);

            if card_comparison != 0 {
                return card_comparison;
            }
        }

        // We should not reach here, no cards should be identical.
        !panic!()
    }
}

fn compare_hands_part_2(hand_1: &str, hand_2: &str) -> i8 {
    // Returns -1 if hand_1 is stronger, 0 if equal, 1 if card_2 is stronger

    let (upgraded_hand_1_type, upgraded_hand_1) = try_get_best_hand_type(hand_1);
    let (upgraded_hand_2_type, upgraded_hand_2) = try_get_best_hand_type(hand_2);

    let type_comparison = compare_types(&upgraded_hand_1_type, &upgraded_hand_2_type);

    if type_comparison != 0 {
        // Either hand_1 is a different type, or hand_2.
        // We return the result of the comparison, which is the same
        // as the current function.
        return type_comparison;
    } else {
        println!(
            "{hand_1}, {hand_2} --> {upgraded_hand_1}, {upgraded_hand_2} [{:?}, {:?}]",
            upgraded_hand_1_type, upgraded_hand_2_type
        );
        //  However,
        // for the purpose of breaking ties between two hands of the same type,
        // J is always treated as J, not the card it's pretending to be:
        // JKKK2 is weaker than QQQQ2 because J is weaker than Q.
        //
        // Here we are thus using the original hands, not the upgraded ones.
        for (hand_1_card, hand_2_card) in zip(hand_1.chars(), hand_2.chars()) {
            let card_comparison = compare_card_part_2(hand_1_card, hand_2_card);

            if card_comparison != 0 {
                return card_comparison;
            }
        }

        // We should not reach here, no cards should be identical.
        !panic!()
    }
}

fn _find_keys_for_value<'a>(map: &'a HashMap<char, i8>, value: i8) -> Vec<char> {
    map.iter()
        .filter_map(|(key, &val)| {
            if val == value {
                Some(key.clone())
            } else {
                None
            }
        })
        .collect()
}

fn try_get_best_hand_type(hand: &str) -> (HandType, String) {
    // Returns the next best HandType, and the corresponding next best hand.
    let current_type = get_hand_type(hand);
    if hand.chars().position(|card| card == 'J').is_none() {
        // There is no joker in the hand, we just return the standard
        // type.
        return (current_type, hand.to_owned());
    } else {
        // We replace the joker 'J' with the card that will upgrade the type.
        // We thus have to find the highest card in the hand and upgrade the
        // 'J' to it.

        let hand_count = count_cards(hand);
        return match current_type {
            HandType::FiveOfAKind => (HandType::FiveOfAKind, hand.to_owned()), // JJJJJ -> JJJJJ
            HandType::FourOfAKind => {
                let (high, low) = get_four_of_kind_cards(&hand_count).unwrap();

                if high == 'J' {
                    // JJJJ2 --> 22222

                    let mut new_hand = hand.replace('J', low.to_string().as_str());
                    return (HandType::FiveOfAKind, new_hand);
                } else {
                    // 2222J --> 2222
                    let mut new_hand = hand.replace('J', high.to_string().as_str());
                    return (HandType::FiveOfAKind, new_hand);
                }
            }
            HandType::FullHouse => {
                let (high, low) = get_full_house_cards(&hand_count).unwrap();

                if high == 'J' {
                    // JJJ22 --> 22222

                    let mut new_hand = hand.replace('J', low.to_string().as_str());
                    return (HandType::FiveOfAKind, new_hand);
                } else {
                    // 222JJ --> 2222
                    let mut new_hand = hand.replace('J', high.to_string().as_str());
                    return (HandType::FiveOfAKind, new_hand);
                }
            }
            HandType::ThreeOfAKind => {
                let high = get_three_of_kind_cards(&hand_count).unwrap();
                if high == 'J' {
                    // JJJ12 -> 22212 (we replace by 2 and not 1, as 2 is stronger)
                    let other_cards: Vec<char> = _find_keys_for_value(&hand_count, 1);
                    let sorted_cards: Vec<char> =
                        sort_cards_from_strongest_to_weakest_part_2(other_cards);

                    let strongest_card: &char = sorted_cards.iter().nth(0).unwrap();
                    let new_hand: String = hand.replace('J', strongest_card.to_string().as_str());
                    return (HandType::FourOfAKind, new_hand);
                } else {
                    // 222J1 --> 22221
                    let new_hand: String = hand.replace('J', high.to_string().as_str());
                    return (HandType::FourOfAKind, new_hand);
                }
            }

            HandType::DoublePair => {
                let (high, low) = get_double_pair_cards(&hand_count).unwrap();

                if high == 'J' {
                    // JJ228 --> 22228
                    let new_hand = hand.replace('J', low.to_string().as_str());
                    return (HandType::FourOfAKind, new_hand);
                } else if low == 'J' {
                    // KKJJ2 --> KKKK2
                    let new_hand: String = hand.replace('J', high.to_string().as_str());
                    return (HandType::FourOfAKind, new_hand);
                } else {
                    // 1122J --> 11222
                    let other_cards: Vec<char> = _find_keys_for_value(&hand_count, 2);
                    let sorted_cards: Vec<char> =
                        sort_cards_from_strongest_to_weakest_part_2(other_cards);

                    let strongest_card: &char = sorted_cards.iter().nth(0).unwrap();
                    let new_hand: String = hand.replace('J', strongest_card.to_string().as_str());
                    return (HandType::FullHouse, new_hand);
                }
            }

            HandType::SinglePair => {
                let pair = get_single_pair_cards(&hand_count).unwrap();

                if pair == 'J' {
                    // JJ123 --> JJJ23 (we replace the weakest)
                    let other_cards: Vec<char> = _find_keys_for_value(&hand_count, 1);
                    let sorted_cards: Vec<char> =
                        sort_cards_from_strongest_to_weakest_part_2(other_cards);

                    let weakest_card: &char = sorted_cards.iter().last().unwrap();
                    let new_hand: String = hand.replace('J', weakest_card.to_string().as_str());
                    return (HandType::ThreeOfAKind, new_hand);
                } else {
                    // KKJ12 --> KKK12
                    let new_hand: String = hand.replace('J', pair.to_string().as_str());
                    return (HandType::ThreeOfAKind, new_hand);
                }
            }
            HandType::HighCard => {
                // 1234J --> 12344
                let sorted_cards: Vec<char> =
                    sort_cards_from_strongest_to_weakest_part_2(hand.chars().collect());

                let strongest_card: &char = sorted_cards.iter().nth(0).unwrap();
                let new_hand: String = hand.replace('J', strongest_card.to_string().as_str());

                return (HandType::SinglePair, new_hand);
            }
        };
    }
}
fn get_hand_type(hand: &str) -> HandType {
    let hand_count = count_cards(hand);

    if get_five_of_kind_cards(&hand_count).is_some() {
        return HandType::FiveOfAKind;
    } else if get_four_of_kind_cards(&hand_count).is_some() {
        return HandType::FourOfAKind;
    } else if get_full_house_cards(&hand_count).is_some() {
        return HandType::FullHouse;
    } else if get_three_of_kind_cards(&hand_count).is_some() {
        return HandType::ThreeOfAKind;
    } else if get_double_pair_cards(&hand_count).is_some() {
        return HandType::DoublePair;
    } else if get_single_pair_cards(&hand_count).is_some() {
        return HandType::SinglePair;
    } else {
        return HandType::HighCard;
    }
}

fn compare_types(hand_1_type: &HandType, hand_2_type: &HandType) -> i8 {
    //Return -1 if hand_1_type is higher than hand_2_type, 0 if equal, 1 if hand_2_type is higher.

    if hand_1_type > hand_2_type {
        return -1;
    } else if hand_1_type == hand_2_type {
        return 0;
    } else {
        return 1;
    }
}

fn compare_card_part_1(card_1: char, card_2: char) -> i8 {
    let strongest_to_weakest: [char; 13] = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];

    return _compare_card(card_1, card_2, strongest_to_weakest);
}

fn compare_card_part_2(card_1: char, card_2: char) -> i8 {
    let strongest_to_weakest: [char; 13] = [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];

    return _compare_card(card_1, card_2, strongest_to_weakest);
}

fn _compare_card(card_1: char, card_2: char, strongest_to_weakest: [char; 13]) -> i8 {
    // Returns -1 if card_1 is stronger than card_2, 0 if equal, 1 if card_2 is stronger.

    let card_1_position = strongest_to_weakest
        .to_vec()
        .iter()
        .position(|x: &char| x == &card_1)
        .unwrap();

    let card_2_position = strongest_to_weakest
        .to_vec()
        .iter()
        .position(|x: &char| x == &card_2)
        .unwrap();

    if card_1_position < card_2_position {
        return -1;
    } else if card_1_position == card_2_position {
        return 0;
    } else {
        return 1;
    }
}

fn sort_cards_from_strongest_to_weakest_part_2(chars: Vec<char>) -> Vec<char> {
    let strongest_to_weakest: [char; 13] = [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];

    return _sort_chars_from_strongest_to_weakest(chars, strongest_to_weakest);
}

fn _sort_chars_from_strongest_to_weakest(
    chars: Vec<char>,
    strongest_to_weakest: [char; 13],
) -> Vec<char> {
    // As strongest_to_weakest is sorted, filterting while it's in `chars` will return a Vec with only
    // the cards in `chars` but in the same order as in strongest_to_weakest.
    return strongest_to_weakest
        .iter()
        .cloned()
        .filter(|x| chars.contains(x))
        .collect::<Vec<char>>();
}

fn count_cards(hand: &str) -> HashMap<char, i8> {
    let mut card_types: HashMap<char, i8> = HashMap::from([
        ('2', 0),
        ('3', 0),
        ('4', 0),
        ('5', 0),
        ('6', 0),
        ('7', 0),
        ('8', 0),
        ('9', 0),
        ('T', 0),
        ('J', 0),
        ('Q', 0),
        ('K', 0),
        ('A', 0),
    ]);

    for card in hand.chars() {
        card_types.entry(card).and_modify(|counter| *counter += 1);
    }

    return card_types;
}

fn get_five_of_kind_cards(hand: &HashMap<char, i8>) -> Option<char> {
    for (card, count) in hand {
        if *count == 5 {
            return Some(*card);
        }
    }

    return None;
}

fn get_four_of_kind_cards(hand: &HashMap<char, i8>) -> Option<((char, char))> {
    let mut highest_card: Option<char> = None;
    let mut lowest_card: Option<char> = None;
    for (card, count) in hand {
        if *count == 4 {
            highest_card = Some(*card);
        }
        if *count == 1 {
            lowest_card = Some(*card);
        }
    }

    if highest_card.is_some() && lowest_card.is_some() {
        return Some((highest_card.unwrap(), lowest_card.unwrap()));
    }
    return None;
}

fn get_full_house_cards(hand: &HashMap<char, i8>) -> Option<(char, char)> {
    let mut highest_card: Option<char> = None;
    let mut lowest_card: Option<char> = None;
    for (card, count) in hand {
        if *count == 3 {
            highest_card = Some(*card);
        } else if *count == 2 {
            lowest_card = Some(*card);
        }
    }

    if highest_card.is_some() && lowest_card.is_some() {
        return Some((highest_card.unwrap(), lowest_card.unwrap()));
    }
    return None;
}

fn get_three_of_kind_cards(hand: &HashMap<char, i8>) -> Option<char> {
    let mut highest_card: Option<char> = None;
    let mut lowest_card: Option<char> = None;
    for (card, count) in hand {
        if *count == 3 {
            highest_card = Some(*card);
        }
        if *count == 2 {
            lowest_card = Some(*card);
        }
    }

    if highest_card.is_some() && lowest_card.is_none() {
        return Some(highest_card.unwrap());
    }
    return None;
}

fn get_double_pair_cards(hand: &HashMap<char, i8>) -> Option<((char, char))> {
    let mut first_pair: Option<char> = None;
    let mut second_pair: Option<char> = None;
    for (card, count) in hand {
        if *count == 2 && first_pair.is_none() {
            first_pair = Some(*card);
        } else if *count == 2 && first_pair.is_some() {
            second_pair = Some(*card);
        }
    }

    if first_pair.is_some() && second_pair.is_some() {
        return Some((first_pair.unwrap(), second_pair.unwrap()));
    }
    return None;
}

fn get_single_pair_cards(hand: &HashMap<char, i8>) -> Option<(char)> {
    let mut pair: Option<char> = None;
    let mut other_pair: Option<char> = None;
    for (card, count) in hand {
        if *count == 2 && pair.is_none() {
            pair = Some(*card);
        } else if *count == 2 && pair.is_some() {
            return None; // It's not a pair, it's a double pair
        }
    }

    if pair.is_some() {
        // It's only a pair
        return Some(pair.unwrap());
    }

    // It's something else.
    return None;
}
