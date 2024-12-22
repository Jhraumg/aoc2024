use rustc_hash::{FxHashMap, FxHashSet};

advent_of_code::solution!(22);

fn mix(secret: usize, value: usize) -> usize {
    secret ^ value
}
fn prune(secret: usize) -> usize {
    secret % 16777216
}
fn update_secret(secret: usize) -> usize {
    let mut secret = prune(mix(secret, secret << 6));
    secret = prune(mix(secret, secret >> 5));
    prune(mix(secret, secret << 11))
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut secrets: Vec<usize> = input.lines().map(|l| l.parse().unwrap()).collect();

    for _ in 0..2000 {
        secrets = secrets.into_iter().map(update_secret).collect();
    }

    Some(secrets.into_iter().sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut secrets: Vec<usize> = input.lines().map(|l| l.parse().unwrap()).collect();

    let mut changes: FxHashSet<[isize; 4]> = FxHashSet::default();
    let secrets1: Vec<_> = secrets.iter().copied().map(update_secret).collect();
    let secrets2: Vec<_> = secrets1.iter().copied().map(update_secret).collect();
    let secrets3: Vec<_> = secrets2.iter().copied().map(update_secret).collect();
    let secrets4: Vec<_> = secrets3.iter().copied().map(update_secret).collect();
    let mut secrets_and_changes: Vec<(usize, [isize; 4])> = (0..secrets.len())
        .map(|i| {
            (
                secrets4[i],
                [
                    (secrets1[i] % 10) as isize - (secrets[i] % 10) as isize,
                    (secrets2[i] % 10) as isize - (secrets1[i] % 10) as isize,
                    (secrets3[i] % 10) as isize - (secrets2[i] % 10) as isize,
                    (secrets4[i] % 10) as isize - (secrets3[i] % 10) as isize,
                ],
            )
        })
        .collect();
    secrets = secrets4;

    let mut first_encouter: Vec<FxHashMap<[isize; 4], usize>> =
        vec![FxHashMap::default(); secrets.len()];

    // FIXME : look for period on each secret
    for _ in 0..2000 - 4 {
        let new_secrets_and_changes: Vec<_> = secrets_and_changes
            .iter()
            .enumerate()
            .map(|(i, (sec, change))| {
                first_encouter[i].entry(*change).or_insert(*sec % 10);
                changes.insert(*change);
                let new_sec = update_secret(*sec);
                let new_change = [
                    change[1],
                    change[2],
                    change[3],
                    (new_sec % 10) as isize - (*sec % 10) as isize,
                ];
                changes.insert(new_change);
                first_encouter[i].entry(new_change).or_insert(new_sec % 10);
                (new_sec, new_change)
            })
            .collect();

        secrets_and_changes = new_secrets_and_changes;
    }

    changes
        .into_iter()
        .map(|change| {
            first_encouter
                .iter()
                .filter_map(|local_changes| local_changes.get(&change))
                .sum::<usize>()
        })
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(update_secret(123), 15887950);
        assert_eq!(update_secret(15887950), 16495136);
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }
}
