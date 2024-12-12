use std::collections::{HashMap, HashSet};

advent_of_code::solution!(12);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn neighbors(&self) -> [Point; 4] {
        let &Point { x, y } = self;
        [
            Point { x: x - 1, y },
            Point { x: x + 1, y },
            Point { x, y: y - 1 },
            Point { x, y: y + 1 },
        ]
    }
}

fn cost(area: &HashSet<Point>) -> usize {
    let surface = area.len();
    let perimeter = {
        area.iter()
            .map(|&p| {
                p.neighbors()
                    .into_iter()
                    .filter(|n| !area.contains(n))
                    .count()
            })
            .sum::<usize>()
    };
    surface * perimeter
}

fn reduced_cost(area: &HashSet<Point>) -> usize {
    let surface = area.len();

    let perimeter = {
        area.iter()
            .map(|&Point { x, y }| {
                let mut p = 0usize;
                //left
                if !area.contains(&Point { x: x - 1, y })
                    && (!area.contains(&Point { x, y: y - 1 })
                        || area.contains(&Point { x: x - 1, y: y - 1 }))
                {
                    p += 1
                }
                //right
                if (!area.contains(&Point { x: x + 1, y }))
                    && (!area.contains(&Point { x, y: y - 1 })
                        || area.contains(&Point { x: x + 1, y: y - 1 }))
                {
                    p += 1
                }

                //up
                if !area.contains(&Point { x, y: y - 1 })
                    && (!area.contains(&Point { x: x - 1, y })
                        || area.contains(&Point { x: x - 1, y: y - 1 }))
                {
                    p += 1
                }
                //down
                if (!area.contains(&Point { x, y: y + 1 }))
                    && (!area.contains(&Point { x: x - 1, y })
                        || area.contains(&Point { x: x - 1, y: y + 1 }))
                {
                    p += 1
                }

                p
            })
            .sum::<usize>()
    };
    surface * perimeter
}

fn collect_areas(input: &str) -> Vec<HashSet<Point>> {
    let mut plots: HashMap<Point, char> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().map(move |(x, sort)| {
                (
                    Point {
                        x: x as isize,
                        y: y as isize,
                    },
                    sort,
                )
            })
        })
        .collect();

    let mut areas: Vec<HashSet<Point>> = vec![];
    let mut current_sort = plots.remove(&Point { x: 0, y: 0 }).unwrap();
    let mut current_area: HashSet<Point> = HashSet::new();
    current_area.insert(Point { x: 0, y: 0 });
    while !plots.is_empty() {
        let mut neighbors: Vec<Point> = vec![];

        for p in &current_area {
            for neighbour in p
                .neighbors()
                .iter()
                .filter(|n| plots.get(*n).map(|c| *c == current_sort).unwrap_or(false))
            {
                neighbors.push(*neighbour)
            }
        }

        for n in &neighbors {
            plots.remove(n);
        }
        let no_new_points = neighbors.is_empty();
        current_area.extend(neighbors);

        if no_new_points {
            areas.push(current_area.clone());

            if let Some((p, c)) = plots.iter().next() {
                current_area = HashSet::new();
                let p = *p;
                current_area.insert(p);
                current_sort = *c;
                plots.remove(&p);
            }
        }
    }
    if !current_area.is_empty() {
        areas.push(current_area);
    }

    areas
}

pub fn part_one(input: &str) -> Option<usize> {
    let areas = collect_areas(input);

    Some(areas.iter().map(cost).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let areas = collect_areas(input);

    Some(areas.iter().map(reduced_cost).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
