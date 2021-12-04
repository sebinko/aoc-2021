fn main() {
    println!("Part 1: {}", part_1(include_str!("input.txt")));
    println!("Part 2: {}", part_b(include_str!("input.txt")));
}

fn part_1(input: &str) -> i64 {
    let depths: Vec<i64> = input
        .lines()
        .map(|i| i.parse::<i64>().unwrap())
        .collect();

    let mut i = 0;

    for depth_window in depths.windows(2) {
        if depth_window[1] > depth_window[0] {
            i += 1;
        }
    }

    i
}

fn part_b(input: &str) -> i64 {
    let depths: Vec<i64> = input
        .lines()
        .map(|i| i.parse::<i64>().unwrap())
        .collect();

    let mut i = 0;

    for depths_window in depths.windows(4) {
        if depths_window[3] > depths_window[0] {
            i += 1;
        }
    }
    
    i
}