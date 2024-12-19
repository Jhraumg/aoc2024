use rustc_hash::{FxHashMap, FxHashSet};

advent_of_code::solution!(19);

fn read_input(input: &str) -> (FxHashSet<&str>, Vec<&str>) {
    let towels: FxHashSet<_> = input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|w| w.trim())
        .filter(|t| !t.is_empty())
        .collect();
    let patterns: Vec<_> = input
        .lines()
        .skip(2)
        .map(str::trim)
        .filter(|w| !w.is_empty())
        .collect();

    assert!(towels
        .iter()
        .all(|w| w.chars().all(|c| ['w', 'u', 'b', 'r', 'g'].contains(&c))));
    assert!(patterns
        .iter()
        .all(|w| w.chars().all(|c| ['w', 'u', 'b', 'r', 'g'].contains(&c))));
    (towels, patterns)
}

fn possible<'a>(
    pattern: &'a str,
    towels: &FxHashSet<&'a str>,
    impossible: &mut FxHashSet<&'a str>,
) -> bool {
    if impossible.contains(pattern) {
        return false;
    }
    for t in towels.iter().filter(|t| pattern.starts_with(*t)) {
        if *t == pattern {
            return true;
        }
        let remain = &pattern[t.len()..];

        if possible(remain, towels, impossible) {
            return true;
        }
    }
    impossible.insert(pattern);
    false
}

fn count_possible<'a>(
    pattern: &'a str,
    towels: &FxHashSet<&'a str>,
    counts_by_pattern: &mut FxHashMap<&'a str, usize>,
) -> usize {
    if counts_by_pattern.contains_key(pattern) {
        return *counts_by_pattern.get(&pattern).unwrap();
    }
    let mut count = 0;
    for t in towels.iter().filter(|t| pattern.starts_with(*t)) {
        if *t == pattern {
            count += 1;
        }
        let remain = &pattern[t.len()..];

        count += count_possible(remain, towels, counts_by_pattern);
    }
    counts_by_pattern.insert(pattern, count);
    count
}

pub fn part_one<'a>(input: &'a str) -> Option<usize> {
    let (towels, pattern) = read_input(input);
    let mut impossible: FxHashSet<&'a str> = Default::default();

    Some(
        pattern
            .into_iter()
            .filter(|p| possible(p, &towels, &mut impossible))
            .count(),
    )
}

pub fn part_two<'a>(input: &'a str) -> Option<usize> {
    let (towels, pattern) = read_input(input);
    let mut count_by_pattern: FxHashMap<&'a str, usize> = Default::default();

    Some(
        pattern
            .into_iter()
            .map(|p| count_possible(p, &towels, &mut count_by_pattern))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
