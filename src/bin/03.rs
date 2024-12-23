use regex::Regex;

advent_of_code::solution!(3);

/// Given a string of commands, parse the exact matches for `mul(X,Y)` where
/// X and Y are non-negative integers. Multiply each X and Y and return the
/// sum of all matches.
pub fn part_one(input: &str) -> Option<u64> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let total = re.captures_iter(input).fold(0, |acc, cap| {
        let x = cap.get(1).unwrap().as_str().parse::<u64>().unwrap();
        let y = cap.get(2).unwrap().as_str().parse::<u64>().unwrap();
        acc + x * y
    });

    Some(total)
}

/// Same as above except with new commands `do()` and `don't()` which may be present.
/// After reading a `don't()`, do not sum any subsequent `mul(X,Y)` until reading a `do()` again.
pub fn part_two(input: &str) -> Option<u64> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut total = 0;
    let mut do_muls = true;

    for cap in re.captures_iter(input) {
        match cap.get(0).unwrap().as_str() {
            "don't()" => {
                do_muls = false;
            }

            "do()" => {
                do_muls = true;
            }

            _ => {
                if do_muls {
                    let x = cap.get(1).unwrap().as_str().parse::<u64>().unwrap();
                    let y = cap.get(2).unwrap().as_str().parse::<u64>().unwrap();
                    total += x * y;
                }
            }
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
