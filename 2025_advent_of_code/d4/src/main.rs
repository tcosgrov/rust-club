use std::fs;
use std::path::PathBuf;

const NEIGHBORS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

/// Parses the input string into a 2D grid of bytes.
///
/// # Arguments
/// * `input` - The input string representing the grid.
///
/// # Returns
/// A vector of vectors containing the grid data as bytes.
fn parse_grid(input: &str) -> Vec<Vec<u8>> {
    input
        .lines() // Split the input string into an iterator of lines
        .map(|line| line.trim().bytes().collect()) // Convert each line into a vector of bytes
        .collect() // Collect the results into a vector of vectors
}

/// Counts the accessible rolls of paper in a specific row based on its neighbors.
///
/// # Arguments
/// * `prev` - The previous row (if any).
/// * `curr` - The current row being processed.
/// * `next` - The next row (if any).
/// * `row_index` - The index of the current row in the overall grid.
///
/// # Notes
/// A slice &[u8] allows the caller to pass anything that is a contiguous sequence of bytes.
///
/// # Returns
/// A vector of (row, col) coordinates for accessible rolls in this row.
fn count_accessible_in_row(
    prev: Option<&[u8]>,
    curr: &[u8],
    next: Option<&[u8]>,
    row_index: usize,
) -> Vec<(usize, usize)> {
    curr.iter()
        .enumerate()
        // Filter to keep only cells that contain a roll of paper ('@').
        .filter(|(_, &cell)| cell == b'@')
        // Map each valid cell to its coordinates if it meets the accessibility criteria.
        // The tuple (c, _) comes from enumerate(): 'c' is the column index,
        // and '_' is the cell value (which is ignored here as we know it is '@').
        .filter_map(|(c, _)| {
            // Count how many neighbors are also rolls of paper.
            let neighbor_count = NEIGHBORS
                .iter()
                // The pattern &&(dr, dc) handles the double indirection: iter() yields references to tuples,
                // and filter() passes a reference to that item.
                // 'dr' is the row offset (delta row) and 'dc' is the column offset (delta col).
                .filter(|&&(dr, dc)| {
                    // Determine which row slice to look at based on the vertical offset.
                    let target_row = match dr {
                        -1 => prev,
                        0 => Some(curr),
                        1 => next,
                        _ => None,
                    };
                    // Check if the neighbor exists within the grid bounds and is a roll of paper.
                    // is_some_and is used here to verify that target_row is Some AND that the
                    // condition inside the closure is met. This cleanly handles the case where
                    // target_row is None (e.g., at the top or bottom edge) by returning false.
                    target_row.is_some_and(|row| {
                        let nc = c as isize + dc;
                        nc >= 0 && row.get(nc as usize) == Some(&b'@')
                    })
                })
                .count();

            // If fewer than 4 neighbors are rolls, this roll is accessible.
            if neighbor_count < 4 {
                Some((row_index, c))
            } else {
                None
            }
        })
        .collect()
}

/// Day 4 / Part 1: Count accessible rolls of paper.
///
/// A roll of paper ('@') is accessible if there are fewer than four other
/// rolls of paper in its eight adjacent positions (including diagonals).
///
/// # Arguments
/// * `input` - A string slice containing the grid representation.
///
/// # Returns
/// The total count of accessible rolls of paper.
fn solve_part1(input: &str) -> u32 {
    let grid = parse_grid(input);
    let rows = grid.len();
    // Iterate through each row of the grid to check for accessible rolls.
    (0..rows)
        .map(|row| {
            // Determine the previous row slice, if it exists.
            let prev = if row > 0 { Some(&grid[row - 1]) } else { None };
            // Get the current row slice.
            let curr = &grid[row];
            // Determine the next row slice, if it exists.
            let next = if row < rows - 1 {
                Some(&grid[row + 1])
            } else {
                None
            };

            // Count accessible rolls in the current row using the sliding window of rows.
            count_accessible_in_row(
                prev.map(|v| v.as_slice()),
                curr,
                next.map(|v| v.as_slice()),
                row,
            )
            .len()
        })
        .sum::<usize>() as u32
}

/// Day 4 / Part 2: Calculate the total number of rolls of paper
/// that could be removed by iteratively finding and removing accessible rolls.
///
/// Once a roll is removed, it might change the accessibility of its neighbors,
/// potentially making new rolls accessible. The process repeats until no more
/// rolls can be accessed.
///
/// # Arguments
/// * `input` - A string slice containing the initial grid representation.
///
/// # Returns
/// The total number of rolls of paper removed.
fn solve_part2(input: &str) -> u32 {
    let mut grid = parse_grid(input);
    let rows = grid.len();
    let mut total_removed_rolls = 0;

    // Continue the process until no more rolls can be removed.
    loop {
        // Iterate through the grid to find all currently accessible rolls.
        let to_remove: Vec<_> = (0..rows)
            // flat_map is used here because count_accessible_in_row returns a Vec of coordinates.
            // We want to flatten these individual vectors into a single iterator of coordinates
            // representing all accessible rolls across the entire grid.
            .flat_map(|r| {
                // Determine the previous row slice, if it exists.
                let prev = if r > 0 { Some(&grid[r - 1]) } else { None };
                // Get the current row slice.
                let curr = &grid[r];
                // Determine the next row slice, if it exists.
                let next = if r < rows - 1 {
                    Some(&grid[r + 1])
                } else {
                    None
                };

                // Find accessible rolls in the current row.
                count_accessible_in_row(
                    prev.map(|v| v.as_slice()),
                    curr,
                    next.map(|v| v.as_slice()),
                    r,
                )
            })
            .collect();

        // If no rolls are accessible in this iteration, we are done.
        if to_remove.is_empty() {
            break;
        }

        // Add the count of newly accessible rolls to the total removed count.
        total_removed_rolls += to_remove.len() as u32;

        // "Remove" the accessible rolls from the grid by changing '@' to '.'.
        // It's important to iterate over the collected coordinates to modify the grid
        // after the analysis phase to ensure simultaneous updates.
        for (r, c) in to_remove {
            grid[r][c] = b'.';
        }
        // The loop will then re-evaluate accessibility based on the modified grid.
    }

    total_removed_rolls
}

fn main() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("input.txt");
    let input = fs::read_to_string(&input_path).expect("Could not read input.txt");
    println!("DEBUG: Input contains {} lines", input.lines().count());

    let grid = parse_grid(&input);
    if let Some(first_row) = grid.first() {
        let width = first_row.len();
        println!("DEBUG: Grid width: {}", width);
        for (i, row) in grid.iter().enumerate() {
            if row.len() != width {
                println!(
                    "DEBUG: Row {} has length {}, expected {}",
                    i,
                    row.len(),
                    width
                );
            }
            for (j, &b) in row.iter().enumerate() {
                if b != b'@' && b != b'.' {
                    println!("DEBUG: Unexpected char '{}' at ({}, {})", b as char, i, j);
                }
            }
        }
    }

    println!("\nPart 1: Total accessible rolls: {}", solve_part1(&input));
    println!("Part 2: Total rolls removed: {}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2};

    const SIMPLE_INPUT: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(SIMPLE_INPUT), 13);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(SIMPLE_INPUT), 43);
    }
}
