use itertools::Itertools;
use std::collections::{HashSet};

advent_of_code::solution!(8);

#[derive(Debug, Eq, PartialEq, Hash)]
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

    // todo : map val -> Vec<Antenna>
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

fn is_antinode(p: &Point, antennas: &[Antenna]) -> bool {
    let result = antennas
        .iter()
        .find(|a| a.pos == *p)
        .map(|a| antennas.iter().filter(|&a1| a1.val == a.val).count() >= 3)
        .unwrap_or_else(|| {
            antennas.iter().filter(|a| a.pos != *p).any(|a1| {
                antennas.iter().any(|a2| {
                    a2.pos != *p && a2.val == a1.val && a2.pos != a1.pos && {
                        let v1 = Point {
                            x: a1.pos.x - p.x,
                            y: a1.pos.y - p.y,
                        };
                        let v2 = Point {
                            x: a2.pos.x - p.x,
                            y: a2.pos.y - p.y,
                        };
                        (v1.x * v2.y) - (v1.y * v2.x) == 0
                    }
                })
            })
        });
    result
}

pub fn part_two(input: &str) -> Option<usize> {
    let input: Vec<&str> = input.lines().collect();
    let rows: isize = input.len().try_into().unwrap();
    let cols: isize = input[0].len().try_into().unwrap();

    let antennas = read_antennas(&input);

    Some(
        (0..cols)
            .map(|x| {
                (0..rows)
                    .map(|y| Point { x, y })
                    .filter(|p| is_antinode(p, &antennas))
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
