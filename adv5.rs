#[derive(Copy, Clone)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize
}

fn read_into_vector(path : &str) -> Vec<Instruction> {
    std::fs::read_to_string(path)
        .expect("Error reading file!")
        .lines()
        .map(|x| {
            let numbers: Vec<&str> = x.split(' ').map(|y| y.trim()).collect();
            let mut result_array: [usize; 3] = [0; 3];
            let mut counter: usize = 0;
            for string in numbers {
                if let Ok(x) = string.parse::<usize>() {
                    result_array[counter] = x;
                    counter += 1;
                }
            }
            Instruction { amount: result_array[0], from: result_array[1], to: result_array[2] }
        })
        .collect()
}

fn generate_initial_stacks_state() -> [Vec<char>; 9] {
    let result_array: [Vec<char>; 9] = [
        vec!['C', 'Z', 'N', 'B', 'M', 'W', 'Q', 'V'],
        vec!['H', 'Z', 'R', 'W', 'C', 'B'],
        vec!['F', 'Q', 'R', 'J'],
        vec!['Z', 'S', 'W', 'H', 'F', 'N', 'M', 'T'],
        vec!['G', 'F', 'W', 'L', 'N', 'Q', 'P'],
        vec!['L', 'P', 'W'],
        vec!['V', 'B', 'D', 'R', 'G', 'C', 'Q', 'J'],
        vec!['Z', 'Q', 'N', 'B', 'W'],
        vec!['H', 'L', 'F', 'C', 'G', 'T', 'J']
    ];

    result_array
}

fn move_stuff_read_result(stacks: &mut [Vec<char>; 9], instructions: &Vec<Instruction>) -> String {
    for inst in instructions {
        for _ in 0..inst.amount {
            let char_to_move = stacks[inst.from - 1].pop().unwrap();
            stacks[inst.to - 1].push(char_to_move);
        }
    }

    let mut result = String::from("");
    for stack in stacks {
        result.push(stack[stack.len() - 1]);
    }

    result
}

fn move_stuff_read_result_part_2(stacks: &mut [Vec<char>; 9], instructions: &Vec<Instruction>) -> String {
    for inst in instructions {
        let len = stacks[inst.from - 1].len();
        let mut drained_vec: Vec<char> = stacks[inst.from - 1].drain((len - inst.amount)..len).collect();
        stacks[inst.to - 1].append(&mut drained_vec);
    }

    let mut result = String::from("");
    for stack in stacks {
        result.push(stack[stack.len() - 1]);
    }

    result
}

fn main() {
    let mut stacks = generate_initial_stacks_state();
    let mut stacks_copy = stacks.clone();
    let instructions = read_into_vector("input.txt");
    println!("Result part 1: {}", move_stuff_read_result(&mut stacks, &instructions));
    println!("Result part 2: {}", move_stuff_read_result_part_2(&mut stacks_copy, &instructions));
}
