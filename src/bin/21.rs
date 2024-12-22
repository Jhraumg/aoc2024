use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::HashMap;

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

        let vert_first_possible = (1..=dy.abs()).all(|y| {
            self.keys
                .contains_key(&(ix as usize, (iy + y * dy.signum()) as usize))
        });
        let hz_first_possible = (1..=dx.abs()).all(|x| {
            self.keys
                .contains_key(&((ix + x * dx.signum()) as usize, iy as usize))
        });
        [
            if vert_first_possible {
                Some(
                    vmove.to_string().repeat(dy.abs() as usize)
                        + &hmove.to_string().repeat(dx.abs() as usize),
                )
            } else {
                None
            },
            if hz_first_possible {
                Some(
                    hmove.to_string().repeat(dx.abs() as usize)
                        + &vmove.to_string().repeat(dy.abs() as usize),
                )
            } else {
                None
            },
        ]
        .into_iter()
        .flatten()
        .unique_by(|seq| seq.replace("^", ">"))
        .collect()

        // let mut result: FxHashSet<((isize, isize), String)> =
        //     FxHashSet::from_iter(vec![((ix, iy), "".to_string())]);
        // let len = dx.abs() + dy.abs();
        // for _ in 0..len {
        //     let new_result: FxHashSet<((isize, isize), String)> = result
        //         .iter()
        //         .flat_map(|(pos, path)| {
        //             let (x, y) = pos;
        //             [(*x + dx.signum(), *y), (*x, *y + dy.signum())]
        //                 .into_iter()
        //                 .filter(|(nx, ny)| *nx >= 0 && *ny >= 0)
        //                 .filter(|(nx, ny)| self.keys.contains_key(&(*nx as usize, *ny as usize)))
        //                 .map(move |(nx, ny)| {
        //                     let mut p = path.clone();
        //                     p.push(if nx != *x { hmove } else { vmove });
        //                     // println!("({x},{y}) to ({nx},{ny}) => {p}");
        //                     ((nx, ny), p)
        //                 })
        //         })
        //         .collect();
        //     result = new_result;
        // }
        //
        // // Nawak
        // result
        //     .into_iter()
        //     .filter(|((nx, ny), _)| (*nx as usize, *ny as usize) == end)
        //     .map(|(_, path)| path)
        //     .collect()
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
}

fn cost(c: char) -> usize {
    match c {
        '^' => 2,
        '>' => 2,
        'v' => 3,
        '<' => 4,
        _ => 0,
    }
}

fn next_step(
    kb: &Keyboard,
    seq: &FxHashMap<String, usize>,
    next_by_command_sequence: &mut FxHashMap<String, FxHashSet<String>>,
) -> Vec<FxHashMap<String, usize>> {
    let mut result: Vec<FxHashMap<String, usize>> = vec![Default::default()];
    for (cmd, count) in seq.iter() {
        let cmd = cmd.clone(); //replace("^",">");
        assert_eq!(cmd.chars().filter(|c| *c == 'A').count(), 1, "{:?}", seq);

        if !next_by_command_sequence.contains_key(&cmd) {
            // println!("cmd {cmd} not found ({:?})",next_by_command_sequence.len());
            next_by_command_sequence.insert(cmd.clone(), kb.robot_code_seqs(&cmd));
        }
        let mut new_result: Vec<FxHashMap<String, usize>> = Vec::new();
        // let nexts: &FxHashSet<String> = next_by_command_sequence
        //     .entry(cmd.clone())
        //     .or_insert({ println!("cmd {cmd} not found ({})",len);kb.robot_code_seqs(&cmd) });
        let condensed: Vec<FxHashMap<String, usize>> = next_by_command_sequence
            .get(&cmd)
            .unwrap()
            .iter()
            .map(|n| {
                n.split_inclusive('A').map(|cmd| cmd.to_string()).fold(
                    FxHashMap::default(),
                    |mut acc, sub| {
                        *acc.entry(sub).or_insert(0) += 1;

                        acc
                    },
                )
            })
            .collect();
        // if ! next_by_command_sequence.contains_key(cmd) {
        //     println!(" {cmd} still not found ");
        // }

        // println!("{nexts:?} => {condensed:?}");
        for next in condensed {
            for r in &result {
                let mut r = r.clone();
                for (n, local_count) in &next {
                    *r.entry(n.clone()).or_insert(0) += *local_count * count;
                }
                new_result.push(r);
            }
        }
        result = new_result;
    }

    result
}

// TODO : build the best conversion from 3 rounds
fn next_chars(init: char, end: char) -> String {
    if init == end {
        return "A".to_string();
    }
    if init == 'A' && end == 'v' {
        return "<vA".to_string(); //here
    }
    if init == 'v' && end == 'A' {
        return "^>A".to_string(); // >^ worse
    }
    if init == 'A' && end == '^' {
        return "<A".to_string();
    }
    if init == '^' && end == 'A' {
        return ">A".to_string();
    }
    if init == 'A' && end == '>' {
        return "vA".to_string();
    }
    if init == '>' && end == 'A' {
        return "^A".to_string();
    }

    if init == 'A' && end == '<' {
        return "v<<A".to_string();
    }
    if init == '<' && end == 'A' {
        return ">>^A".to_string();
    }

    if init == 'v' && end == '<' {
        return "<A".to_string();
    }
    if init == '<' && end == 'v' {
        return ">A".to_string();
    }

    if init == '>' && end == '<' {
        return "<<A".to_string();
    }
    if init == '<' && end == 'v' {
        return ">>A".to_string();
    }

    if init == '^' && end == '<' {
        return "v<A".to_string();
    }
    if init == '<' && end == '^' {
        return ">^A".to_string();
    }

    if init == '^' && end == 'v' {
        return "vA".to_string();
    }
    if init == 'v' && end == '^' {
        return "^A".to_string();
    }

    if init == '^' && end == '>' {
        return "v>A".to_string(); //here, >v worse
    }
    if init == '>' && end == '^' {
        return "<^A".to_string(); //here ^< worse
    }

    if init == 'v' && end == '>' {
        return ">A".to_string();
    }
    if init == '>' && end == 'v' {
        return "<A".to_string();
    }
    println!("unexpected {init}{end}");
    unreachable!()
}
fn next_cmds(keyb: &Keyboard, cmd: &str) -> FxHashMap<String, usize> {
    let mut init = 'A';
    let mut result: FxHashMap<String, usize> = Default::default();
    for c in cmd.chars() {
        *result.entry(next_chars(init, c)).or_insert(0) += 1;

        init = c;
    }
    result
}
fn input_code(
    code: &str,
    next_by_command_sequence: &mut FxHashMap<String, FxHashSet<String>>,
    depth: usize,
) -> usize {
    let door_kb = Keyboard::doors_keyboard();
    let robot_kb = Keyboard::robot_keyboard();

    // TODO : group equivalent Map
    let mut options: Vec<FxHashMap<String, usize>> = door_kb
        .robot_code_seqs(code)
        .into_iter()
        .map(|seq| {
            let mut cmds_freq: FxHashMap<String, usize> = Default::default();

            for cmd in seq.split_inclusive('A') {
                *cmds_freq.entry(cmd.to_string()).or_insert(0) += 1;
            }
            cmds_freq
        })
        .collect();

    assert!(options.iter().all(|cmd| cmd
        .keys()
        .all(|k| k.chars().filter(|c| *c == 'A').count() == 1)));

    // let mut cmds :Vec<FxHashMap<String,usize>>= next_by_command_sequence.keys().map(|k|FxHashMap::from_iter(vec![(k.clone(),1)])).collect();
    // for i in 0..depth {
    //     for cmd in &cmds {
    //
    //     }
    //     for (cmd,nexts) in next_by_command_sequence.iter() {
    //         best_moves_by_depth.entry((cmd.clone(),i)).or_insert(nexts.iter().map(|k|k.len()).min().unwrap());
    //     }
    //
    // }
    for i in 0..depth {
        let new_options: Vec<_> = options
            .into_iter()
            .map(|cmds| {
                let mut result: FxHashMap<String, usize> = Default::default();
                for (cmd, count) in cmds.iter() {
                    let mut init = 'A';
                    for c in cmd.chars() {
                        let next = next_chars(init, c);
                        *result.entry(next).or_insert(0) += count;
                        init = c;
                    }
                }
                result
            })
            .collect();
        options = new_options;
    }
    // seq => [cmd] => [[cmd]]
    // for i in 0..depth {
    //     let new_options: Vec<_> = options
    //         .into_iter()
    //         .flat_map(|seq| next_step(&robot_kb, &seq, next_by_command_sequence).into_iter())
    //         .collect();
    //     println!("{i}  before reduct {:?}", new_options.len());
    //     options = new_options
    //         // .iter()
    //         // .enumerate()
    //         // .filter(|(k, cmds)| {
    //         //     !new_options.iter().enumerate().any(|(l, other_cmds)| {
    //         //         *k != l
    //         //             && cmds.iter().all(|(c, count)| {
    //         //                 other_cmds
    //         //                     .get(c)
    //         //                     .filter(|ocount| **ocount >= *count)
    //         //                     .is_some()
    //         //             })
    //         //     })
    //         // })
    //         // .map(|(_,cmd)|cmd.clone())
    //         .into_iter()
    //         // .sorted_by(|f1,f2| {
    //         //     let v1 :usize=f1.iter().map(|(cmd,count)|cmd.chars().map(|c|cost(c)).sum::<usize>()   * *count).sum();
    //         //     let v2 :usize=f2.iter().map(|(cmd,count)|cmd.chars().map(|c|cost(c)).sum::<usize>()   * *count).sum();
    //         //     v1.cmp(&v2)
    //         // })
    //         .unique_by(|cmds| {
    //             cmds.iter()
    //                 .sorted_by(|(k1, _), (k2, _)| k1.replace("^", ">").cmp(&k2.replace("^", ">")))
    //                 .map(|(k, v)| (k.clone(), *v))
    //                 .collect_vec()
    //         })
    //         // .take(70)
    //         .collect(); // */
    //     println!("{i} after reduct {:?}", options.len());
    // }
    options
        .into_iter()
        .map(|o| o.into_iter().map(|(cmd, count)| cmd.len() * count).sum())
        .min()
        .unwrap()
}

fn complexity(
    code: &str,
    next_by_command_sequence: &mut FxHashMap<String, FxHashSet<String>>,
    depth: usize,
) -> usize {
    let code = code.trim();
    let first_non_digit = code.find(|c: char| !c.is_digit(10)).unwrap_or(code.len());
    let val_code = code[0..first_non_digit].parse::<usize>().unwrap();

    val_code * input_code(code, next_by_command_sequence, depth)
}
pub fn part_one(input: &str) -> Option<usize> {
    let mut next_by_command_sequence: FxHashMap<String, FxHashSet<String>> = Default::default();

    let result = input
        .lines()
        .map(|code| complexity(code, &mut next_by_command_sequence, 2))
        .sum();

    if next_by_command_sequence.len() > 25 {
        for (cmd, next) in next_by_command_sequence {
            println!("{cmd} => {:?}", next);
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut next_by_command_sequence: FxHashMap<String, FxHashSet<String>> = Default::default();

    let result = input
        .lines()
        .map(|code| complexity(code, &mut next_by_command_sequence, 25))
        .sum();

    // for (cmd, next) in next_by_command_sequence {
    //     println!("{cmd} => {}", next.len());
    // }

    Some(result)
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
        let mut next_by_command_sequence: FxHashMap<String, FxHashSet<String>> = Default::default();
        assert_eq!(
            complexity(&"029A", &mut next_by_command_sequence, 2),
            68 * 29
        );

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
        // assert_eq!(result, None);
    }
}
