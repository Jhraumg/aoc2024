use rustc_hash::FxHashSet;
use std::str::FromStr;

advent_of_code::solution!(6);

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Dir {
    N,
    W,
    S,
    E,
}

#[derive(Clone)]
struct Area {
    width: usize,
    height: usize,
    obstacles: FxHashSet<Point>,
    guard: Point,
    gdir: Dir,
    visited: FxHashSet<Point>,
    visited_dir: FxHashSet<(Point, Dir)>,
}

impl FromStr for Area {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        let height = lines.len();
        let width = lines[0].len();

        let obstacles =
            lines
                .iter()
                .enumerate()
                .flat_map(|(y, l)| {
                    l.chars().enumerate().filter_map(move |(x, c)| {
                        if c == '#' {
                            Some(Point { x, y })
                        } else {
                            None
                        }
                    })
                })
                .collect();

        let (guard, gdir) = lines
            .iter()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars().enumerate().filter_map(move |(x, c)| match c {
                    '^' => Some((Point { x, y }, Dir::N)),
                    '>' => Some((Point { x, y }, Dir::E)),
                    '<' => Some((Point { x, y }, Dir::W)),
                    'v' => Some((Point { x, y }, Dir::S)),
                    _ => None,
                })
            })
            .next()
            .unwrap();
        let mut visited: FxHashSet<Point> = Default::default();
        visited.insert(guard);
        let mut visited_dir: FxHashSet<(Point, Dir)> = Default::default();
        visited_dir.insert((guard, gdir));

        Ok(Self {
            height,
            width,
            obstacles,
            guard,
            gdir,
            visited,
            visited_dir,
        })
    }
}

impl Area {
    fn next_move(&self) -> (Point, Dir) {
        let Point { x, y } = self.guard;
        match self.gdir {
            Dir::W => {
                let nextp = Point { x: x - 1, y };
                if self.obstacles.contains(&nextp) {
                    (self.guard, Dir::N)
                } else {
                    (nextp, self.gdir)
                }
            }
            Dir::S => {
                let nextp = Point { x, y: y + 1 };
                if self.obstacles.contains(&nextp) {
                    (self.guard, Dir::W)
                } else {
                    (nextp, self.gdir)
                }
            }
            Dir::E => {
                let nextp = Point { x: x + 1, y };
                if self.obstacles.contains(&nextp) {
                    (self.guard, Dir::S)
                } else {
                    (nextp, self.gdir)
                }
            }
            Dir::N => {
                let nextp = Point { x, y: y - 1 };
                if self.obstacles.contains(&nextp) {
                    (self.guard, Dir::E)
                } else {
                    (nextp, self.gdir)
                }
            }
        }
    }
    // returns true if next tick would exit
    fn tick(&mut self, check_loops: bool) -> (bool, bool) {
        (self.guard, self.gdir) = self.next_move();

        debug_assert!(!self.obstacles.contains(&self.guard));
        debug_assert!(self.guard.x < self.width);
        debug_assert!(self.guard.y < self.height);

        if check_loops {
            if self.visited_dir.contains(&(self.guard, self.gdir)) {
                return (false, true);
            }
            self.visited_dir.insert((self.guard, self.gdir));
        } else {
            self.visited.insert(self.guard);
        }

        let will_exit = match self.gdir {
            Dir::N => self.guard.y == 0,
            Dir::W => self.guard.x == 0,
            Dir::S => self.guard.y == self.height - 1,
            Dir::E => self.guard.x == self.width - 1,
        };
        (will_exit, false)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut area = Area::from_str(input).unwrap();

    while !area.tick(false).0 {}

    Some(area.visited.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut area = Area::from_str(input).unwrap();

    let mut new_obstacles: FxHashSet<_> = Default::default();
    let mut count = 0;

    // for each tick, we try to replace the next move with an obstacle, then
    // see if it loops
    // we do it only if next move would lead to a never visited place
    loop {
        // loops must be checked for original area
        // to be available in new_area
        let (next_move, _) = area.next_move();

        if new_obstacles.insert(next_move) {
            // cloning area avoid restarting path from the beginning
            let mut new_area = area.clone();
            new_area.obstacles.insert(next_move);

            loop {
                let (will_exit, will_loop) = new_area.tick(true);
                if will_exit {
                    /*exit without loop*/
                    break;
                };
                if will_loop {
                    count += 1;
                    break;
                }
            }
        }
        if area.tick(true).0 {
            break;
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
