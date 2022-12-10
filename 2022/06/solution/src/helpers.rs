use std::collections::HashSet;
use std::convert::TryInto;
#[derive(Debug)]
struct Window {
    start: usize,
    end: usize
}

impl Window {
    fn size(&self) -> usize {
        &self.end - &self.start
    }
}

pub fn find_first_marker(input_str: &String, input_window_size: usize) -> i32 {
    let mut window = Window { start: 0, end: 0, };
    let mut set = HashSet::new();
    let chars_iter = input_str.chars();
    let converted_chars: Vec<String> = 
        chars_iter
            .clone()
            .map(|cc| cc.to_string())
            .collect();

    for c in &converted_chars {
        if window.size() < input_window_size {
            // expand window
            if !set.contains(&c) {
                set.insert(c);
                window.end += 1;
            } else {
                // contract window
                while set.contains(&c) {
                    set.remove(&converted_chars[window.start]);
                    window.start += 1;
                }
                set.insert(c);
                window.end += 1;
            }
        } else if window.size() == input_window_size {
            return (window.end).try_into().unwrap();
        }
    }
    
    (window.end).try_into().unwrap()
}