use std::fs;
use std::collections::HashSet;

const START_OF_PACKET_SIZE: usize = 4;
const START_OF_MESSAGE_SIZE: usize = 14;

fn get_signal_index(input: &str, size: usize) -> usize {
    let mut result = 0;
    let mut char_set: HashSet<u8> = HashSet::with_capacity(START_OF_MESSAGE_SIZE);

    for n in (size - 1)..(input.len()) {
        for i in (n + 1 - size)..(n + 1) {
            char_set.insert(input.as_bytes()[i]);
        }

        if char_set.len() == size {
            result = n + 1;
            break;
        }

        char_set.clear();
    }

    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file!");
    println!("Index of start of signal: {}. Index of start of message: {}", 
        get_signal_index(&input, START_OF_PACKET_SIZE),
        get_signal_index(&input, START_OF_MESSAGE_SIZE));
}
