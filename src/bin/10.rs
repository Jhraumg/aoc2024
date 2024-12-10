use std::collections::{HashMap, HashSet};

advent_of_code::solution!(10);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

pub fn part_one(input: &str) -> Option<usize> {
    let map: HashMap<Point, usize> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| (Point { x, y }, c.to_digit(10).unwrap() as usize))
        })
        .collect();

    Some(
        map.iter()
            .filter(|(_, h)| **h == 0)
            .map(|(start, _)| {
                let mut current_pos: HashSet<Point> = HashSet::new();
                current_pos.insert(*start);
                for h in 1..10 {
                    let new_pos: HashSet<Point> = current_pos
                        .iter()
                        .flat_map(|&Point { x, y }| {
                            [
                                if x > 0 {
                                    Some(Point { x: x - 1, y })
                                } else {
                                    None
                                },
                                if y > 0 {
                                    Some(Point { x, y: y - 1 })
                                } else {
                                    None
                                },
                                Some(Point { x: x + 1, y }),
                                Some(Point { x, y: y + 1 }),
                            ]
                            .into_iter()
                            .flatten()
                            .filter(|p| map.get(p).filter(|nh| **nh == h).is_some())
                        })
                        .collect();
                    current_pos = new_pos;
                }

                current_pos.len()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let map: HashMap<Point, usize> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| (Point { x, y }, c.to_digit(10).unwrap() as usize))
        })
        .collect();

    Some(
        map.iter()
            .filter(|(_, h)| **h == 0)
            .map(|(start, _)| {
                let mut current_trails: HashSet<Vec<Point>> = HashSet::new();
                current_trails.insert(vec![*start]);
                for h in 1..10 {
                    let new_trails: HashSet<Vec<Point>> = current_trails
                        .iter()
                        .flat_map(|t| {
                            let &Point { x, y } = t.last().unwrap();
                            [
                                if x > 0 {
                                    Some(Point { x: x - 1, y })
                                } else {
                                    None
                                },
                                if y > 0 {
                                    Some(Point { x, y: y - 1 })
                                } else {
                                    None
                                },
                                Some(Point { x: x + 1, y }),
                                Some(Point { x, y: y + 1 }),
                            ]
                            .into_iter()
                            .flatten()
                            .filter(|p| map.get(p).filter(|nh| **nh == h).is_some())
                            .map(|p| {
                                let mut new_t = t.clone();
                                new_t.push(p);
                                new_t
                            })
                        })
                        .collect();
                    current_trails = new_trails;
                }

                current_trails.len()
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
