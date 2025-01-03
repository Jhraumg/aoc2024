use crossterm::style::Color::DarkGreen;
use crossterm::style::SetForegroundColor;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{ExecutableCommand, QueueableCommand};
use itertools::Itertools;
use rustc_hash::FxHashSet;
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;

advent_of_code::solution!(14);

struct Robot {
    x: isize,
    y: isize,
    vx: isize,
    vy: isize,
}

impl Robot {
    pub fn new(l: &str) -> Robot {
        let (p, v) = l.split_whitespace().collect_tuple().unwrap();
        assert!(p.starts_with("p="));
        assert!(v.starts_with("v="));
        let (x, y) = p
            .split(['=', ','])
            .skip(1)
            .map(|c| c.trim().parse::<isize>().unwrap())
            .collect_tuple()
            .unwrap();
        let (vx, vy) = v
            .split(['=', ','])
            .skip(1)
            .map(|cv| cv.trim().parse::<isize>().unwrap())
            .collect_tuple()
            .unwrap();
        Self { x, y, vx, vy }
    }
    pub fn pos_after(&self, times: isize, w: isize, h: isize) -> (isize, isize) {
        let &Self { x, y, vx, vy } = self;
        (
            (w + (x + times * vx) % w) % w,
            (h + (y + times * vy) % h) % h,
        )
    }
}

pub fn safety_factor(input: &str, times: isize, w: isize, h: isize) -> isize {
    let robots: Vec<Robot> = input.lines().map(Robot::new).collect();
    let pos = robots
        .into_iter()
        .map(|r| r.pos_after(times, w, h))
        .collect_vec();
    let (nw, ne, sw, se) = pos.iter().fold((0, 0, 0, 0), |(nw, ne, sw, se), (x, y)| {
        let (x, y) = (*x, *y);
        (
            nw + if x < w / 2 && y < h / 2 { 1 } else { 0 },
            ne + if x > w / 2 && y < h / 2 { 1 } else { 0 },
            sw + if x < w / 2 && y > h / 2 { 1 } else { 0 },
            se + if x > w / 2 && y > h / 2 { 1 } else { 0 },
        )
    });
    nw * ne * sw * se
}

const W: isize = 101;
const H: isize = 103;

pub fn part_one(input: &str) -> Option<usize> {
    Some(safety_factor(input, 100, W, H) as usize)
}

// To be lowered if no image comes out
const EDGE_LEN: usize = 4;

pub fn maybe_christmas_tree(pos: &[(isize, isize)]) -> bool {
    // let's look for some \ edges ?
    let pos: FxHashSet<(isize, isize)> = pos.iter().copied().collect();

    // looking for    #
    // a triangle    # #
    // shape        #   #
    let triangle_found = pos.iter().any(|(x, y)| {
        (1..EDGE_LEN as isize)
            .all(|i| pos.contains(&(*x + i, *y + i)) && pos.contains(&(*x - i, *y + i)))
    });
    triangle_found
}

// set to false when filter is selective enough
const DISPLAY_ON_TERM: bool = false;
pub fn part_two(input: &str) -> Option<usize> {
    let robots: Vec<Robot> = input.lines().map(Robot::new).collect();

    let mut display = stdout();

    let mut tries = 0;

    for t in 0..=H * W {
        let pos = robots.iter().map(|r| r.pos_after(t, W, H)).collect_vec();

        if maybe_christmas_tree(&pos) {
            tries += 1;
            if DISPLAY_ON_TERM {
                display.execute(Clear(ClearType::All)).unwrap();
                display.execute(SetForegroundColor(DarkGreen)).unwrap();
                for (x, y) in &pos {
                    display
                        .queue(crossterm::cursor::MoveTo(*x as u16, *y as u16))
                        .unwrap()
                        .queue(crossterm::style::Print('#'))
                        .unwrap();
                }
                display
                    .queue(crossterm::cursor::MoveTo(0, 1 + H as u16))
                    .unwrap()
                    .queue(crossterm::style::Print(format!("time: {}", t)))
                    .unwrap();

                display
                    .queue(crossterm::cursor::MoveTo(20, 1 + H as u16))
                    .unwrap()
                    .queue(crossterm::style::Print(format!("tries: {}", tries)))
                    .unwrap();

                display.flush().unwrap();
                sleep(Duration::from_millis(500));
            } else {
                //
                return Some(t.try_into().unwrap());
            }
        }
    }
    unreachable!("All value seen");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = safety_factor(
            &advent_of_code::template::read_file("examples", DAY),
            100,
            11,
            7,
        );
        assert_eq!(result, 12);
    }

    // #[test]
    // fn test_part_two() {
    //     // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     // assert_eq!(result, None);
    // }
}
