use crate::Direction::*;

use rustc_hash::{FxHashMap, FxHashSet};
advent_of_code::solution!(16);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    W,
    E,
    N,
    S,
}
impl Direction {
    pub fn left(&self) -> Direction {
        match self {
            W => S,
            E => N,
            N => W,
            S => E,
        }
    }
    pub fn right(&self) -> Direction {
        match self {
            W => N,
            E => S,
            N => E,
            S => W,
        }
    }
    fn next_post(&self, pos: (usize, usize)) -> (usize, usize) {
        let (x, y) = pos;
        match self {
            W => (x - 1, y),
            E => (x + 1, y),
            N => (x, y - 1),
            S => (x, y + 1),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Path {
    pos: (usize, usize),
    dir: Direction,
    end: (usize, usize),
}
impl Path {
    fn read(input: &str) -> Path {
        let mut start: (usize, usize) = (0, 0);
        let mut end: (usize, usize) = (0, 0);

        for (j, l) in input.lines().enumerate() {
            for (i, c) in l.chars().enumerate() {
                match c {
                    'S' => {
                        start = (i, j);
                    }
                    'E' => {
                        end = (i, j);
                    }
                    _ => {}
                }
            }
        }
        Path {
            pos: start,
            end,
            dir: E,
        }
    }

    fn next_move(&self, score: usize, maze: &FxHashSet<(usize, usize)>) -> Option<(Path, usize)> {
        if self.end == self.pos {
            return None;
        }

        let pos = self.dir.next_post(self.pos);

        if self.end != pos && !maze.contains(&pos) {
            return None;
        }
        Some((Path { pos, ..*self }, score + 1))
    }

    fn next_moves(
        &self,
        score: usize,
        maze: &FxHashSet<(usize, usize)>,
    ) -> [Option<(Path, usize)>; 3] {
        if self.end == self.pos {
            return [const { None }; 3];
        }
        [
            self.next_move(score, maze),
            Path {
                dir: self.dir.left(),
                ..*self
            }
            .next_move(score + 1000, maze),
            Path {
                dir: self.dir.right(),
                ..*self
            }
            .next_move(score + 1000, maze),
        ]
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut maze = FxHashSet::default();
    for (j, l) in input.lines().enumerate() {
        for (i, c) in l.chars().enumerate() {
            if c == '.' {
                maze.insert((i, j));
            }
        }
    }
    let maze = &maze;

    // ~ Dijkstra
    let mut best_moves: FxHashMap<Path, usize> = Default::default();
    let mut visited: FxHashSet<Path> = Default::default();
    best_moves.insert(Path::read(input), 0);
    let mut next_moves = best_moves.clone();

    while let Some((p, score)) = next_moves
        .iter()
        // .filter(|(p, _)| !visited.contains(p))
        .min_by(|(_, s1), (_, s2)| s1.cmp(s2))
    {
        let p = *p;
        let score = *score;
        visited.insert(p);
        next_moves.remove(&p);
        for (p, score) in p.next_moves(score, maze).iter().flatten() {
            if !visited.contains(p) {
                next_moves
                    .entry(*p)
                    .and_modify(|best| {
                        if *score < *best {
                            *best = *score;
                        }
                    })
                    .or_insert(*score);
            }
            best_moves
                .entry(*p)
                .and_modify(|best| {
                    if *score < *best {
                        *best = *score;
                    }
                })
                .or_insert(*score);
        }
    }

    best_moves
        .iter()
        .filter(|(p, _)| p.pos == p.end)
        .min_by(|(_, s1), (_, s2)| s1.cmp(s2))
        .map(|(_, score)| *score)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut maze = FxHashSet::default();
    for (j, l) in input.lines().enumerate() {
        for (i, c) in l.chars().enumerate() {
            if c == '.' {
                maze.insert((i, j));
            }
        }
    }
    let maze = &maze;

    // ~ Dijkstra
    let mut best_moves: FxHashMap<Path, (usize, FxHashSet<(usize, usize)>)> = Default::default();
    let mut visited: FxHashSet<Path> = Default::default();
    let init = Path::read(input);

    best_moves.insert(init, (0, FxHashSet::from_iter([init.pos])));
    let mut next_moves: FxHashMap<Path, usize> = Default::default();
    next_moves.insert(init, 0);

    while let Some((p, score)) = next_moves
        .iter()
        // .filter(|(p, _)| !visited.contains(*p))
        .min_by(|(_, s1), (_, s2)| s1.cmp(s2))
    {
        let p = *p;
        let score = *score;
        let on_path = best_moves.get(&p).cloned().unwrap().1;

        visited.insert(p);
        next_moves.remove(&p);
        for (p, score) in p.next_moves(score, maze).iter().flatten() {
            if !visited.contains(p) {
                next_moves
                    .entry(*p)
                    .and_modify(|best| {
                        if *score < *best {
                            *best = *score;
                        }
                    })
                    .or_insert(*score);
            }
            best_moves
                .entry(*p)
                .and_modify(|(best, on_path_previous)| {
                    if *score <= *best {
                        if *score < *best {
                            on_path_previous.drain();
                        }
                        *best = *score;
                        for op in on_path.iter() {
                            on_path_previous.insert(*op);
                        }
                        on_path_previous.insert(p.pos);
                    }
                })
                .or_insert({
                    let mut on_path = on_path.clone();
                    on_path.insert(p.pos);
                    (*score, on_path)
                });
        }
    }

    best_moves
        .iter()
        .filter(|(p, _)| p.pos == p.end)
        .min_by(|(_, (s1, _)), (_, (s2, _))| s1.cmp(s2))
        .map(|(_, (_, on_p))| on_p.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(64));
    }
}
