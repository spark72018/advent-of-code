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

pub fn compute_total_priority_sum<R: Read>(
    reader: &mut BufReader<R>,
    priority_map: HashMap<String, i32>,
) -> i32 {
    let mut total_priority_sum = 0;

    for line in reader.lines() {
        if line.is_ok() {
            let mut items: Vec<String> = line.unwrap().chars().map(|s| s.to_string()).collect();
            let common_letters = get_common_letters(&mut items);
            let mut current_priority_sum = 0;

            for common_letter in common_letters {
                current_priority_sum = current_priority_sum + priority_map.get(&common_letter).unwrap();
            }
            
            total_priority_sum = total_priority_sum + current_priority_sum;
        }
    }

    total_priority_sum
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
        let letter = letters[i].clone();

        if set.contains(&letter) && !already_seen.contains(&letter) {
            common_letters.push(letter);
            already_seen.insert(letters[i].clone());
        }
    }

    return common_letters;
}