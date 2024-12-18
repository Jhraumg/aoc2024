use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::min;

advent_of_code::solution!(18);

pub fn read_blocks(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|l| {
            if let Some((x, y)) = l.split_once(',') {
                (x.parse().unwrap(), y.parse().unwrap())
            } else {
                unreachable!("no coordinate to read");
            }
        })
        .collect()
}
fn escape_memory(blocks: &[(usize, usize)], edge: usize, after: usize) -> Option<usize> {
    // FIXME : this is not a time rush !!
    let time_by_blocks: FxHashMap<(usize, usize), usize> = if after > 0 {
        blocks
            .iter()
            .enumerate()
            .filter(|(t, _)| *t < after)
            .map(|(_, coord)| (*coord, 0))
            .collect()
    } else {
        blocks
            .iter()
            .enumerate()
            .map(|(t, coord)| (*coord, t))
            .collect()
    };
    let mut best_moves: FxHashMap<(usize, usize), usize> = Default::default();

    let mut visited: FxHashSet<(usize, usize)> = Default::default();
    best_moves.insert((0, 0), 0);

    while let Some(((x, y), t)) = best_moves
        .iter()
        .filter(|(pos, _)| !visited.contains(*pos))
        .min_by(|(_, t1), (_, t2)| t1.cmp(t2))
    {
        let (x, y) = (*x, *y);
        let t = *t;
        if (x, y) == (edge, edge) {
            return Some(t);
        }
        for next_move in [
            if x > 0usize { Some((x - 1, y)) } else { None },
            if x < edge { Some((x + 1, y)) } else { None },
            if y > 0 { Some((x, y - 1)) } else { None },
            if y < edge { Some((x, y + 1)) } else { None },
        ]
        .into_iter()
        .flatten()
        .filter(|(x, y)| {
            time_by_blocks
                .get(&(*x, *y))
                .map(|tb| *tb > t + 1)
                .unwrap_or(true)
        }) {
            best_moves
                .entry(next_move)
                .and_modify(|pt| *pt = min(t + 1, *pt))
                .or_insert(t + 1);
        }
        visited.insert((x, y));
    }

    None
}
pub fn first_blocking(input: &str, edge: usize) -> (usize, usize) {
    let blocks = read_blocks(input);
    for after in 1.. {
        if escape_memory(&blocks, edge, after).is_none() {
            return blocks[after - 1];
        }
    }
    unreachable!();
}
pub fn part_one(input: &str) -> Option<usize> {
    let blocks: Vec<(usize, usize)> = read_blocks(input);
    escape_memory(&blocks, 70, 1024)
}

pub fn part_two(input: &str) -> Option<String> {
    Some(format!("{:?}", first_blocking(input, 70)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let blocks: Vec<(usize, usize)> =
            read_blocks(&advent_of_code::template::read_file("examples", DAY));
        let result = escape_memory(&blocks, 6, 12);
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = first_blocking(&advent_of_code::template::read_file("examples", DAY), 6);
        assert_eq!(result, (6, 1));
    }
}
