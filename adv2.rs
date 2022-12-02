fn play_get_total_part_1(rounds : &Vec<(i32, i32)>) -> i32 {
    let mut total : i32 = 0;
    for i in rounds {
        total += (i.1 + 1);
        if i.0 == i.1 {
            total += 3; // we draw
        } else if i.1 == ((i.0 + 1) % 3) {
            total += 6; // we win
        }
    }

    total
}

fn play_get_total_part_2(rounds : &Vec<(i32, i32)>) -> i32 {
    let mut total : i32 = 0;
    for i in rounds {
        match i.1 {
            // rust doesn't have a ternary operator :(
            0 => total += if i.0 - 1 < 0 {2} else {(i.0 - 1) % 3}, // we lose
            1 => total += i.0 + 3, // we draw
            2 => total += ((i.0 + 1) % 3) + 6, // we win
            _ => {}
        }
        total += 1;
    }

    total
}

fn read_into_vector(path : &str) -> Vec<(i32, i32)> {
    let as_score = |x : char| {
        match x {
            'A' | 'X' => 0,
            'B' | 'Y' => 1,
            'C' | 'Z' => 2,
            _ => 0
        }
    };

    std::fs::read_to_string(path)
        .expect("Error reading file!")
        .lines()
        .map(|x| (as_score(x.chars().nth(0).unwrap()), as_score(x.chars().nth(2).unwrap())))
        .collect()
}

fn main() {
    let vec = read_into_vector("input.txt");
    println!("total is {}", play_get_total_part_1(&vec));
    println!("total part 2 is {}", play_get_total_part_2(&vec));
}
