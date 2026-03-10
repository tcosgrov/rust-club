use std::fs;
use std::path::PathBuf;

struct Part1Problem {
    numbers: Vec<u64>,
    operator: char,
}

struct Part2Problem2 {
    numbers: Vec<Vec<Option<u32>>>,
    operator: char,
}

fn parse_input_for_part1(input: &str) -> Vec<Part1Problem> {
    let grid: Vec<Vec<&str>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.split_whitespace().collect())
        .collect();

    if grid.is_empty() {
        return Vec::new();
    }

    let width = grid[0].len();
    let height = grid.len();

    let mut problems = Vec::new();
    for col in 0..width {
        let mut numbers = Vec::new();
        let mut operator = '+';
        for row in 0..height {
            let token = grid[row][col];
            if row == height - 1 {
                operator = token.chars().next().unwrap_or('+');
            } else if let Ok(num) = token.parse::<u64>() {
                numbers.push(num);
            }
        }
        problems.push(Part1Problem { numbers, operator });
    }
    problems
}

fn solve_part1(input: &str) -> u64 {
    let problems = parse_input_for_part1(input);
    problems
        .iter()
        .map(|p| match p.operator {
            '+' => p.numbers.iter().sum::<u64>(),
            '*' => p.numbers.iter().product::<u64>(),
            _ => 0,
        })
        .sum()
}

fn extract_part2_problem(
    grid: &[Vec<char>],
    operator_line: &[char],
    start_col: usize,
    end_col: usize,
) -> Part2Problem2 {
    let numbers = grid
        .iter()
        .map(|row| {
            row[start_col..end_col]
                .iter()
                .map(|&c| c.to_digit(10))
                .collect()
        })
        .collect();
    let operator = operator_line[start_col..end_col]
        .iter()
        .find(|&&c| c != ' ')
        .copied()
        .unwrap_or('+');
    Part2Problem2 { numbers, operator }
}

fn parse_input_for_part2(input: &str) -> Vec<Part2Problem2> {
    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let mut grid: Vec<Vec<char>> = lines
        .iter()
        .map(|line| {
            let mut chars: Vec<char> = line.chars().collect();
            chars.resize(max_width, ' ');
            chars.reverse();
            chars
        })
        .collect();

    let operator_line = grid.pop().unwrap();
    let mut problems = Vec::new();
    let mut start_col = 0;

    for col in 0..max_width {
        let is_separator = operator_line[col] == ' ' && grid.iter().all(|row| row[col] == ' ');
        if is_separator {
            if col > start_col {
                problems.push(extract_part2_problem(&grid, &operator_line, start_col, col));
            }
            start_col = col + 1;
        }
    }

    // Handle the final problem if the grid doesn't end with a separator column.
    if start_col < max_width {
        problems.push(extract_part2_problem(&grid, &operator_line, start_col, max_width));
    }

    problems
}

fn solve_part2(input: &str) -> u64 {
    let problems = parse_input_for_part2(input);
    let mut grand_total = 0;

    for problem in problems {
        let mut current_numbers: Vec<u64> = Vec::new();
        let width = problem.numbers[0].len();

        for col in 0..width {
            let mut num: u64 = 0;
            for row in &problem.numbers {
                if let Some(digit) = row[col] {
                    num = num * 10 + (digit as u64);
                }
            }
            current_numbers.push(num);
        }

        match problem.operator {
            '+' => grand_total += current_numbers.iter().sum::<u64>(),
            '*' => grand_total += current_numbers.iter().product::<u64>(),
            _ => {}
        }
    }

    grand_total
}

fn main() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("input.txt");
    let input = fs::read_to_string(&input_path).expect("Could not read input.txt");

    println!(
        "Part 1: Grand total: {}",
        solve_part1(&input)
    );
    println!(
        "Part 2: Grand total: {}",
        solve_part2(&input)
    );
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2};

    const SIMPLE_INPUT: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(SIMPLE_INPUT), 4277556);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(SIMPLE_INPUT), 3263827);
    }
}
