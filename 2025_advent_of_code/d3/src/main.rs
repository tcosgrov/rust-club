use std::env;
use std::error::Error;
use std::fs;

fn max_two_digits(string_of_digits: &str) -> Option<u8> {
    let len = string_of_digits.len();
    if len < 2 {
        return None;
    }

    let mut digits: Vec<u8> = Vec::with_capacity(len);
    for d in string_of_digits.chars() {
        let d_val = d.to_string().parse::<u8>().unwrap();
        digits.push(d_val);
    }

    let mut max_num: Option<u8> = None;
    for i in 0..(len - 1) {
        let tens = digits[i];
        let mut max_ones: u8 = 0;
        ((i + 1)..len).for_each(|j| {
            max_ones = max_ones.max(digits[j]);
        });

        let check_value = (tens * 10) + max_ones;

        max_num = match max_num {
            Some(x) => Some(x.max(check_value)),
            None => Some(check_value),
        };
    }
    max_num
}

fn sum_max_two_digits(lines: &[String]) -> u32 {
    lines
        .iter()
        // I originally had this as:
        //   .filter(|opt| opt.is_some())  // Keep only the Some(_) results
        //   .map(|opt| opt.unwrap())      // Unwrap the Some values
        // but filter_map() does both. :-)
        .filter_map(|line| max_two_digits(line))
        .map(|v| v as u32)
        .sum()
}

pub fn max_twelve_digits(string_of_digits: &str) -> Option<u128> {
    let len = string_of_digits.len();
    const REQ_SIZE: usize = 12;
    if string_of_digits.len() < REQ_SIZE {
        return None;
    }

    let mut digits: Vec<u8> = Vec::with_capacity(len);
    for d in string_of_digits.chars() {
        let d_val = d.to_string().parse::<u8>().unwrap();
        digits.push(d_val);
    }

    let mut answer: [u8; REQ_SIZE] = [0; REQ_SIZE];
    let mut start_pos = 0;
    (0..REQ_SIZE).for_each(|pos| {
        // Leave room for the remaining (MIN_SIZE - pos - 1) digits.
        //   end = n - (MIN_SIZE - pos)
        // Search inclusive range [start ..= end] for the maximum digit.
        let end = len - (REQ_SIZE - pos);
        let mut best_digit = 0u8;
        let mut best_idx = start_pos;

        (start_pos..=end).for_each(|i| {
            let digit = digits[i];
            if digit > best_digit {
                best_digit = digit;
                best_idx = i;
            }
        });

        answer[pos] = best_digit;
        start_pos = best_idx + 1;
    });

    let mut max_num: u128 = 0;
    for d in answer {
        max_num = max_num * 10 + d as u128;
    }
    Some(max_num)
}

pub fn sum_max_twelve_digits(lines: &[String]) -> u128 {
    lines
        .iter()
        .filter_map(|line| max_twelve_digits(line))
        .sum()
}

fn read_file_into_vector() -> Result<Vec<String>, Box<dyn Error>> {
    if let Some(input_file) = env::args().nth(1) {
        let contents = fs::read_to_string(input_file)?; // Can I use .lines() here?
        let lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();
        Ok(lines)
    } else {
        Err("Missing input file as first arg.".into())
    }
}

fn main() {
    let input_data = read_file_into_vector().expect("Failed to read input file");
    // Part 1: Sum of largest 2-digit number per line
    println!("\nSum (2-digits) : {}", sum_max_two_digits(&input_data));
    // Part 2: Sum of largest 12-digit number per line
    println!("Sum (12-digits): {}", sum_max_twelve_digits(&input_data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test_basic() {
        assert_eq!(max_two_digits("1"), None);
        assert_eq!(max_two_digits("00"), Some(0));
        assert_eq!(max_two_digits("12"), Some(12));
        assert_eq!(max_two_digits("21"), Some(21));
        assert_eq!(max_two_digits("212"), Some(22));
        assert_eq!(max_two_digits("2012"), Some(22));
        assert_eq!(max_two_digits("999"), Some(99));
        assert_eq!(max_two_digits("123456789"), Some(89));
        assert_eq!(max_two_digits("1234567891"), Some(91));
    }

    #[test]
    fn part_1_test_example() {
        let lines = vec![
            "987654321111111".to_string(),
            "811111111111119".to_string(),
            "234234234234278".to_string(),
            "818181911112111".to_string(),
        ];
        assert_eq!(sum_max_two_digits(&lines), 357);
    }

    #[test]
    fn part_2_test_basic() {
        assert_eq!(max_twelve_digits("12345678901"), None);
        assert_eq!(max_twelve_digits("123456789012"), Some(123456789012u128));
        assert_eq!(max_twelve_digits("999999999999"), Some(999999999999u128));
        assert_eq!(max_twelve_digits("987654321111111"), Some(987654321111u128));
        assert_eq!(max_twelve_digits("811111111111119"), Some(811111111119u128));
        assert_eq!(max_twelve_digits("1811111111111119"), Some(811111111119u128));
    }

    #[test]
    fn part_2_test_example() {
        let lines = vec![
            "987654321111111".to_string(),
            "811111111111119".to_string(),
            "234234234234278".to_string(),
            "818181911112111".to_string(),
        ];
        assert_eq!(sum_max_twelve_digits(&lines), 3121910778619u128);
    }
}
