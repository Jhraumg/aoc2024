advent_of_code::solution!(1);

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let pairs: Vec<(u32, u32)> = input
        .lines()
        .filter_map(|l| {
            l.split_whitespace()
                // don't need error handling, but better see it as soon as possible
                .map(|w| w.parse().unwrap())
                .collect_tuple()
        })
        .collect();

    let first_list_sorted: Vec<u32> = pairs.iter().map(|(id1, _)| *id1).sorted().collect();
    let second_list_sorted: Vec<u32> = pairs.iter().map(|(_, id2)| *id2).sorted().collect();

    Some(
        first_list_sorted
            .into_iter()
            .zip(second_list_sorted)
            .map(|(first, second)| first.abs_diff(second))
            .sum(),
    )
}

fn similarity_score(id: u32, check_list: &[u32]) -> u32 {
    id * check_list.iter().filter(|c| **c == id).count() as u32
}

pub fn part_two(input: &str) -> Option<u32> {
    let pairs: Vec<(u32, u32)> = input
        .lines()
        .filter_map(|l| {
            l.split_whitespace()
                // don't need error handling, but better see it as soon as possible
                .map(|w| w.parse().unwrap())
                .collect_tuple()
        })
        .collect();

    let first_list: Vec<u32> = pairs.iter().map(|(id1, _)| *id1).collect();
    let second_list: Vec<u32> = pairs.iter().map(|(_, id2)| *id2).collect();

    Some(
        first_list
            .into_iter()
            .map(|id1| similarity_score(id1, &second_list))
            .sum(),
    )
}

#[cfg(test)]
mod tests_day01 {

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
