use itertools::Itertools;
use std::cmp::Ordering;

advent_of_code::solution!(05);

fn is_ordered(update: &[usize], rules: &[(usize, usize)]) -> bool {
    update.iter().enumerate().all(|(i, n)| {
        //let rules:Vec<_>=rules.iter().filter(|(first,_)|*first==*n).collect();
        !update[i + 1..].iter().any(|m| {
            rules.iter().any(|(first, last)| {
                let result = *last == *n && *first == *m;
                // if result{println!("{n} <{m} in {update:?} but {rules:?}=>{first},{last}");}
                result
            })
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

fn sort(updates: &[usize], rules: &[(usize, usize)]) -> Vec<usize> {
    let mut result = Vec::with_capacity(updates.len());

    let updates_ruled: Vec<_> = updates
        .iter()
        .filter(|u| {
            rules
                .iter()
                .any(|(first, last)| *first == **u || *last == **u)
        })
        .copied()
        .collect();
    let mut remaining = updates_ruled.to_owned();
    while remaining.len() > 1 {
        // println!("{:?}", remaining);
        let first = *remaining
            .iter()
            .find(|f| {
                // TODO : replace other.contains() with remaining.any()
                let other: Vec<_> = remaining.iter().filter(|o| **o != **f).copied().collect();
                let is_before_some = rules
                    .iter()
                    .any(|(first, second)| *first == **f && other.contains(second));
                let is_after_some = rules
                    .iter()
                    .enumerate()
                    .any(|(i, (first, last))| other.iter().any(|o| *first == *o && *last == **f));
                // println!("{f} is before {is_before_some} / is after {is_after_some}");
                is_before_some && !is_after_some
            })
            .unwrap();
        result.push(first);
        remaining = remaining.into_iter().filter(|r| *r != first).collect_vec();
    }
    result.push(remaining[0]);
    updates
        .into_iter()
        .filter(|u| {
            !rules
                .iter()
                .any(|(first, last)| *first == **u || *last == **u)
        })
        .for_each(|u| result.push(*u));

    result
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

    Some(
        updates
            .into_iter()
            .filter(|u| !is_ordered(u, &rules))
            .map(|u| middle(&sort(&u, &rules)))
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
        let updates = [
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];
        let results = [
            vec![97, 75, 47, 61, 53],
            vec![61, 29, 13],
            vec![97, 75, 47, 29, 13],
        ];
        for i in 0..updates.len() {
            assert_eq!(sort(&updates[i], &rules), results[i]);
        }
    }
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
