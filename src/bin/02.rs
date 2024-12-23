advent_of_code::solution!(2);

/// Given a list of reports, which itself is a
/// list of levels, calculate how many reports are safe.
/// A report is safe the levels in the report are monotonically
/// increasing or decreasing, and the rate by which they increase
/// or decrease is at least 1 and at most 3.
pub fn part_one(input: &str) -> Option<u64> {
    let reports = parse_nums(input);

    let mut num_safe_reports = 0;

    for report in reports {
        // Handle reports with only 0 or 1 levels
        if report.len() < 2 {
            num_safe_reports += 1;
            continue;
        }

        // Track whether based on the first two elements if this report
        // should be monotonically increasing or decreasing.
        let increasing = report[1] > report[0];

        // Track last level
        let mut last_level = report[0];

        // Start with report is safe, set false and break if conditions not held
        let mut report_safe = true;

        for level in &report[1..] {
            let level_change = level.abs_diff(last_level);

            if level_change > 3 || level_change < 1 {
                report_safe = false;
                break;
            }

            // Whether this current change increased or decreased
            let level_increased = *level > last_level;

            if level_increased != increasing {
                report_safe = false;
                break;
            }

            // Set last level to be this level
            last_level = *level;
        }

        // Loop breaks if unsafe, otherwise leaves report_safe to be true
        if !report_safe {
            continue;
        }

        num_safe_reports += 1;
    }

    Some(num_safe_reports)
}

/// Given the list of reports as above, calculate the number of safe reports
/// if you are allowed to remove up to 1 bad level from each report.
pub fn part_two(input: &str) -> Option<u64> {
    let reports = parse_nums(input);

    let mut num_safe_reports = 0;

    for report in reports {
        // Handle reports with only 0 or 1 levels
        if report.len() < 2 {
            num_safe_reports += 1;
            continue;
        }

        // Track whether based on the first two elements if this report
        // should be monotonically increasing or decreasing.
        let increasing = report[1] > report[0];

        // Track last level
        let mut last_level = report[0];

        // Start with report is true, set false and break if conditions not held
        let mut report_safe = true;

        // Track whether we used the dampener already
        let mut dampener_used = false;

        for level in &report[1..] {
            // Change in level
            let level_change = level.abs_diff(last_level);
            // Whether this current change increased or decreased
            let level_increased = *level > last_level;

            if level_change > 3 || level_change < 1 || level_increased != increasing {
                if !dampener_used {
                    // Try dropping this level and see if we're OK
                    dampener_used = true;
                    continue;
                }
                report_safe = false;
                break;
            }

            // Set last level to be this level
            last_level = *level;
        }

        // Loop breaks if unsafe, otherwise leaves report_safe to be true
        if !report_safe {
            continue;
        }

        num_safe_reports += 1;
    }

    Some(num_safe_reports)
}

pub fn parse_nums(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|level| level.parse::<u64>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
