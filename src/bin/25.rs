advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<u64> {
    let (locks, keys) = parse_locks_and_keys(input);

    let mut total_fitting = 0;

    for lock in locks.iter() {
        for key in keys.iter() {
            if lock.iter().zip(key).all(|p| p.0 + p.1 <= 5) {
                total_fitting += 1;
            }
        }
    }

    Some(total_fitting)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

pub fn parse_locks_and_keys(input: &str) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    // Assume number of characters in first row defines the schematic width
    let schematic_width = input.lines().nth(0).unwrap().trim().len();

    let mut item = vec![0; schematic_width];
    let mut is_lock = true;
    let mut new_item_started = false;

    for line in input.lines() {

        if line.trim().is_empty() {
            // New lock/key
            if !item.iter().all(|l| {*l == 0}) {
                if is_lock {
                    locks.push(item);
                } else {
                    // Minus 1 from each height for keys so we don't count the last layer of #
                    item.iter_mut().for_each(|l| *l -= 1);
                    keys.push(item);
                }
                new_item_started = false;
            }
            item = vec![0; schematic_width];
        } else if line.trim() == "#".repeat(schematic_width) && !new_item_started {
                is_lock = true;
                new_item_started = true;
        }
        else if line.trim() == ".".repeat(schematic_width) && !new_item_started {
                is_lock = false;
                new_item_started = true;
        } else {
            for (i, space) in line.trim().chars().enumerate() {
                if space == '#' {
                    item[i] += 1;
                }
            }
        }
    }

    if !item.is_empty() {
        if is_lock {
            locks.push(item);
        } else {
            // Minus 1 from each height for keys so we don't count the last layer of #
            item.iter_mut().for_each(|l| *l -= 1);
            keys.push(item);
        }
    }

    // println!("Found {} locks and {} keys", locks.len(), keys.len());

    (locks, keys)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));

        let key = ".....
                        ...#.
                        .#.#.
                        ##.#.
                        ##.#.
                        #####
                        #####";

        let (_, keys) = parse_locks_and_keys(key);

        assert_eq!(keys[0], vec![3u8, 4u8, 1u8, 5u8, 1u8]);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
