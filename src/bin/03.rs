advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new("mul\\(([0-9]{1,3}),([0-9]{1,3})\\)").unwrap();
    Some(
        re.captures_iter(input)
            .map(|c| {
                let v1: u32 = c.get(1).unwrap().as_str().parse().unwrap();
                let v2: u32 = c.get(2).unwrap().as_str().parse().unwrap();
                v1 * v2
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let main_re = Regex::new("mul\\(([0-9]{1,3}),([0-9]{1,3})\\)|do(?:n\'t)?\\(\\)").unwrap();

    Some(
        main_re
            .captures_iter(input)
            .fold((true, 0u32), |(active, sum), c| {
                match c.get(0).unwrap().as_str() {
                    "do()" => (true, sum),
                    "don't()" => (false, sum),
                    _ => (
                        active,
                        if active {
                            let v1: u32 = c.get(1).unwrap().as_str().parse().unwrap();
                            let v2: u32 = c.get(2).unwrap().as_str().parse().unwrap();
                            sum + v1 * v2
                        } else {
                            sum
                        },
                    ),
                }
            })
            .1,
    )
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
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(48));
    }
}
