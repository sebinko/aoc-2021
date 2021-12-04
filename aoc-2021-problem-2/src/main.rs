fn main() {
    part_a(include_str!("in.txt"));
    part_b(include_str!("in.txt"))
}

fn part_a(input: &str) {
    let mut x: i64 = 0;
    let mut y: i64 = 0;

    for line in input.lines() {
        let mut parsed_line = line.split(' ');
        let dir = parsed_line.next().unwrap();
        let amount: i64 = parsed_line.next().unwrap().parse().unwrap();

        match dir {
            "forward" => {
                x += amount;
            }
            "up" => {
                y -= amount;
            }
            "down" => {
                y += amount;
            }
            _ => {}
        }
    }

    println!("Part A: {} - x: {} ; y: {}", x * y, x, y)
}

fn part_b(input: &str) {
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut aim: i64 = 0;

    for line in input.lines() {
        let mut parsed_line = line.split(' ');
        let dir = parsed_line.next().unwrap();
        let amount: i64 = parsed_line.next().unwrap().parse().unwrap();

        match dir {
            "up" => {
                aim -= amount;
            }
            "down" => aim += amount,
            "forward" => {
                x += amount;
                y += aim * amount;
            }
            _ => {}
        }
    }

    println!("Part B: {} - x: {} ; y: {} ; aim: {}", x * y, x, y, aim)
}
