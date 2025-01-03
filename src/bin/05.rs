use itertools::Itertools;
use std::cmp::Ordering;

advent_of_code::solution!(5);

fn is_ordered(update: &[usize], rules: &[(usize, usize)]) -> bool {
    update.iter().enumerate().all(|(i, n)| {
        //let rules:Vec<_>=rules.iter().filter(|(first,_)|*first==*n).collect();
        !update[i + 1..].iter().any(|m| {
            rules
                .iter()
                .any(|(first, last)| *last == *n && *first == *m)
        })
    })
}
fn middle(update: &[usize]) -> usize {
    let len = update.len();
    assert_eq!(1, len % 2, "no middle for {len}");
    update[len / 2]
}

pub fn part_one(input: &str) -> Option<u32> {
    let rules: Vec<(usize, usize)> = input
        .lines()
        .filter_map(|l| {
            if l.contains('|') {
                l.split('|')
                    .map(|w| w.parse::<usize>().unwrap())
                    .collect_tuple()
            } else {
                None
            }
        })
        .collect();
    let updates: Vec<Vec<usize>> = input
        .lines()
        .filter_map(|l| {
            if l.contains(',') {
                Some(l.split(',').map(|w| w.parse::<usize>().unwrap()).collect())
            } else {
                None
            }
        })
        .collect();

    Some(
        updates
            .iter()
            .filter(|u| is_ordered(u, &rules))
            .map(|u| middle(u))
            .sum::<usize>() as u32,
    )
}

fn get_cmp_by_rules(rules: &[(usize, usize)]) -> impl FnMut(&usize, &usize) -> Ordering + use<'_> {
    |a, b| {
        rules
            .iter()
            .find_map(|(first, second)| match (*first, *second) {
                (s, o) if s == *a && o == *b => Some(Ordering::Less),
                (s, o) if s == *b && o == *a => Some(Ordering::Greater),
                _ => None,
            })
            .expect("no rule to compare theses values : cannot fully sort")
    }
}
pub fn part_two(input: &str) -> Option<u32> {
    let rules: Vec<(usize, usize)> = input
        .lines()
        .filter_map(|l| {
            if l.contains('|') {
                l.split('|')
                    .map(|w| w.parse::<usize>().unwrap())
                    .collect_tuple()
            } else {
                None
            }
        })
        .collect();
    let updates: Vec<Vec<usize>> = input
        .lines()
        .filter_map(|l| {
            if l.contains(',') {
                Some(l.split(',').map(|w| w.parse::<usize>().unwrap()).collect())
            } else {
                None
            }
        })
        .collect();

    let rules = &rules;
    Some(
        updates
            .into_iter()
            .filter(|u| !is_ordered(u, rules))
            .map(|us| {
                let sorted = us
                    .into_iter()
                    .sorted_by(get_cmp_by_rules(rules))
                    .collect_vec();
                middle(&sorted)
            })
            .sum::<usize>() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn sort_can_sort() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let rules: Vec<(usize, usize)> = input
            .lines()
            .filter_map(|l| {
                if l.contains('|') {
                    l.split('|')
                        .map(|w| w.parse::<usize>().unwrap())
                        .collect_tuple()
                } else {
                    None
                }
            })
            .collect();
        let rules = &rules;
        let updates = [
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];
        let results = vec![
            vec![97, 75, 47, 61, 53],
            vec![61, 29, 13],
            vec![97, 75, 47, 29, 13],
        ];
        for i in 0..updates.len() {
            let sorted = updates[i]
                .iter()
                .copied()
                .sorted_by(get_cmp_by_rules(rules))
                .collect_vec();
            assert_eq!(sorted, results[i]);
        }
    }
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
