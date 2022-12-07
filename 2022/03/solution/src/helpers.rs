use std::collections::{HashMap, HashSet};
use std::io::{prelude::*, BufReader};
use std::string::String;

pub fn get_priority_map() -> HashMap<String, i32> {
    let uppercase_offset = 26;
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let mut map = HashMap::new();

    for (idx, lowercase_char) in alphabet.chars().enumerate() {
        let lowercase_char_priority = (idx + 1) as i32;
        let uppercase_char = String::from(lowercase_char).to_uppercase();
        let uppercase_char_priority = lowercase_char_priority + uppercase_offset;

        map.insert(String::from(lowercase_char), lowercase_char_priority);
        map.insert(uppercase_char, uppercase_char_priority);
    }

    map
}

pub fn compute_total_priority_sum_part_one<R: Read>(
    reader: &mut BufReader<R>,
    priority_map: HashMap<String, i32>,
) -> i32 {
    let mut total_priority_sum = 0;
    let mut line_count = 0;

    for line in reader.lines() {
        line_count += 1;
        if line.is_ok() {
            let mut items: Vec<String> = line.unwrap().chars().map(|s| s.to_string()).collect();
            let common_letters = get_common_letters(&mut items);
            let mut current_priority_sum = 0;

            for common_letter in common_letters {
                current_priority_sum += priority_map.get(&common_letter).unwrap();
            }
            
            total_priority_sum += current_priority_sum;
        }
    }

    total_priority_sum
}

pub fn compute_total_priority_sum_part_two<R: Read>(
    reader: &mut BufReader<R>,
    priority_map: HashMap<String, i32>,
) -> i32 {
    let mut total_priority_sum = 0;
    let mut line_count = 0;
    let mut badges: Vec<String> = Vec::new();
    let mut sets: Vec<HashSet<String>> = Vec::new();

    for line in reader.lines() {
        if line.is_ok() {
            line_count += 1;
            let mut items: Vec<String> = line.unwrap().chars().map(|s| s.to_string()).collect();
            
            if line_count == 3 {
                let mut final_item_set: HashSet<String> = HashSet::new();

                for item in items {
                    final_item_set.insert(item);
                }

                let (first_set, second_set) = (&sets[0], &sets[1]);
                let first_second_intersection: HashSet<_> = first_set.intersection(&second_set).collect();
                let mut intersected_set: HashSet<String> = HashSet::new();

                for intersected_value in first_second_intersection {
                    intersected_set.insert(intersected_value.to_string());
                }

                let final_intersection: HashSet<_> = intersected_set.intersection(&final_item_set).collect();
                let mut answer: Vec<&String> = Vec::new();

                for val in final_intersection {
                    answer.push(&val);
                }

                badges.push(answer[0].to_string());

                sets.clear();
                line_count = 0;
            } else {
                let mut item_set = HashSet::new();

                for item in items {
                    item_set.insert(item);
                }

                sets.push(item_set);
            }
        }
    }

    badges
        .iter()
        .map(|badge| priority_map.get(badge).unwrap())
        .sum()
}

fn get_common_letters(letters: &mut Vec<String>) -> Vec<String> {
    let mut common_letters: Vec<String> = Vec::new();
    let letters_vec_length = letters.len();
    let half_point = letters_vec_length / 2;
    let mut set = HashSet::new();
    let mut already_seen = HashSet::new();

    // iterate over first half of input vec and insert into set
    for i in 0 ..half_point {
        set.insert(letters[i].clone());
    }

    for i in half_point..letters_vec_length {
        let letter = &letters[i].clone();

        if set.contains(letter) && !already_seen.contains(letter) {
            common_letters.push(letter.to_string());
            already_seen.insert(letter.to_string());
        }
    }

    return common_letters;
}