use itertools::Itertools;

advent_of_code::solution!(13);

#[derive(Debug, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone)]
struct Button {
    cost: usize,
    mv: Point,
}

fn read_button(line: &str) -> Button {
    debug_assert!(line.starts_with("Button "), "{line}");
    let cost = match line.as_bytes()["Button ".len()] {
        b'A' => 3,
        b'B' => 1,
        _ => unreachable!("Unknown button"),
    };
    let x = line[line.find("X+").unwrap() + 2..]
        .split(',')
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let y = line[line.find("Y+").unwrap() + 2..]
        .trim()
        .parse::<usize>()
        .unwrap();
    let mv = Point { x, y };
    Button { cost, mv }
}
fn read_prize(line: &str) -> Point {
    debug_assert!(line.starts_with("Prize:"));

    let x = line[line.find("X=").unwrap() + 2..]
        .split(',')
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let y = line[line.find("Y=").unwrap() + 2..]
        .trim()
        .parse::<usize>()
        .unwrap();
    Point { x, y }
}

struct Game {
    a: Button,
    b: Button,
    prize: Point,
    max_push: usize,
}

impl Game {
    pub fn min_cost(&self) -> Option<usize> {
        let &Self {
            a,
            b,
            prize,
            max_push,
        } = self;
        let ax = a.mv.x as isize;
        let ay = a.mv.y as isize;
        let bx = b.mv.x as isize;
        let by = b.mv.y as isize;
        let px = prize.x as isize;
        let py = prize.y as isize;
        let max = max_push as isize;
        if by * ax - bx * ay != 0 {
            let nb_a = (by * px - bx * py) / (by * ax - bx * ay);
            let nb_b = if bx != 0 {
                (px - nb_a * ax) / bx
            } else {
                (py - nb_a * ay) / by
            };
            if (0..=max).contains(&nb_a)
                && (0..=max).contains(&nb_b)
                && (px == nb_a * ax + nb_b * bx)
                && (py == nb_a * ay + nb_b * by)
            {
                return Some((nb_a as usize) * a.cost + (nb_b as usize) * b.cost);
            } else {
                return None;
            }
        } else if px * by - py * bx != 0 {
            // no solution
            return None;
        }
        unreachable!("let's hope we don't have to resolve infinite solutions case");
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    let games: Vec<Game> = input
        .lines()
        .chunks(4)
        .into_iter()
        .map(|mut v| {
            let a = read_button(v.next().unwrap());
            let b = read_button(v.next().unwrap());
            let prize = read_prize(v.next().unwrap());
            Game {
                a,
                b,
                prize,
                max_push: 100,
            }
        })
        .collect();

    Some(games.iter().filter_map(|g| g.min_cost()).sum::<usize>())
}

pub fn part_two(input: &str) -> Option<usize> {
    let games: Vec<Game> = input
        .lines()
        .chunks(4)
        .into_iter()
        .map(|mut v| {
            let a = read_button(v.next().unwrap());
            let b = read_button(v.next().unwrap());
            let prize = read_prize(v.next().unwrap());
            let prize = Point {
                x: prize.x + 10000000000000,
                y: prize.y + 10000000000000,
            };
            Game {
                a,
                b,
                prize,
                max_push: isize::MAX as usize,
            }
        })
        .collect();

    Some(games.iter().filter_map(|g| g.min_cost()).sum::<usize>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }
}
