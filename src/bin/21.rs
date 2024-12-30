use rustc_hash::{FxHashMap, FxHashSet};

advent_of_code::solution!(21);

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
            .flat_map(move |(j, l)| l.iter().enumerate().map(move |(i, c)| ((i, j), *c)))
            .filter(|(_, c)| '#' != *c),
        );

        Self { keys, init: (2, 3) }
    }

    fn robot_keyboard() -> Self {
        let keys = FxHashMap::from_iter(
            [['#', '^', 'A'], ['<', 'v', '>']]
                .iter()
                .enumerate()
                .flat_map(move |(j, l)| l.iter().enumerate().map(move |(i, c)| ((i, j), *c)))
                .filter(|(_, c)| '#' != *c),
        );

        Self { keys, init: (2, 0) }
    }
    fn get_key(&self, key: char) -> (usize, usize) {
        *self.keys.iter().find(|(_, c)| **c == key).unwrap().0
    }

    #[allow(dead_code)]
    fn convert_back(&self, seq: &str) -> String {
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
            return FxHashSet::from_iter(vec![vmove.to_string().repeat(dy.unsigned_abs())]);
        }
        if dy == 0 {
            return FxHashSet::from_iter(vec![hmove.to_string().repeat(dx.unsigned_abs())]);
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
                    vmove.to_string().repeat(dy.unsigned_abs())
                        + &hmove.to_string().repeat(dx.unsigned_abs()),
                )
            } else {
                None
            },
            if hz_first_possible {
                Some(
                    hmove.to_string().repeat(dx.unsigned_abs())
                        + &vmove.to_string().repeat(dy.unsigned_abs()),
                )
            } else {
                None
            },
        ]
        .into_iter()
        .flatten()
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
}

// TODO : build the best conversion from 3 rounds
// for robot_fmk
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
    eprintln!("unexpected {init}{end}");
    unreachable!()
}

fn input_code(code: &str, depth: usize) -> usize {
    let door_kb = Keyboard::doors_keyboard();
    let _robot_kb = Keyboard::robot_keyboard(); // FIXME : use it instead of next_cmds

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

    for _ in 0..depth {
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

    options
        .into_iter()
        .map(|o| o.into_iter().map(|(cmd, count)| cmd.len() * count).sum())
        .min()
        .unwrap()
}

fn complexity(code: &str, depth: usize) -> usize {
    let code = code.trim();
    let first_non_digit = code
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(code.len());
    let val_code = code[0..first_non_digit].parse::<usize>().unwrap();

    val_code * input_code(code, depth)
}
pub fn part_one(input: &str) -> Option<usize> {
    let result = input.lines().map(|code| complexity(code, 2)).sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let result = input.lines().map(|code| complexity(code, 25)).sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(complexity(&"029A", 2), 68 * 29);

        assert_eq!(complexity(&"379A", 2), 64 * 379);

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }
}
