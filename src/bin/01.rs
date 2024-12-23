use counter::Counter;

advent_of_code::solution!(1);

/// Compute the total distance between the corresponding elements in the
/// left and right lists, where the smallest element in the left list
/// corresponds to the smallest element in the right list, then the next
/// smallest element in the left list corresponds to the next smallest element
/// in the right list, and so on
pub fn part_one(input: &str) -> Option<u64> {
    // Get both lists
    let (mut left_list, mut right_list) = parse_nums(input);

    // Sort both lists in ascending order
    left_list.sort_unstable();
    right_list.sort_unstable();

    let total_distance = left_list
        .iter()
        .zip(right_list)
        .map(|(l, r)| l.abs_diff(r))
        .sum();

    Some(total_distance)
}

/// Compute the total similarity score between the left and the right
/// lists, where the similarity score of an element in the left list
/// is defined as its value multiplied by the number of times it occurs
/// in the right list
pub fn part_two(input: &str) -> Option<u64> {
    // Get both lists
    let (left_list, right_list) = parse_nums(input);

    // Generate a counter for each element in the right list
    let right_list_counts: Counter<_> = right_list.iter().collect();

    let mut total_similarity_score = 0;

    for id in left_list {
        if let Some(id_count) = right_list_counts.get(&id) {
            total_similarity_score += id * (*id_count as u64);
        }
    }

    Some(total_similarity_score)
}

/// Parse the input contents into a vector of the left and right
/// columns, respectively
pub fn parse_nums(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    for line in input.lines() {
        // Split each line on whitespace
        let mut nums = line.split_whitespace();

        // Parse both elements into u64s and append them to left and right lists
        if let (Some(left), Some(right)) = (nums.next(), nums.next()) {
            if let (Ok(left), Ok(right)) = (left.parse::<u64>(), right.parse::<u64>()) {
                left_list.push(left);
                right_list.push(right);
            }
        }
    }
    (left_list, right_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
