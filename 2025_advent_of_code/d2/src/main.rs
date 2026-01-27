use std::env;
use std::fs;

fn part_one() {
    let mut sum_of_invalid_ids = 0;
    let input_file = env::args().nth(1).unwrap();
    let input_data = fs::read_to_string(input_file).unwrap();
    let input_parts: Vec<&str> = input_data.split(',').collect();

    for raw_line in input_parts {
        let trimmed_line = raw_line.trim();
        if trimmed_line.is_empty() {
            continue;
        }

        let (min_str, max_str) = trimmed_line.split_once('-').unwrap();
        let min: usize = min_str.parse().unwrap();
        let max: usize = max_str.parse().unwrap();
        for num in min..=max {
            let num_string = num.to_string();
            if (num_string.len() % 2) == 1 {
                continue;
            }
            let (first_half, second_half) = num_string.split_at(num_string.len() / 2);
            if first_half == second_half {
                sum_of_invalid_ids += num;
            }
        }
    }

    println!("\nPart  I: sum of invalid ids: {}", sum_of_invalid_ids);
}

fn is_all_same(test_string: &str) -> bool {
    let mut string_chars = test_string.chars();
    let first_char = string_chars.next().unwrap();
    string_chars.all(|c| c == first_char)
}

fn part_two() {
    let mut sum_of_invalid_ids = 0;
    let input_file = env::args().nth(1).unwrap();
    let input_data = fs::read_to_string(input_file).unwrap();
    let input_parts: Vec<&str> = input_data.split(',').collect();

    for raw_line in input_parts {
        let trimmed_line = raw_line.trim();
        if trimmed_line.is_empty() {
            continue;
        }

        let (min_str, max_str) = raw_line.split_once('-').unwrap();
        let min: usize = min_str.parse().unwrap();
        let max: usize = max_str.parse().unwrap();
        for num in min..=max {
            if num < 10 {
                continue;
            }

            let num_string = num.to_string();
            if is_all_same(&num_string) {
                sum_of_invalid_ids += num;
                continue;
            }

            let max_pattern_len = num_string.len() / 2;
            for pattern_len in 2..=max_pattern_len {
                if num_string.len() % pattern_len != 0 {
                    continue;
                }

                let pattern = &num_string[0..pattern_len];
                let mut is_match = true;
                for offset in (pattern_len..=(num_string.len() - pattern_len)).step_by(pattern_len)
                {
                    let current_chunk = &num_string[offset..offset + pattern_len];
                    if current_chunk != pattern {
                        is_match = false;
                        break;
                    }
                }

                if is_match {
                    sum_of_invalid_ids += num;
                    break; //  >:/
                }
            }
        }
    }
    println!("Part II: sum of invalid ids: {}", sum_of_invalid_ids);
}

fn main() {
    part_one();
    part_two();
}
