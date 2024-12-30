use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

advent_of_code::solution!(23);

pub fn read_interco<'i>(input: &'i str) -> FxHashSet<(&'i str, &'i str)> {
    let mut result: FxHashSet<(&'i str, &'i str)> = Default::default();
    for (c1, c2) in input.lines().map(|l| l.split_once('-').unwrap()) {
        let c1 = c1.trim();
        let c2 = c2.trim();
        assert_ne!(c1, c2);
        if c1 < c2 {
            result.insert((c1, c2));
        } else {
            result.insert((c2, c1));
        }
    }
    result
}

fn count_3cycles_with_t(interco: &FxHashSet<(&str, &str)>) -> usize {
    let target_computer: FxHashSet<&str> = interco
        .iter()
        .flat_map(|(c1, c2)| [*c1, *c2].into_iter())
        .filter(|c| c.starts_with('t'))
        .collect();
    target_computer
        .iter()
        .map(|tc| {
            let candidates: FxHashSet<_> = interco
                .iter()
                .filter_map(|(c1, c2)| {
                    if c1 == tc {
                        Some(*c2)
                    } else if c2 == tc {
                        Some(*c1)
                    } else {
                        None
                    }
                })
                .collect();
            candidates
                .iter()
                .map(|c| {
                    interco
                        .iter()
                        .filter(|(it, oc)| {
                            if c.starts_with('t') && c < tc {
                                return false;
                            }
                            if oc.starts_with('t') && oc < tc {
                                return false;
                            }
                            if it == c && oc != tc && candidates.contains(oc) {
                                return true;
                            }
                            false
                        })
                        .count()
                })
                .sum::<usize>()
        })
        .sum()
}

fn all_connected(computers: &FxHashSet<&str>, interco: &FxHashSet<(&str, &str)>) -> bool {
    computers.iter().all(|c| {
        computers.iter().all(|oc| {
            oc == c
                || if c < oc {
                    interco.contains(&(*c, *oc))
                } else {
                    interco.contains(&(*oc, *c))
                }
        })
    })
}

fn password(interco: &FxHashSet<(&str, &str)>) -> String {
    let edges_by_computer: FxHashMap<&str, FxHashSet<&str>> =
        interco
            .iter()
            .fold(Default::default(), |mut acc, (c1, c2)| {
                acc.entry(*c1)
                    .and_modify(|edg| {
                        edg.insert(*c2);
                    })
                    .or_insert(FxHashSet::from_iter([*c2]));
                acc.entry(*c2)
                    .and_modify(|edg| {
                        edg.insert(*c1);
                    })
                    .or_insert(FxHashSet::from_iter([*c1]));
                acc
            });

    let max_set = 1 + edges_by_computer
        .values()
        .map(|edges| edges.len())
        .max()
        .unwrap();

    for i in 0..max_set {
        for (cp, edges) in edges_by_computer
            .iter()
            .filter(|(_, edges)| edges.len() + 1 >= max_set - i)
        {
            if max_set == edges.len() + 1 + i {
                let mut set: FxHashSet<&str> = edges.clone();
                set.insert(*cp);
                if all_connected(&set, interco) {
                    return set.into_iter().sorted().join(",");
                }
            } else {
                for others in edges.iter().combinations(i + 1 + edges.len() - max_set) {
                    let mut set: FxHashSet<_> = edges
                        .iter()
                        .filter(|e| !others.contains(e))
                        .copied()
                        .collect();
                    set.insert(*cp);
                    if all_connected(&set, interco) {
                        return set.into_iter().sorted().join(",");
                    }
                }
            }
        }
    }

    unreachable!()
}
pub fn part_one(input: &str) -> Option<usize> {
    Some(count_3cycles_with_t(&read_interco(input)))
}

pub fn part_two(input: &str) -> Option<String> {
    Some(password(&read_interco(input)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
