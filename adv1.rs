use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::BinaryHeap;

fn read_into_vector(path : &str) -> Vec<String> {
    std::fs::read_to_string(path)
        .expect("Error reading file!")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn get_sum_heap(vector : &Vec<String>) -> BinaryHeap<i32> {
    let mut temp_max = 0;
    let mut heap_result = BinaryHeap::new();
    for i in vector {
        if let Ok(parsed) = i.parse::<i32>() {
            temp_max += parsed;
        } 
        else {
            heap_result.push(temp_max);
            temp_max = 0;
        }
    }

    heap_result
}

fn main() {
    let path = "./input.txt";
    let string_list = read_into_vector(path);
    let mut sum_heap = get_sum_heap(&string_list);
    println!("Result part 1: {}", sum_heap.peek().unwrap());
    println!("Result part 2: {}", 
        sum_heap.pop().unwrap() + 
        sum_heap.pop().unwrap() + 
        sum_heap.pop().unwrap());
}
