use itertools::Itertools;

advent_of_code::solution!(25);

enum Schema {
    Lock(Vec<usize>),
    Key(Vec<usize>),
}

fn read_item<'a>(input: &mut impl Iterator<Item = &'a str>) -> Option<(Schema, usize)> {
    let first_line = input.next()?;
    if first_line.trim().chars().all(|c| c == '#') {
        let mut lock = vec![0usize; first_line.trim().len()];
        let mut i = 0;
        for line in input.by_ref() {
            if line.is_empty() {
                break;
            }
            i += 1;
            for (j, _) in line.trim().chars().enumerate().filter(|(_, c)| *c == '#') {
                assert_eq!(lock[j], i - 1);
                lock[j] = i;
            }
        }
        Some((Schema::Lock(lock), i))
    } else {
        let mut key: Vec<usize> = first_line
            .trim()
            .chars()
            .map(|c| if c == '.' { 1 } else { 0 })
            .collect();
        let mut i = 1;
        for line in input.by_ref() {
            if line.is_empty() {
                break;
            }
            i += 1;
            for (j, _) in line.trim().chars().enumerate().filter(|(_, c)| *c == '.') {
                assert_eq!(key[j], i - 1);
                key[j] = i;
            }
        }
        Some((
            Schema::Key(key.into_iter().map(|h| i - 1 - h).collect_vec()),
            i - 1,
        ))
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    let mut lines = input.lines();

    let mut locks: Vec<Vec<usize>> = vec![];
    let mut keys: Vec<Vec<usize>> = vec![];

    let mut size: Option<usize> = None;
    while let Some((s, sz)) = read_item(&mut lines) {
        if size.is_none() {
            size = Some(sz);
        }
        assert_eq!(size.unwrap(), sz);
        match s {
            Schema::Lock(v) => {
                locks.push(v);
            }
            Schema::Key(v) => {
                keys.push(v);
            }
        }
    }

    let size = size.unwrap();

    let len = locks[0].len();
    let mut no_overlap = 0;
    for l in &locks {
        for k in &keys {
            if (0..len).all(|i| l[i] + k[i] <= size - 1) {
                no_overlap += 1;
            }
        }
    }

    Some(no_overlap)
}

pub fn part_two(_input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
