use std::collections::HashMap;
use std::io::{prelude::*, BufReader};

use crate::types::{Choice, Hand, Result};

pub fn get_hand_points_map() -> HashMap<Hand, i32> {
    let mut hand_points = HashMap::new();

    hand_points.insert(Hand::Rock, 1);
    hand_points.insert(Hand::Paper, 2);
    hand_points.insert(Hand::Scissors, 3);
    hand_points.insert(Hand::Empty, 0);

    hand_points
}

pub fn get_result_points_map() -> HashMap<Result, i32> {
    let mut result_points = HashMap::new();

    result_points.insert(Result::Lose, 0);
    result_points.insert(Result::Draw, 3);
    result_points.insert(Result::Win, 6);

    result_points
}

pub fn get_result_part_two(str: &str) -> Result {
    match str {
        "X" => Result::Lose,
        "Y" => Result::Draw,
        "Z" => Result::Win,
        _ => Result::Draw
    }
}

pub fn get_hand(choice: &str) -> Choice {
    match choice {
        "A" | "X" => Choice { choice: Hand::Rock },
        "B" | "Y" => Choice { choice: Hand::Paper },
        "C" | "Z" => Choice { choice: Hand::Scissors },
        _ => Choice { choice: Hand::Empty }
    }
}

pub fn compute_total_points<R: Read>(
    reader: &mut BufReader<R>,
    hand_points_map: HashMap<Hand, i32>,
    result_points_map: HashMap<Result, i32>
) -> (i32, i32) {
    let mut total_points_part_one: i32 = 0;
    let mut total_points_part_two: i32 = 0;

    for line in reader.lines() {
        if line.is_ok() {
            let input_string = line.unwrap();
            let split: Vec<&str> = input_string.split(" ").collect();
            // part one
            let computed_points_for_line_part_one = compute_line_points_part_one(
                split[0], 
                split[1],
                &hand_points_map,
                &result_points_map
            );

            // part two
            let computed_points_for_line_part_two = compute_line_points_part_two(
                split[0], 
                split[1],
                &hand_points_map,
                &result_points_map
            );

            total_points_part_one = total_points_part_one + computed_points_for_line_part_one;
            total_points_part_two = total_points_part_two + computed_points_for_line_part_two;
        }
    }

    (total_points_part_one, total_points_part_two)
}

fn compute_line_points_part_one(
    elf_hand_input: &str, 
    my_hand_input: &str,
    hand_points_map: &HashMap<Hand, i32>,
    result_points_map: &HashMap<Result, i32>
) -> i32 {
    let (elf_hand, my_hand) = (get_hand(elf_hand_input).choice, get_hand(my_hand_input).choice);
    let result: Result = elf_hand.play_against(my_hand);
    let hand_points = hand_points_map.get(&my_hand).unwrap();
    let result_points = result_points_map.get(&result).unwrap();

    hand_points + result_points
}

fn compute_line_points_part_two(
    elf_hand_input: &str,
    expected_result: &str,
    hand_points_map: &HashMap<Hand, i32>,
    result_points_map: &HashMap<Result, i32>
) -> i32 {
        let (elf_hand, expected_result) = (get_hand(elf_hand_input).choice, get_result_part_two(expected_result));
        let my_hand = match expected_result {
            Result::Lose => match elf_hand {
                Hand::Rock => Hand::Scissors,
                Hand::Paper => Hand::Rock,
                Hand::Scissors => Hand::Paper,
                Hand::Empty => Hand::Empty
            },
            Result::Win => match elf_hand {
                Hand::Rock => Hand::Paper,
                Hand::Paper => Hand::Scissors,
                Hand::Scissors => Hand::Rock,
                Hand::Empty => Hand::Empty
            },
            Result::Draw => elf_hand
        };

        let hand_points = hand_points_map.get(&my_hand).unwrap();
        let result_points = result_points_map.get(&expected_result).unwrap();

        hand_points + result_points
}