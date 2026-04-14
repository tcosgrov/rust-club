use rstar::primitives::Line;
use rstar::{AABB, RTree};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

/// 1. Reads all coordinates from the text file into a list of (x, y) scalar pairs.
/// By making it generic (`T: FromStr`), it can cleanly read as `i64` for Part 1 or `f64` for Part 2!
fn parse_points<T: FromStr>(input: &str) -> Vec<(T, T)> {
    let mut points = Vec::new();
    for line in input.lines() {
        // Strip off any invisible whitespace or newline characters from the line
        let line = line.trim();
        
        // Skip any empty lines occasionally found at the end of input files.
        if line.is_empty() {
            continue;
        }
        
        // Split the line precisely at the comma character (e.g., "7,1" -> "7" and "1")
        if let Some((x, y)) = line.split_once(',') {
            // Attempt to parse both string halves into the desired numeric type.
            // Using the `if let (Ok... Ok...)` syntax guarantees we safely handle parsing errors!
            if let (Ok(x), Ok(y)) = (x.trim().parse::<T>(), y.trim().parse::<T>()) {
                // Store the parsed coordinate in our list
                points.push((x, y));
            }
        }
    }
    points
}

/// Solves Part 1 of the puzzle by finding the largest possible rectangle area
/// formed by using ANY two red tiles as opposite corners. 
/// There are no "green tile" shape constraints in Part 1.
fn solve_part1(input: &str) -> u64 {
    // 1. Read all coordinates from the text file into a list of (x, y) integer pairs.
    let points: Vec<(i64, i64)> = parse_points(input);

    // We will keep track of the largest computed area we successfully find.
    let mut max_area = 0;
    
    // 2. We want to test every unique combination of two points.
    // Iterating `i` from 0 to N, and then `j` from `i + 1` to N ensures we don't test
    // a pair against itself (where j=i) and we never test duplicate pairs backward (e.g., A-B then B-A).
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            // Grab the two points from the list
            let point_a = points[i];
            let point_b = points[j];
            
            // 3. Compute the horizontal (width) and vertical (height) distance.
            // `abs_diff` safely calculates the absolute distance between two numbers, mathematically dropping negative signs.
            let dx = point_a.0.abs_diff(point_b.0) as u64;
            let dy = point_a.1.abs_diff(point_b.1) as u64;
            
            // 4. Calculate the bounding area of the rectangle they form.
            // The problem uses a tiling model where a point "X=2" refers to the entire tile.
            // So if you stretch from X=2 to X=9, you physically cover 8 tiles!
            // Adding (+ 1) to both the mathematical width and height converts coordinates to physical tile lengths.
            let area = (dx + 1) * (dy + 1);
            
            // Dynamically update the global maximum if this newly constructed rectangle is larger.
            max_area = max_area.max(area);
        }
    }
    
    // Return the absolute largest area discovered and solved!
    max_area
}

/// This function mathematically tests if a polygon's boundary line
/// slices directly through the *strict, hollow center* of our candidate rectangle.
/// It verifies: if the candidate is solid green, no red/green border should cut through the middle of it.
fn strict_intersects_interior(
    line: &Line<[f64; 2]>,
    rx_min: f64,
    ry_min: f64,
    rx_max: f64,
    ry_max: f64,
) -> bool {
    // 1. Unpack the starting and ending (x,y) coordinates of the polygon's boundary line.
    let x1 = line.from[0];
    let y1 = line.from[1];
    let x2 = line.to[0];
    let y2 = line.to[1];

    // 2. Identify the lowest and highest mathematical extents of the boundary line. 
    // This calculates the bounding box for this specific segment.
    let min_x = x1.min(x2);
    let max_x = x1.max(x2);
    let min_y = y1.min(y2);
    let max_y = y1.max(y2);

    // 3. Since all our polygon boundary lines are perfectly orthogonal (up/down or left/right), we check which way this one points:
    if x1 == x2 {
        // The line is VERTICAL because its X coordinate doesn't change.
        // First check: Is this vertical line located strictly *between* the candidate's left and right borders?
        if x1 > rx_min && x1 < rx_max {
            // Second check: Does this vertical line vertically extend *into* the top/bottom boundary of the candidate?
            if max_y > ry_min && min_y < ry_max {
                // If both are true, the wall slices vertically straight through the heart of the rectangle!
                return true;
            }
        }
    } else {
        // The line is HORIZONTAL because its Y coordinate doesn't change.
        // First check: Is this horizontal line located strictly *between* the candidate's top and bottom borders?
        if y1 > ry_min && y1 < ry_max {
            // Second check: Does this horizontal line horizontally extend *into* the left/right boundary of the candidate?
            if max_x > rx_min && min_x < rx_max {
                // If both are true, the wall slices horizontally straight through the heart of the rectangle!
                return true;
            }
        }
    }
    // If we reach here, the boundary line might safely touch the outer edge of our green rectangle, but it never cuts *through* the center.
    false
}

/// Determines if a point is strictly "inside" the polygon.
/// It uses the classic "Ray Casting" algorithm: imagine shooting a laser straight to the right from the point.
/// Every time the laser crosses a vertical boundary of the polygon, it switches from inside to outside, or vice-versa.
/// If it crosses an ODD number of lines, the starting point was Inside. 
/// If it crosses an EVEN number, it was Outside.
fn is_point_in_polygon(px: f64, py: f64, segments: &[Line<[f64; 2]>]) -> bool {
    // We will keep a running count of how many vertical walls our laser hits.
    let mut intersections = 0;

    // Test our laser against every single boundary line of the polygon.
    for seg in segments {
        // Unpack the segment's endpoints.
        let x1 = seg.from[0];
        let y1 = seg.from[1];
        let x2 = seg.to[0];
        let y2 = seg.to[1];

        // Is this boundary wall vertical? A horizontal laser traveling straight right can only hit vertical walls!
        if x1 == x2 {
            // Find the bottom and top boundaries of the vertical wall
            let min_y = y1.min(y2);
            let max_y = y1.max(y2);

            // To be hit by the laser, two conditions must be met:
            // 1. The wall must be strictly to the RIGHT of our starting point (px < x1)
            // 2. The laser's Y height (py) must fall within the Y-span of the wall.
            // Note: We use a "half-open interval" (py > min_y && py <= max_y) which avoids double-counting 
            // if the laser strikes directly precisely on a corner joining two separate wall segments!
            if px < x1 && py > min_y && py <= max_y {
                intersections += 1;
            }
        }
    }
    // The modulo operator (%) divides by 2 and returns the remainder. 
    // If the remainder is NOT zero, the intersection count was odd, meaning we were inside the entire time.
    intersections % 2 != 0
}

fn solve_part2(input: &str) -> u64 {
    // 1. We first extract the raw coordinate (x,y) pairs from the input list as floating point numbers.
    let points: Vec<(f64, f64)> = parse_points(input);

    if points.is_empty() {
        return 0;
    }

    // 2. The problem states that the coordinates loop back to the start.
    // So we iterate through the list and link point[i] to point[i+1] to form the outer boundary "segments".
    let mut segments = Vec::new();
    for i in 0..points.len() {
        let p1 = points[i];
        let p2 = points[(i + 1) % points.len()];
        let seg = Line::new([p1.0, p1.1], [p2.0, p2.1]);
        segments.push(seg);
    }

    // 3. We build an R-Tree using the `rstar` crate.
    // An R-Tree (Rectangle Tree) is a spatial index. It groups nearby line segments into progressively 
    // larger bounding boxes. Later, instead of looping through all 500 lines to see if they overlap 
    // a candidate area, we query the R-Tree. It traverses its bounding boxes and skips thousands of checks 
    // by ignoring whole branches that are nowhere near our target area!
    let tree = RTree::bulk_load(segments.clone());

    let mut max_area = 0;

    // 4. We evaluate every possible pair of coordinates as opposite corners of a candidate rectangle.
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let p1 = points[i];
            let p2 = points[j];

            // Extract the minimum and maximum dimensions of our candidate rectangle.
            let x_min = p1.0.min(p2.0);
            let y_min = p1.1.min(p2.1);
            let x_max = p1.0.max(p2.0);
            let y_max = p1.1.max(p2.1);

            // Re-wrap bounds into an AABB (Axis-Aligned Bounding Box) so the R-Tree understands it.
            let aabb = AABB::from_corners([x_min, y_min], [x_max, y_max]);

            // Query the R-Tree: "Give me only the boundary lines that roughly overlap my candidate's target area"
            let mut strict_intersection = false;
            for seg in tree.locate_in_envelope_intersecting(&aabb) {
                // If any boundary line cuts directly through our green rectangle, the rectangle is invalid.
                if strict_intersects_interior(seg, x_min, y_min, x_max, y_max) {
                    strict_intersection = true;
                    break;
                }
            }

            // 5. If no boundary sliced through our rectangle, we must additionally verify the candidate
            // isn't just floating in the empty void strictly OUTSIDE the polygon. 
            if !strict_intersection {
                let px = (x_min + x_max) / 2.0;
                let py = (y_min + y_max) / 2.0;

                // We use Ray Casting on the exact center of our rectangle to verify if it's currently inside.
                // We mathematically offset the ray slightly (+0.314) because polygon vertices are uniformly on whole integers.
                // This guarantees our ray never perfectly strikes a vertex head-on, which usually breaks the raycast mathematics.
                let mut is_contained = is_point_in_polygon(px + 0.31415, py + 0.31415, &segments);

                // Quick fallback: if a flat width=0 or height=0 candidate failed the floating point check (since
                // it acts exactly on a 1D border and lacks area), we verify its midpoint precisely lies on an allowed border segment.
                if !is_contained && (x_min == x_max || y_min == y_max) {
                    // Iterate through every boundary segment sitting natively inside our candidate's AABB zone
                    for seg in tree.locate_in_envelope_intersecting(&aabb) {
                        // Extract the exact boundary wall endpoints
                        let x1 = seg.from[0];
                        let y1 = seg.from[1];
                        let x2 = seg.to[0];
                        let y2 = seg.to[1];

                        // Find the top/bottom and left/right limits for this specific wall
                        let min_x = x1.min(x2);
                        let max_x = x1.max(x2);
                        let min_y = y1.min(y2);
                        let max_y = y1.max(y2);
                        
                        // Check: Is our candidate's fractional center point located exactly upon this boundary wall's body?
                        if px >= min_x && px <= max_x && py >= min_y && py <= max_y {
                            // If it's on the boundary, we accept it as inside!
                            is_contained = true;
                            break;
                        }
                    }
                }

                // 6. Finally, if the rectangle passed the strict boundary cross test AND we proved it is inside the green polygon...
                if is_contained {
                    // Calculate the size of the rectangle by adding +1 to convert mathematical lengths to physical tile coverage
                    let area =
                        ((x_max - x_min).abs() as u64 + 1) * ((y_max - y_min).abs() as u64 + 1);
                        
                    // Replace our current max score if we beat it
                    max_area = max_area.max(area);
                }
            }
        }
    }
    max_area
}

fn main() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("input.txt");
    let input = fs::read_to_string(&input_path).expect("Could not read input.txt");

    println!("\nPart 1: Largest Area   : {}", solve_part1(&input));
    println!("Part 2: Largest Green  : {}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2};

    const EXAMPLE_INPUT: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 50);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE_INPUT), 24);
    }
}
