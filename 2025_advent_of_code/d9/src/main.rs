use rusqlite::{params, Connection};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

/// Read all coordinates from the text file into a list of (x, y) pairs.
/// Note: using the generic so I can use for both parts
fn parse_points<T: FromStr>(input: &str) -> Vec<(T, T)> {
    let mut points = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some((x, y)) = line.split_once(',') {
            if let (Ok(x), Ok(y)) = (x.trim().parse::<T>(), y.trim().parse::<T>()) {
                points.push((x, y));
            }
        }
    }
    points
}

fn solve_part1(input: &str) -> u64 {
    let points: Vec<(i64, i64)> = parse_points(input);
    let mut max_area = 0;
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let p1 = points[i];
            let p2 = points[j];
            let len1 = p1.0.abs_diff(p2.0) as u64;
            let len2 = p1.1.abs_diff(p2.1) as u64;
            let area = (len1 + 1) * (len2 + 1);
            max_area = max_area.max(area);
        }
    }
    max_area
}

fn populate_database(points: &[(f64, f64)]) -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute("CREATE VIRTUAL TABLE walls USING rtree(id, minX, maxX, minY, maxY)", []).unwrap();

    for i in 0..points.len() {
        let p1 = points[i];
        let p2 = points[(i + 1) % points.len()];

        let min_x = p1.0.min(p2.0);
        let max_x = p1.0.max(p2.0);
        let min_y = p1.1.min(p2.1);
        let max_y = p1.1.max(p2.1);

        conn.execute(
            "INSERT INTO walls (minX, maxX, minY, maxY) VALUES (?1, ?2, ?3, ?4)",
            params![min_x, max_x, min_y, max_y],
        ).unwrap();
    }
    
    conn
}

fn solve_part2(input: &str) -> u64 {
    let points: Vec<(f64, f64)> = parse_points(input);
    let conn = populate_database(&points);

    // Intersection check:
    // Does any wall of the polygon cut straight through the middle of the rectangle?
    let mut stmt_wall_intersect_count = conn.prepare(
        "SELECT COUNT(1) FROM walls 
         WHERE minX <= ?2 AND maxX >= ?1 AND minY <= ?4 AND maxY >= ?3
           AND (
             (minX = maxX AND minX > ?1 AND minX < ?2 AND maxY > ?3 AND minY < ?4)
             OR 
             (minY = maxY AND minY > ?3 AND minY < ?4 AND maxX > ?1 AND minX < ?2)
           ) LIMIT 1"
    ).unwrap();

    // Inside v. Outside check:
    // Is the center of the rectangle actually inside the polygon?
    let mut stmt_crossings = conn.prepare(
        "SELECT COUNT(1) FROM walls 
         WHERE minX >= ?1 AND minY <= ?2 AND maxY >= ?2 
           AND minX = maxX 
           AND ?1 < minX AND ?2 > minY AND ?2 <= maxY"
    ).unwrap();

    // Flat and on-edge check
    // If the rectangle is completely squashed flat, is it laying perfectly on top of a side?
    let mut stmt_on_side = conn.prepare(
        "SELECT COUNT(1) FROM walls
         WHERE minX <= ?1 AND maxX >= ?1 AND minY <= ?2 AND maxY >= ?2
         AND (?1 >= minX AND ?1 <= maxX AND ?2 >= minY AND ?2 <= maxY) LIMIT 1"
    ).unwrap();

    let mut max_area = 0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let p1 = points[i];
            let p2 = points[j];

            let x_min = p1.0.min(p2.0);
            let y_min = p1.1.min(p2.1);
            let x_max = p1.0.max(p2.0);
            let y_max = p1.1.max(p2.1);

            let strict_count: i64 = stmt_wall_intersect_count.query_row(
                params![x_min, x_max, y_min, y_max], 
                |row| row.get(0)
            ).unwrap();

            if strict_count == 0 {
                // No intersecting walls, continue...
                let px = (x_min + x_max) / 2.0;
                let py = (y_min + y_max) / 2.0;

                let ray_crossings: i64 = stmt_crossings.query_row(
                    params![px + 0.31415, py + 0.31415],
                    |row| row.get(0)
                ).unwrap();
                
                let mut is_contained = ray_crossings % 2 != 0;

                if !is_contained && (x_min == x_max || y_min == y_max) {
                    let border_count: i64 = stmt_on_side.query_row(
                        params![px, py],
                        |row| row.get(0)
                    ).unwrap();
                    
                    if border_count > 0 {
                        is_contained = true;
                    }
                }

                if is_contained {
                    let area = ((x_max - x_min).abs() as u64 + 1) * ((y_max - y_min).abs() as u64 + 1);
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
