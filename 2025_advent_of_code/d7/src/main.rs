use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

fn solve_part1(input: &str) -> u64 {
    let diagram_grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let beam_start_col = diagram_grid[0].iter().position(|&c| c == 'S').unwrap();

    let mut beams = HashSet::new();
    beams.insert(beam_start_col);

    let mut splits = 0;
    for row in 1..diagram_grid.len() {
        let mut next_row_beams = HashSet::new();
        for &beam_col in &beams {
            match diagram_grid[row][beam_col] {
                '^' => {
                    // splitter, add 2 beams on next row
                    next_row_beams.insert(beam_col - 1); // left
                    next_row_beams.insert(beam_col + 1); // right
                    splits += 1;
                }
                '.' => {
                    // empty space, continues down
                    next_row_beams.insert(beam_col);
                }
                _ => {}
            }
        }
        if next_row_beams.is_empty() {
            break;
        }
        beams = next_row_beams;
    }
    splits
}

fn solve_part2(input: &str) -> u64 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start_col = grid[0].iter().position(|&c| c == 'S').unwrap();
    let mut positions_store: HashMap<(usize, usize), u64> = HashMap::new();
    count_timelines_recursively(1, start_col, &grid, &mut positions_store)
}

fn count_timelines_recursively(
    row: usize,
    col: usize,
    grid: &Vec<Vec<char>>,
    positions_store: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    // already processed this one => return stored value.
    if let Some(&result) = positions_store.get(&(row, col)) {
        return result;
    }
    // moved past the last row => done with timeline.
    if row >= grid.len() {
        return 1;
    }

    let result = match grid[row][col] {
        '^' => { // splitter
            let left_timelines = count_timelines_recursively(row + 1, col - 1, grid, positions_store);
            let right_timelines = count_timelines_recursively(row + 1, col + 1, grid, positions_store);
            left_timelines + right_timelines
        }
        _ => {
            // empty space (continue straight down)
            count_timelines_recursively(row + 1, col, grid, positions_store)
        }
    };

    // store and return
    positions_store.insert((row, col), result);
    result
}

fn main() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("input.txt");
    let input = fs::read_to_string(&input_path).expect("Could not read input.txt");

    println!("\nPart 1: Splits   : {}", solve_part1(&input));
    println!("Part 2: Timelines: {}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2};

    const EXAMPLE_INPUT: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 21);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE_INPUT), 40);
    }
}
