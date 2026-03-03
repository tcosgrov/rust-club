use std::fs;
use std::path::PathBuf;

/// Parses the input string into two vectors:
/// 1. A vector of inclusive ranges (start, end).
/// 2. A vector of ingredient IDs to check.
///
/// The input is expected to be two sections separated by a double newline.
/// The first section contains ranges in "min-max" format.
/// The second section contains individual IDs.
fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    // Split the input into two sections based on the blank line.
    let mut sections = input.split("\n\n");
    // Use the iterator to get the first section
    let ranges_str = sections.next().unwrap_or("");
    // Use the iterator to get the second section
    let ids_str = sections.next().unwrap_or("");

    // Parse the first section into a vector of (start, end) tuples.
    let ranges = ranges_str
        .lines()
        .filter_map(|line| {
            let mut parts = line.split('-');
            let start = parts.next()?.parse::<u64>().ok()?;
            let end = parts.next()?.parse::<u64>().ok()?;
            Some((start, end))
        })
        .collect();

    // Parse the second section into a vector of individual IDs.
    let ids = ids_str
        .lines()
        .filter_map(|line| line.trim().parse::<u64>().ok())
        .collect();

    (ranges, ids)
}

/// Solves Part 1: Counts how many available ingredient IDs are "fresh".
///
/// An ID is considered fresh if it falls within *any* of the fresh ingredient ranges.
/// The ranges are inclusive.
fn solve_part1(input: &str) -> u64 {
    let (ranges, ids) = parse_input(input);
    // Iterate through each ID and check if it exists in any of the ranges.
    // The filter keeps only IDs that satisfy the condition (id >= start && id <= end).
    ids.iter()
        .filter(|&&id| ranges.iter().any(|&(start, end)| id >= start && id <= end))
        .count() as u64
}

/// Solves Part 2: Calculates the total number of unique IDs covered by the fresh ingredient ranges.
///
/// Since ranges can overlap, we cannot simply sum their lengths.
/// The strategy is to:
/// 1. Sort the ranges by their start value.
/// 2. Merge overlapping or adjacent ranges into a set of disjoint ranges.
/// 3. Sum the lengths of these merged ranges.
fn solve_part2(input: &str) -> u64 {
    // Don't care about the second arg in the tuple for the IDs
    let (mut ranges, _) = parse_input(input);
    // Sort ranges by start value for merging.
    ranges.sort_by_key(|r| r.0);

    // Iterate through sorted ranges and merge them.
    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();
    for current_range in ranges {
        if let Some(previous_range) = merged_ranges.last_mut() {
            // If the current range overlaps with or is adjacent to the last merged range...
            // (e.g., [1-5] and [6-10] are adjacent and can be merged into [1-10])
            if current_range.0 <= previous_range.1 + 1 {
                // Extend the end of the last range if the current range goes further.
                previous_range.1 = previous_range.1.max(current_range.1);
            } else {
                // No overlap/adjacency, start a new merged range.
                merged_ranges.push(current_range);
            }
        } else {
            merged_ranges.push(current_range);
        }
    }

    // Calculate the total count of integers in the merged ranges.
    // Length of an inclusive range [a, b] is (b - a + 1).
    merged_ranges
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum()
}

fn main() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("input.txt");
    let input = fs::read_to_string(&input_path).expect("Could not read input.txt");

    println!(
        "Part 1: Fresh ingredients in list : {}",
        solve_part1(&input)
    );
    println!(
        "Part 2: Count of fresh ingredients: {}",
        solve_part2(&input)
    );
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2};

    const PART1_SIMPLE_INPUT: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    const PART2_SIMPLE_INPUT: &str = "\
3-5
10-14
16-20
12-18";

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(PART1_SIMPLE_INPUT), 3);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(PART2_SIMPLE_INPUT), 14);
    }
}
