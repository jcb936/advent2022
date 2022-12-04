struct Interval(i32, i32);

fn read_into_vector(path : &str) -> Vec<(Interval, Interval)> {
    std::fs::read_to_string(path)
        .expect("Error reading file!")
        .lines()
        .map(|x| {
            let substr : Vec<&str> = x.split(',').collect();
            let it : Vec<&str> = substr[0].split('-').collect();
            let it2 : Vec<&str> = substr[1].split('-').collect();
            (Interval(it[0].parse().unwrap(), it[1].parse().unwrap()), Interval(it2[0].parse().unwrap(), it2[1].parse().unwrap()))
        })
        .collect()
}

fn get_overlaps(vector : &Vec<(Interval, Interval)>) -> (i32, i32) {
    let is_contained = |x : &Interval, y : &Interval| x.0 >= y.0 && x.1 <= y.1; 
    let do_overlap = |x : &Interval, y : &Interval| std::cmp::min(x.1, y.1) >= std::cmp::max(x.0, y.0); 
    let mut counter_fully_contained = 0;
    let mut counter_overlaps = 0;
    for assignment in vector {
        if is_contained(&assignment.0, &assignment.1) || is_contained(&assignment.1, &assignment.0) {
            counter_fully_contained += 1;
            counter_overlaps += 1;
        } else if do_overlap(&assignment.0, &assignment.1) {
            counter_overlaps += 1;
        }
    }

    (counter_fully_contained, counter_overlaps)
}

fn main() {
    let parsed_vector = read_into_vector("input.txt");
    let results = get_overlaps(&parsed_vector);
    println!("Total part 1: {}", results.0);
    println!("Total part 2: {}", results.1);
}
