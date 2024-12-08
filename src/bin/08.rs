use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Antenna {
    val: u8,
    pos: Point,
}

fn get_antinodes(p1: &Point, p2: &Point) -> [Point; 2] {
    let v21 = Point {
        x: p1.x - p2.x,
        y: p1.y - p2.y,
    };

    [
        Point {
            x: p1.x + v21.x,
            y: p1.y + v21.y,
        },
        Point {
            x: p2.x - v21.x,
            y: p2.y - v21.y,
        },
    ]
}

fn read_antennas(input: &[&str]) -> Vec<Antenna> {
    input
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.bytes().enumerate().filter_map(move |(x, val)| {
                if val.is_ascii_alphanumeric() {
                    Some(Antenna {
                        val,
                        pos: Point {
                            x: x.try_into().unwrap(),
                            y: y.try_into().unwrap(),
                        },
                    })
                } else {
                    None
                }
            })
        })
        .collect()
}
pub fn part_one(input: &str) -> Option<usize> {
    let input: Vec<&str> = input.lines().collect();
    let rows: isize = input.len().try_into().unwrap();
    let cols: isize = input[0].len().try_into().unwrap();

    let antennas = read_antennas(&input);

    let antinodes: HashSet<_> = antennas
        .iter()
        .combinations(2)
        .filter(|antens| antens[0].val == antens[1].val)
        .flat_map(|antens| get_antinodes(&antens[0].pos, &antens[1].pos).into_iter())
        .filter(|antinode| {
            antinode.x >= 0 && antinode.x < rows && antinode.y >= 0 && antinode.y < cols
        })
        .collect();
    Some(antinodes.len())
}

fn is_antinode(p: &Point, pos_by_val: &HashMap<u8, Vec<Point>>) -> bool {
    pos_by_val
        .iter()
        .find(|(_, positions)| positions.contains(p))
        .map(|(_, positions)| positions.len() >= 3)
        .unwrap_or_else(|| {
            pos_by_val.iter().any(|(_, positions)| {
                positions
                    .iter()
                    .filter(|pos| **pos != *p)
                    .combinations(2)
                    .any(|positions| {
                        let [p1, p2] = positions[0..2] else {
                            unreachable!("combinations of 2 pos")
                        };
                        let v1 = Point {
                            x: p1.x - p.x,
                            y: p1.y - p.y,
                        };
                        let v2 = Point {
                            x: p2.x - p.x,
                            y: p2.y - p.y,
                        };
                        (v1.x * v2.y) - (v1.y * v2.x) == 0
                    })
            })
        })
}

pub fn part_two(input: &str) -> Option<usize> {
    let input: Vec<&str> = input.lines().collect();
    let rows: isize = input.len().try_into().unwrap();
    let cols: isize = input[0].len().try_into().unwrap();

    let antennas = read_antennas(&input);
    let positions_by_val =
        antennas
            .iter()
            .fold(HashMap::<u8, Vec<Point>>::new(), |mut acc, antenna| {
                acc.entry(antenna.val)
                    .and_modify(|positions| positions.push(antenna.pos))
                    .or_insert(vec![antenna.pos]);
                acc
            });

    Some(
        (0..cols)
            .map(|x| {
                (0..rows)
                    .map(|y| Point { x, y })
                    .filter(|p| is_antinode(p, &positions_by_val))
                    .count()
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
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
