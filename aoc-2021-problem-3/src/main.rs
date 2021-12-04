fn main() {
    part_a(include_str!("input.txt"));
    part_b(include_str!("input.txt"));
}

fn part_a(input: &str) {
    let mut epsilon_string: String = String::new();
    let mut gamma_string: String = String::new();

    for i in 0..12 {
        let mut bit_one = 0;
        let mut bit_zero = 0;

        for line in input.lines() {
            let bit = line.chars().nth(i).unwrap();

            if bit == '1' {
                bit_one += 1;
            } else {
                bit_zero += 1;
            }
        }

        if bit_one > bit_zero {
            gamma_string = [gamma_string, "1".to_string()].join("");
            epsilon_string = [epsilon_string, "0".to_string()].join("");
        } else {
            gamma_string = [gamma_string, "0".to_string()].join("");
            epsilon_string = [epsilon_string, "1".to_string()].join("");
        }
    }

    let gamma = i32::from_str_radix(&gamma_string[..], 2).unwrap();
    let epsilon = i32::from_str_radix(&epsilon_string[..], 2).unwrap();

    println!("Part A: {}", gamma * epsilon);
}

fn part_b(input: &str) {
    let ocr = part_b_helper(input,'1', '0');
    let csr = part_b_helper(input, '0', '1');

    println!("Part B:{}", ocr*csr);
}

fn part_b_helper(input: &str ,bit_one_p: char, bit_zero_p: char) -> i64 {
    let mut bits_strings = vec![];

    for line in input.lines() {
        bits_strings.push(line);
    }

    for i in 0..12 {
        if bits_strings.iter().count() == 1 {
            continue;
        }

        let mut bit_one = 0;
        let mut bit_zero = 0;
        let mut bit: char = '1';

        let tmp_bit_string = bits_strings.to_vec();

        for line in tmp_bit_string {
            let bit = line.chars().nth(i).unwrap();

            if bit == '1' {
                bit_one += 1;
            } else {
                bit_zero += 1;
            }
        }

        if bit_one > bit_zero {
            bit = bit_one_p
        } else if bit_one == bit_zero {
            bit = bit_one_p
        } else {
            bit = bit_zero_p 
        }

        let mut new_bits_strings = vec![];

        for str_bit in bits_strings {
            if str_bit.chars().nth(i).unwrap() == bit {
                new_bits_strings.push(str_bit)
            }
        }

        bits_strings = new_bits_strings;
    }

    return i64::from_str_radix(bits_strings.first().unwrap(), 2).unwrap();
}
