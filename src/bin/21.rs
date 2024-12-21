use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

advent_of_code::solution!(21);

const KEYB_KEYS: &'static str = "A<>^v";

struct Keyboard {
    keys: FxHashMap<(usize, usize), char>,
    init: (usize, usize),
}
impl Keyboard {
    fn doors_keyboard() -> Self {
        let keys = FxHashMap::from_iter(
            [
                ['7', '8', '9'],
                ['4', '5', '6'],
                ['1', '2', '3'],
                ['#', '0', 'A'],
            ]
            .iter()
            .enumerate()
            .flat_map(move |(j, l)| l.into_iter().enumerate().map(move |(i, c)| ((i, j), *c)))
            .filter(|(_, c)| '#' != *c),
        );

        Self { keys, init: (2, 3) }
    }

    fn robot_keyboard() -> Self {
        let keys = FxHashMap::from_iter(
            [['#', '^', 'A'], ['<', 'v', '>']]
                .iter()
                .enumerate()
                .flat_map(move |(j, l)| l.into_iter().enumerate().map(move |(i, c)| ((i, j), *c)))
                .filter(|(_, c)| '#' != *c),
        );

        Self { keys, init: (2, 0) }
    }
    fn get_key(&self, key: char) -> (usize, usize) {
        *self.keys.iter().find(|(_, c)| **c == key).unwrap().0
    }
    fn convert(&self, seq: &str) -> String {
        let mut result: Vec<char> = Vec::new();
        let mut current = self.init;
        for (i, c) in seq.chars().enumerate() {
            assert!(
                self.keys.contains_key(&current),
                "invalid seq at {} ({})",
                seq,
                &seq[0..i + 1]
            );
            match c {
                '<' => {
                    current.0 -= 1;
                }
                '>' => {
                    current.0 += 1;
                }
                '^' => {
                    current.1 -= 1;
                }
                'v' => {
                    current.1 += 1;
                }
                'A' => result.push(self.keys[&current]),
                _ => {
                    unreachable!()
                }
            }
        }

        String::from_iter(result)
    }

    fn possible_moves(&self, init: (usize, usize), end: (usize, usize)) -> FxHashSet<String> {
        let (ix, iy) = (init.0 as isize, init.1 as isize);
        let (ex, ey) = (end.0 as isize, end.1 as isize);
        let (dx, dy) = (ex - ix, ey - iy);
        let vmove = if dy > 0 { 'v' } else { '^' };
        let hmove = if dx > 0 { '>' } else { '<' };

        if dx == 0 {
            return FxHashSet::from_iter(vec![vmove.to_string().repeat(dy.abs() as usize)]);
        }
        if dy == 0 {
            return FxHashSet::from_iter(vec![hmove.to_string().repeat(dx.abs() as usize)]);
        }

        let mut result: FxHashSet<((isize, isize), String)> = FxHashSet::from_iter(vec![(
            (ix, iy),
            "".to_string(),
        )]);
        let len = dx.abs() + dy.abs();
        for _ in 0..len {
            let new_result: FxHashSet<((isize, isize), String)> = result
                .iter()
                .flat_map(|(pos, path)| {
                    let (x, y) = pos;
                    [(*x + dx.signum(), *y), (*x, *y + dy.signum())]
                        .into_iter()
                        .filter(|(nx, ny)| *nx >= 0 && *ny >= 0)
                        .filter(|(nx, ny)| self.keys.contains_key(&(*nx as usize, *ny as usize)))
                        .map(move |(nx, ny)| {
                            let mut p = path.clone();
                            p.push(if nx != *x { hmove } else { vmove });
                            // println!("({x},{y}) to ({nx},{ny}) => {p}");
                            ((nx, ny), p)
                        })
                })
                .collect();
            result = new_result;
        }

        // Nawak
        result
            .into_iter()
            .filter(|((nx, ny), _)| (*nx as usize, *ny as usize) == end)
            .map(|(_, path)| path)
            .collect()
    }

    fn robot_code_seqs(&self, code: &str) -> FxHashSet<String> {
        let mut result: FxHashSet<String> = Default::default();
        result.insert("".to_string());

        let mut current = self.init;
        for c in code.chars() {
            let new_pos = self.get_key(c);
            let mut new_result: FxHashSet<String> = Default::default();
            for possible_move in self.possible_moves(current, new_pos) {
                for r in &result {
                    new_result.insert(r.clone() + &possible_move + "A");
                }
            }
            result = new_result;
            current = new_pos;
        }

        result
    }

    // fn shortest_seqs(&self, depth: usize) -> FxHashMap<(String, usize), FxHashSet<String>> {
    //     let mut result: FxHashMap<(String, usize), FxHashSet<String>> = Default::default();
    //
    //     for c in KEYB_KEYS.chars() {
    //         result.insert(
    //             (c.to_string(), 1),
    //             FxHashSet::from_iter(
    //                 match c {
    //                     'A' => vec!["A"],
    //                     '^' => vec!["<A>A"],
    //                     '>' => vec!["vA^A"],
    //                     '<' => vec!["<v<A>^>A", "v<<A>^>A", "<v<A>>^A", "v<<A>>^A"],
    //                     'v' => vec!["<vA>^A", "v<A>^A", "<vA^>A", "v<A^>A"],
    //                     _ => unreachable!(),
    //                 }
    //                 .into_iter()
    //                 .map(str::to_string),
    //             ),
    //         );
    //     }
    //
    //     for i in 2..=depth {
    //         for c in KEYB_KEYS.chars() {
    //             let seqs = result.get(&(c.to_string(), i - 1)).unwrap().clone();
    //             for seq in seqs.iter() {
    //                 let bits: Vec<_> = seq
    //                     .chars()
    //                     .filter_map(|tc| result.get(&(tc.to_string(), 1)))
    //                     .collect();
    //                 let mut acc: FxHashSet<String> = bits[0].clone();
    //                 for bit in bits.iter().skip(1) {
    //                     let mut new_bits: FxHashSet<String> = Default::default();
    //                     for prefix in &acc {
    //                         for suffix in bit.iter() {
    //                             new_bits.insert(prefix.to_string() + suffix);
    //                         }
    //                     }
    //                     acc = new_bits;
    //                 }
    //                 result.insert((c.to_string(), i), acc);
    //             }
    //         }
    //     }
    //     // for (k,v) in &result {
    //     //     println!("{k:?} => {v:?}");
    //     // }
    //
    //     result
    // }
}

fn input_code(code: &str) -> String {
    let door_kb = Keyboard::doors_keyboard();
    let robot_kb = Keyboard::robot_keyboard();

    // let shortests = robot_kb.shortest_seqs(2);

    let door_seqs = door_kb.robot_code_seqs(code);

    let robot1_seqs:FxHashSet<String> = door_seqs
        .into_iter()
        .flat_map(|seq|
            robot_kb.robot_code_seqs(&seq).into_iter()
        ).collect();

    let robot2_seqs:FxHashSet<String> = robot1_seqs
        .into_iter()
        .flat_map(|seq|
            robot_kb.robot_code_seqs(&seq).into_iter()
        ).collect();

    robot2_seqs.into_iter().min_by(|s1,s2|s1.len().cmp(&s2.len())).unwrap()
}

fn complexity(code: &str, seq: &str) -> usize {
    let code = code.trim();
    let first_non_digit = code.find(|c: char| !c.is_digit(10)).unwrap_or(code.len());
    let code = code[0..first_non_digit].parse::<usize>().unwrap();

    println!("{} * {code}", seq.trim().len());
    code * seq.trim().len()
}
pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|code| complexity(code, &input_code(code)))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // assert_eq!(29, "029".parse().unwrap());
        // assert_eq!(
        //     complexity(
        //         "029A",
        //         "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A"
        //     ),
        //     68 * 29
        // );
        //
        assert_eq!(complexity(&"029A", &input_code("029A")), 68 * 29);

        // assert_eq!(
        //     complexity(&"179A",&input_code("179A")), 68 * 179);
        // assert_eq!(
        //     complexity(&"379A",&input_code("379A")), 64 * 379);
        //
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
