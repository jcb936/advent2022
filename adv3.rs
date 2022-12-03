use std::collections::HashMap;

fn read_into_vector(path : &str) -> Vec<String> {
    std::fs::read_to_string(path)
        .expect("Error reading file!")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn get_char_priority(c : char) -> u8 {
    if c.is_ascii_uppercase() {(c as u8) - ('A' as u8) + 27} else {(c as u8) - ('a' as u8) + 1}
}

fn find_priority_sum(list : &Vec<String>) -> u32 {
    let mut char_map : HashMap<char, u32> = HashMap::new();
    let mut priority_sum = 0;

    for str in list {
        let sub_1 = &str[..(str.len() / 2)]; // we assume even number in length, also we assume ASCII chars
        let sub_2 = &str[(str.len() / 2)..];

        for c in sub_1.chars() {
            *char_map.entry(c).or_insert(0) += 1;
        }

        for c in sub_2.chars() {
            if char_map.contains_key(&c) {
                priority_sum += get_char_priority(c) as u32;
                char_map.clear();
                break;
            }
        }
    }

    priority_sum
}

fn find_priority_sum_part_two(list : &Vec<String>) -> u32 {
    let mut char_map_str_1 : HashMap<char, u32> = HashMap::new();
    let mut char_map_str_2 : HashMap<char, u32> = HashMap::new();
    let mut priority_sum = 0;
    let mut counter = 0;

    for str in list {
        for c in str.chars() {
            if counter == 0 {
                *char_map_str_1.entry(c).or_insert(0) += 1;
            } else if counter == 1 {
                *char_map_str_2.entry(c).or_insert(0) += 1;
            } else if char_map_str_1.contains_key(&c) && char_map_str_2.contains_key(&c) { // we assume a common character exists to reset the maps here
                priority_sum += get_char_priority(c) as u32;
                char_map_str_1.clear();
                char_map_str_2.clear();
                break;
            }
        }

        counter = (counter + 1) % 3;
    }

    priority_sum
}

fn main() {
    let string_vec = read_into_vector("input.txt");
    println!("Priority total is {}", find_priority_sum(&string_vec));
    println!("Priority total part 2 is {}", find_priority_sum_part_two(&string_vec));
}
