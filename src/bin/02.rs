use itertools::Itertools;

advent_of_code::solution!(2);

fn is_diff_safe(d: isize, is_positive: bool) -> bool {
    d.is_positive() == is_positive && d.abs() >= 1 && d.abs() <= 3
}

fn is_safe(report: impl IntoIterator<Item = usize>) -> bool {
    let diffs: Vec<isize> = report
        .into_iter()
        .tuple_windows::<(_, _)>()
        .map(|(a, b)| b as isize - a as isize)
        .collect();

    !diffs.is_empty() && {
        let is_positive = diffs[0].is_positive();
        diffs.iter().all(|d| is_diff_safe(*d, is_positive))
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    let reports: Vec<Vec<usize>> = input
        .lines()
        .map(|l| l.split_whitespace().map(|w| w.parse().unwrap()).collect())
        .collect();
    Some(
        reports
            .into_iter()
            .filter(|r| is_safe(r.iter().copied()))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let reports: Vec<Vec<usize>> = input
        .lines()
        .map(|l| l.split_whitespace().map(|w| w.parse().unwrap()).collect())
        .collect();
    Some(
        reports
            .iter()
            .filter(|r| {
                is_safe(r.iter().copied())
                    || (0..reports.len())
                        .map(|i| {
                            r.iter().enumerate().filter_map(
                                move |(j, v)| {
                                    if j != i {
                                        Some(*v)
                                    } else {
                                        None
                                    }
                                },
                            )
                        })
                        .any(is_safe)
            })
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
