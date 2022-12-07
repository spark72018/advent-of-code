use std::io::{prelude::*, BufReader, Seek, SeekFrom};
use std::result::Result::Ok;

#[derive(Debug, Clone, Copy)]
pub struct InputRows {
    pub directions_start_line: u32,
    pub num_stacks: u32,
    pub row_line: u32
}

#[derive(Debug)]
struct Procedure {
    num_of_crates_to_move: u32,
    from_stack: usize,
    to_stack: usize
}

pub fn perform_procedures<R: Read + Seek>(reader: &mut BufReader<R>, input_rows: InputRows) -> String {
    let mut stacks: Vec<Vec<String>> = Vec::new();

    for _i in 0..input_rows.num_stacks {
        stacks.push(Vec::new());
    }

    let lines = reader.lines();

    initialize_stacks(reader, &input_rows, &mut stacks);
    
    reader.seek(SeekFrom::Start(0));

    let mut line_number = 0;
    for line in reader.lines() {
        line_number += 1;
        if let Ok(line_val) = line {
            if line_number > (input_rows.row_line + 1) {
                let procedure = get_procedure(line_val);

                for _i in 0..procedure.num_of_crates_to_move {
                    if let Some(crate_to_move) = stacks[procedure.from_stack].pop() {
                        stacks[procedure.to_stack].push(crate_to_move);
                    }
                }
            }
        }
    }
    
    let mut result = Vec::new();

    for i in 0..stacks.iter().len() {
        if let Some(last_val) = stacks[i].pop() {
            result.push(last_val);
        }
    }
    
    result.join("")
}

fn get_procedure(str: String) -> Procedure {
    let split: Vec<&str> = str.split(" ").collect();
    let (num_of_crates_to_move, from_stack, to_stack) = match (split[1].parse::<u32>(), split[3].parse::<usize>(), split[5].parse::<usize>()) {
        (Ok(val_1), Ok(val_2), Ok(val_3)) => (val_1, val_2 - 1, val_3 - 1),
        (_, _, _) => {(0, 0, 0)}
    };
    
    Procedure {
        num_of_crates_to_move,
        from_stack,
        to_stack
    }
}

pub fn perform_procedures_two<R: Read + Seek>(reader: &mut BufReader<R>, input_rows: InputRows) -> String {
    // reset previous use of the reader
    reader.seek(SeekFrom::Start(0));

    let mut stacks: Vec<Vec<String>> = Vec::new();

    for _i in 0..input_rows.num_stacks {
        stacks.push(Vec::new());
    }

    let lines = reader.lines();

    initialize_stacks(reader, &input_rows, &mut stacks);
    
    reader.seek(SeekFrom::Start(0));

    let mut line_number = 0;
    for line in reader.lines() {
        line_number += 1;
        if let Ok(line_val) = line {
            if line_number > (input_rows.row_line + 1) {
                let procedure = get_procedure(line_val);
                let mut temp_stack = Vec::new();

                for _i in 0..procedure.num_of_crates_to_move {
                    if let Some(crate_to_move) = stacks[procedure.from_stack].pop() {
                        temp_stack.push(crate_to_move);
                    }
                }
                temp_stack.reverse();
                for val in temp_stack {
                    stacks[procedure.to_stack].push(val);
                }
            }
        }
    }
    
    let mut result = Vec::new();

    for i in 0..stacks.iter().len() {
        if let Some(last_val) = stacks[i].pop() {
            result.push(last_val);
        }
    }
    
    result.join("")
}

fn extract_letter_from_input(str: &str) -> String {
    str.chars().nth(1).unwrap().into()
}

fn initialize_stacks<R: Read + Seek>(reader: &mut BufReader<R>, input_rows: &InputRows, stacks: &mut Vec<Vec<String>>) {
    let mut line_number = 0;
    let mut should_exit: bool = false;
    for line in reader.lines() {
        if let Ok(line_val) = line {
            line_number += 1;
            
            if line_number < input_rows.directions_start_line {
                for (i, c) in line_val.chars().enumerate() {
                    if c.is_ascii_alphabetic() {
                        let stack_idx = i/4;
                        stacks[stack_idx].push(c.to_string());
                    }
                }
            }
        }
    }
    
    for stack in stacks {
        stack.reverse();
    }
}