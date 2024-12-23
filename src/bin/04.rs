advent_of_code::solution!(4);

/// Given a rectangular grid of characters, computes the number of times the string
/// "XMAS" occurs in any orientation
pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_grid(input);

    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let mut total_occurences = 0;

    for row in 0..num_rows {
        for col in 0..num_cols {
            if grid[row][col] == 'X' {
                for &d in DIRECTIONS.iter() {
                    if match_string_in_direction("XMAS", &grid, (row as isize, col as isize), d) {
                        total_occurences += 1;
                    }
                }
            }
        }
    }

    Some(total_occurences)
}

/// Given the same grid as above, instead computes the number of times
pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse_grid(input);

    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let mut total_occurences = 0;

    for row in 0..num_rows {
        for col in 0..num_cols {
            if grid[row][col] == 'M' {
                for &d in DIAGONAL_DIRECTIONS.iter() {
                    let (y, x) = (row as isize, col as isize);
                    if match_string_in_direction("MAS", &grid, (y, x), d) {
                        let (dy, dx) = d;
                        // Check first possible cross
                        let first_cross_position = (y, x + dx * 2);
                        let first_cross_dir = (dy, -dx);

                        if match_string_in_direction(
                            "MAS",
                            &grid,
                            first_cross_position,
                            first_cross_dir,
                        ) {
                            total_occurences += 1;
                            continue;
                        }

                        // Check second possible cross
                        let second_cross_position = (y + dy * 2, x);
                        let second_cross_dir = (-dy, dx);

                        if match_string_in_direction(
                            "MAS",
                            &grid,
                            second_cross_position,
                            second_cross_dir,
                        ) {
                            total_occurences += 1;
                            continue;
                        }
                    }
                }
            }
        }
    }

    // Each cross gets counted twice, so divide by two
    Some(total_occurences / 2)
}

pub fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

const DIRECTIONS: [(isize, isize); 8] = [
    (1, 0),   // Up
    (1, 1),   // Up Right
    (0, 1),   // Right
    (-1, 1),  // Down Right
    (-1, 0),  // Down
    (-1, -1), // Down Left
    (0, -1),  // Left
    (1, -1),  // Up Left
];

const DIAGONAL_DIRECTIONS: [(isize, isize); 4] = [
    (1, 1),   // Up Right
    (-1, 1),  // Down Right
    (-1, -1), // Down Left
    (1, -1),  // Up Left
];

pub fn position_within_grid(position: (isize, isize), grid: &Vec<Vec<char>>) -> bool {
    let (y, x) = position;
    y >= 0 && y < grid.len() as isize && x >= 0 && x < grid[0].len() as isize
}

pub fn match_string_in_direction(
    string: &str,
    grid: &Vec<Vec<char>>,
    initial_position: (isize, isize),
    direction: (isize, isize),
) -> bool {
    let mut current_position = initial_position;
    for c in string.chars() {
        let (y, x) = current_position;
        let (dy, dx) = direction;

        if !position_within_grid(current_position, grid) || grid[y as usize][x as usize] != c {
            return false;
        }

        current_position = (y + dy, x + dx);
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
