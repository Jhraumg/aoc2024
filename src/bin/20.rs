use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::min;

advent_of_code::solution!(20);

#[derive(Debug, Clone)]
struct Track {
    s: (usize, usize),
    e: (usize, usize),
    pos: FxHashSet<(usize, usize)>,
}
fn read_track(input: &str) -> Track {
    let mut s: (usize, usize) = (0, 0);
    let mut e: (usize, usize) = (0, 0);

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == 'E' {
                e = (x, y);
            }
            if c == 'S' {
                s = (x, y);
            }
        }
    }

    let pos: FxHashSet<_> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter_map(move |(x, c)| if c == '.' { Some((x, y)) } else { None })
        })
        .collect();
    Track { s, e, pos }
}
impl Track {
    pub fn possible_cheats(
        &self,
        pos: (usize, usize),
        len: usize,
    ) -> FxHashMap<(usize, usize), usize> {
        let mut result: FxHashMap<(usize, usize), usize> = Default::default();
        let (x, y) = pos;

        let mut cheets: FxHashSet<(usize, usize)> =
            [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                .into_iter()
                //.filter(|p| !self.pos.contains(p)) // you can cheet while keeping on track !!
                .collect();
        for t in 1..len {
            let mut new_in: FxHashSet<(usize, usize)> = Default::default();
            let mut new_cheets: FxHashSet<(usize, usize)> = Default::default();
            for p in cheets.iter().flat_map(|previous_cheet| {
                let (x, y) = *previous_cheet;
                [
                    if x > 0 { Some((x - 1, y)) } else { None },
                    Some((x + 1, y)),
                    if y > 0 { Some((x, y - 1)) } else { None },
                    Some((x, y + 1)),
                ]
                .into_iter()
                .flatten()
            }) {
                if self.pos.contains(&p) || self.e == p || self.s == p {
                    new_in.insert(p);
                }

                new_cheets.insert(p);
            }
            for ni in new_in.into_iter() {
                result.entry(ni).or_insert(t + 1);
            }
            //else {
            cheets = new_cheets;
            //}
        }

        result.remove(&pos);

        result
    }
}

fn get_distance_from_end(input: &str) -> FxHashMap<(usize, usize), usize> {
    let track = read_track(input);

    let mut result: FxHashMap<(usize, usize), usize> = FxHashMap::default();
    result.insert(track.e, 0);
    let mut visited: FxHashSet<(usize, usize)> = Default::default();

    // FIXME : just
    while let Some(((x, y), t)) = result
        .iter()
        .filter(|(pos, _)| !visited.contains(*pos))
        .max_by(|(_, t1), (_, t2)| t1.cmp(t2))
    {
        let (x, y, t) = (*x, *y, *t);

        for next_move in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
            .into_iter()
            .filter(|p| track.pos.contains(p) || track.e == *p || track.s == *p)
        {
            result
                .entry(next_move)
                .and_modify(|pt| *pt = min(t + 1, *pt))
                .or_insert(t + 1);
        }
        visited.insert((x, y));
    }

    result
}

pub fn count_shortcuts_over(input: &str, shortcut: usize, cheet_len: usize) -> usize {
    let track = read_track(input);
    let distance_from_end = get_distance_from_end(input); // FIXME : read input only once

    distance_from_end
        .iter()
        .filter(|(_, d)| **d > shortcut)
        .map(|(pos, t)| {
            track
                .possible_cheats(*pos, cheet_len)
                .into_iter()
                .filter(|(cheet, d)| {
                    distance_from_end
                        .get(cheet)
                        .filter(|cd| shortcut + **cd + *d <= *t)
                        .is_some()
                })
                .count()
        })
        .sum()
}
pub fn part_one(input: &str) -> Option<usize> {
    Some(count_shortcuts_over(input, 100, 2))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(count_shortcuts_over(input, 100, 20))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(
            count_shortcuts_over(&advent_of_code::template::read_file("examples", DAY), 64, 2),
            1
        );
        assert_eq!(
            count_shortcuts_over(&advent_of_code::template::read_file("examples", DAY), 2, 2),
            44
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            count_shortcuts_over(
                &advent_of_code::template::read_file("examples", DAY),
                76,
                20
            ),
            3
        );
        assert_eq!(
            count_shortcuts_over(
                &advent_of_code::template::read_file("examples", DAY),
                72,
                20
            ),
            29
        );
    }
}
