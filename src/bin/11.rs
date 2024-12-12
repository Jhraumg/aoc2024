use std::collections::HashMap;

advent_of_code::solution!(11);

fn blink(i: usize) -> [Option<usize>; 2] {
    match i {
        0 => [Some(1), None],
        k if k.ilog10() % 2 == 1 => {
            let p10 = 10usize.pow(k.ilog10() / 2 + 1);
            [Some(k / p10), Some(k % p10)]
        }
        _ => [Some(i * 2024), None],
    }
}

fn len_after_blink(i: usize, nb_blinks: usize) -> usize {
    let mut v = vec![i];
    for _ in 0..nb_blinks {
        v = v
            .iter()
            .flat_map(move |i| blink(*i).into_iter().flatten())
            .collect();
    }
    v.len()
}

pub fn part_one(input: &str) -> Option<usize> {
    let stones: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    Some(stones.into_iter().map(|i| len_after_blink(i, 25)).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let stones: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut count_by_stones: HashMap<usize, usize> = HashMap::new();

    for s in stones {
        *count_by_stones.entry(s).or_insert(0) += 1;
    }

    for _ in 0..75 {
        let mut new_counts: HashMap<usize, usize> = HashMap::new();
        for (s, nb) in count_by_stones {
            for ns in blink(s).into_iter().flatten() {
                *new_counts.entry(ns).or_insert(0) += nb;
            }
        }
        count_by_stones = new_counts;
    }

    Some(count_by_stones.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blink() {
        assert_eq!(blink(0), [Some(1), None]);
        assert_eq!(blink(125), [Some(253000), None]);
        assert_eq!(blink(17), [Some(1), Some(7)]);

        assert_eq!(blink(1234567890), [Some(12345), Some(67890)]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
