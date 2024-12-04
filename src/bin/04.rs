use itertools::Itertools;
advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let input: Vec<_> = input.lines().collect();
    let raw_input: Vec<_> = input.iter().map(|l| l.as_bytes()).collect();
    let raw_input = &raw_input;
    let row_count = input.len();
    let col_count = input[0].len();
    let rawxmas = "XMAS".as_bytes();

    let maybe_starts = (0..row_count).flat_map(move |i| {
        (0..col_count).filter_map(move |j| {
            if raw_input[i][j] == b'X' {
                Some((i, j))
            } else {
                None
            }
        })
    });

    Some(
        maybe_starts
            .map(|(i, j)| {
                let mut count = 0;
                if input[i][j..].starts_with("XMAS") {
                    count += 1;
                }
                if input[i][..j + 1].ends_with("SAMX") {
                    count += 1;
                }
                if i < row_count - 3 && (1usize..4).all(|k| raw_input[i + k][j] == rawxmas[k]) {
                    count += 1;
                }

                if i >= 3 && (1usize..4).all(|k| raw_input[i - k][j] == rawxmas[k]) {
                    count += 1;
                }
                if i < row_count - 3
                    && j < col_count - 3
                    && (1usize..4).all(|k| raw_input[i + k][j + k] == rawxmas[k])
                {
                    count += 1;
                }
                if i < row_count - 3
                    && j >= 3
                    && (1usize..4).all(|k| raw_input[i + k][j - k] == rawxmas[k])
                {
                    count += 1;
                }
                if i >= 3
                    && j < col_count - 3
                    && (1usize..4).all(|k| raw_input[i - k][j + k] == rawxmas[k])
                {
                    count += 1;
                }
                if i >= 3 && j >= 3 && (1usize..4).all(|k| raw_input[i - k][j - k] == rawxmas[k]) {
                    count += 1;
                }

                count
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let raw_input: Vec<_> = input.lines().map(|l| l.as_bytes()).collect();
    let raw_input = &raw_input;
    let row_count = raw_input.len();
    let col_count = raw_input[0].len();

    let maybe_center = (1..row_count - 1).flat_map(move |i| {
        (1..col_count - 1).filter_map(move |j| {
            if raw_input[i][j] == b'A' {
                Some((i, j))
            } else {
                None
            }
        })
    });

    Some(
        maybe_center
            .filter(|(i, j)| {
                ((raw_input[i - 1][j - 1] == b'M' && raw_input[i + 1][j + 1] == b'S')
                    || (raw_input[i - 1][j - 1] == b'S' && raw_input[i + 1][j + 1] == b'M'))
                    && ((raw_input[i - 1][j + 1] == b'M' && raw_input[i + 1][j - 1] == b'S')
                        || (raw_input[i - 1][j + 1] == b'S' && raw_input[i + 1][j - 1] == b'M'))
            })
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
