use std::collections::HashSet;

advent_of_code::solution!(15);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct WareHouse<const WIDE: bool> {
    boxes: HashSet<(isize, isize)>,
    walls: HashSet<(isize, isize)>,
    robot: (isize, isize),
    instrs: Vec<Direction>,
}

impl<const WIDE: bool> WareHouse<WIDE> {
    pub fn read(input: &str) -> Self {
        let mut boxes: HashSet<(isize, isize)> = HashSet::new();
        let mut walls: HashSet<(isize, isize)> = HashSet::new();
        let mut robot: (isize, isize) = (0, 0);
        let mut instrs: Vec<Direction> = vec![];
        let w_factor = if WIDE { 2 } else { 1 };

        for (i, j, c) in input
            .lines()
            .enumerate()
            .flat_map(|(j, l)| l.chars().enumerate().map(move |(i, c)| (i, j, c)))
        {
            match c {
                '#' => {
                    walls.insert((w_factor * i as isize, j as isize));
                    if WIDE {
                        walls.insert((w_factor * i as isize + 1, j as isize));
                    }
                }
                '@' => {
                    robot = (w_factor * i as isize, j as isize);
                }
                'O' => {
                    boxes.insert((w_factor * i as isize, j as isize));
                }
                '<' => {
                    instrs.push(Direction::Left);
                }
                '>' => {
                    instrs.push(Direction::Right);
                }
                '^' => {
                    instrs.push(Direction::Up);
                }
                'v' => {
                    instrs.push(Direction::Down);
                }
                _ => {}
            }
        }

        Self {
            boxes,
            walls,
            robot,
            instrs,
        }
    }

    pub fn gps(&self) -> usize {
        self.boxes
            .iter()
            .map(|&(i, j)| i as usize + 100 * j as usize)
            .sum()
    }

    fn tick_narrow(&mut self, instr: Direction) {
        let instr = match instr {
            Direction::Left => |(x, y)| (x - 1, y),
            Direction::Right => |(x, y)| (x + 1, y),
            Direction::Up => |(x, y)| (x, y - 1),
            Direction::Down => |(x, y)| (x, y + 1),
        };

        let mut pos = self.robot;

        loop {
            pos = instr(pos);
            if self.walls.contains(&pos) {
                return;
            }
            if !self.boxes.contains(&pos) {
                break;
            }
        }
        self.robot = instr(self.robot);
        if self.boxes.contains(&self.robot) {
            // snake move (tail -> new head)
            self.boxes.remove(&self.robot);
            self.boxes.insert(pos);
        }
    }
    fn tick_wide(&mut self, d: Direction) {
        let instr = match d {
            Direction::Left => |(x, y)| (x - 1, y),
            Direction::Right => |(x, y)| (x + 1, y),
            Direction::Up => |(x, y)| (x, y - 1),
            Direction::Down => |(x, y)| (x, y + 1),
        };

        let mut moved_boxes: HashSet<(isize, isize)> = HashSet::new();
        let mut front: HashSet<(isize, isize)> = HashSet::new();
        front.insert(instr(self.robot));

        loop {
            if front.iter().any(|p| self.walls.contains(p)) {
                return;
            }

            let new_moved_boxes: Vec<_> = front
                .iter()
                .filter_map(|&(x, y)| self.boxes.get(&(x, y)).or(self.boxes.get(&(x - 1, y))))
                .filter(|nmb| !moved_boxes.contains(nmb))
                .cloned()
                .collect();

            if new_moved_boxes.is_empty() {
                break;
            }
            front = new_moved_boxes
                .iter()
                .flat_map(move |&(x, y)| [instr((x, y)), instr((x + 1, y))].into_iter())
                .collect();
            moved_boxes.extend(new_moved_boxes);
        }

        // here, actual move
        self.robot = instr(self.robot);
        for mb in &moved_boxes {
            self.boxes.remove(mb);
        }
        for mb in moved_boxes {
            self.boxes.insert(instr(mb));
        }
    }

    pub fn compute(&mut self) {
        let instrs = self.instrs.clone();
        for d in instrs {
            if WIDE {
                self.tick_wide(d);
            } else {
                self.tick_narrow(d);
            }
        }
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        let mut w = 0;
        let mut h = 0;
        for (x, y) in &self.walls {
            if *x > w {
                w = *x;
            }
            if *y > h {
                h = *y;
            }
        }
        println!();
        for j in 0..=h {
            print!("{:3} ", j);
            for i in 0..=w {
                if self.walls.contains(&(i, j)) {
                    print!("#");
                } else if self.boxes.contains(&(i, j)) {
                    print!("{}", if WIDE { '[' } else { 'O' });
                } else if WIDE && self.boxes.contains(&(i - 1, j)) {
                    print!("]");
                } else if self.robot == (i, j) {
                    print!("@");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut warehouse = WareHouse::<false>::read(input);
    warehouse.compute();

    Some(warehouse.gps())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut warehouse = WareHouse::<true>::read(input);
    warehouse.compute();

    Some(warehouse.gps())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2028));

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
