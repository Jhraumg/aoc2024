use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

advent_of_code::solution!(24);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Gate<'a> {
    entries: [&'a str; 2],
    output: &'a str,
    op: &'a str,
}

fn read_gate(s: &str) -> Gate<'_> {
    let Some((entry1, op, entry2, _arrow, output)) = s.split_whitespace().collect_tuple() else {
        unreachable!("invalidate gate line")
    };
    Gate {
        entries: [entry1.trim(), entry2.trim()],
        output: output.trim(),
        op: op.trim(),
    }
}

fn read_status_and_gates(input: &str) -> (FxHashMap<&str, bool>, Vec<Gate>) {
    let mut statuses: FxHashMap<&str, bool> = FxHashMap::default();
    let mut gates: Vec<Gate> = vec![];

    for l in input.lines() {
        if l.contains(":") {
            let (input, status) = l.split_once(": ").unwrap();
            statuses.insert(input, status == "1");
        }
        if l.contains("->") {
            gates.push(read_gate(l));
        }
    }
    (statuses, gates)
}

fn compute(init: &FxHashMap<&str, bool>, gates: &[Gate]) -> Option<usize> {
    let mut statuses = init.clone();

    let mut outputs: FxHashSet<&str> =
        FxHashSet::from_iter(statuses.keys().filter(|k| k.starts_with('z')).copied());
    for o in gates
        .iter()
        .filter(|g| g.output.starts_with("z"))
        .map(|g| g.output)
    {
        outputs.insert(o);
    }

    let outputs: Vec<_> = outputs.into_iter().sorted().collect();

    while outputs.iter().any(|o| !statuses.contains_key(o)) {
        let gates_to_compute: Vec<_> = gates
            .iter()
            .filter(|g| !statuses.contains_key(g.output))
            .collect();
        for g in gates_to_compute {
            if let Some(entry1) = statuses.get(g.entries[0]).copied() {
                if let Some(entry2) = statuses.get(g.entries[1]).copied() {
                    let result = match g.op {
                        "AND" => entry1 && entry2,
                        "OR" => entry1 || entry2,
                        "XOR" => entry1 ^ entry2,
                        _ => unreachable!("invalid gate op"),
                    };
                    statuses.insert(g.output, result);
                }
            }
        }
    }

    outputs
        .iter()
        .rev()
        .map(|o| if statuses[o] { 1 } else { 0 })
        .reduce(|acc, v| acc * 2 + v)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (statuses, gates) = read_status_and_gates(input);

    compute(&statuses, &gates)
}

#[derive(Default)]
struct GateFilter<'a> {
    entry1: Option<&'a str>,
    entry2: Option<&'a str>,
    op: Option<&'a str>,
    output: Option<&'a str>,
}
impl GateFilter<'_> {
    fn accept(&self, gate: &Gate) -> bool {
        if let Some(entry1) = self.entry1 {
            if !gate.entries.contains(&entry1) {
                return false;
            }
        }
        if let Some(entry2) = self.entry2 {
            if !gate.entries.contains(&entry2) {
                return false;
            }
        }
        if let Some(output) = self.output {
            if gate.output != output {
                return false;
            }
        }
        if let Some(op) = self.op {
            if gate.op != op {
                return false;
            }
        }
        true
    }
}

fn find_output<'a>(filter: &GateFilter<'a>, gates: &'a [Gate<'a>]) -> Option<&'a Gate<'a>> {
    gates.iter().find(|g| filter.accept(g))
}

fn find_z_predecessor<'a>(n: usize, gates: &'a [Gate<'a>]) -> FxHashSet<String> {
    assert!(n >= 2, "gates are not regular before 2");

    let mut result = FxHashSet::default();

    let (xn, yn, zn) = (format!("x{n:02}"), format!("y{n:02}"), format!("z{n:02}"));

    let xor_n = find_output(
        &GateFilter {
            entry1: Some(&xn),
            entry2: Some(&yn),
            output: None,
            op: Some("XOR"),
        },
        gates,
    )
    .unwrap();

    let (xn_1, yn_1) = (format!("x{:02}", n - 1), format!("y{:02}", n - 1));
    let and_n_1 = find_output(
        &GateFilter {
            entry1: Some(&xn_1),
            entry2: Some(&yn_1),
            output: None,
            op: Some("AND"),
        },
        gates,
    )
    .unwrap();

    let carry = find_output(
        &GateFilter {
            entry2: Some(and_n_1.output),
            op: Some("OR"),
            ..Default::default()
        },
        gates,
    );
    if let Some(carry) = carry {
        let zout_n = find_output(
            &GateFilter {
                entry1: Some(xor_n.output),
                entry2: Some(carry.output),
                output: None,
                op: Some("XOR"),
            },
            gates,
        );

        if let Some(zout_n) = zout_n {
            if zout_n.output != zn {
                result.insert(zout_n.output.to_string());
                result.insert(zn.clone());
            }
        } else {
            let carry_from_xorn = find_output(
                &GateFilter {
                    entry1: Some(xor_n.output),
                    op: Some("XOR"),
                    ..Default::default()
                },
                gates,
            );
            if carry_from_xorn.is_none() {
                result.insert(xor_n.output.to_string());
            } else {
                result.insert(carry.output.to_string());
            }
            // should look for other half
        }
    } else {
        // should look for other half
        result.insert(and_n_1.output.to_string());
    }

    result
}

pub fn part_two(input: &str) -> Option<String> {
    let (_statuses, gates) = read_status_and_gates(input);
    let zs: Vec<_> = gates
        .iter()
        .filter(|g| g.output.starts_with("z"))
        .map(|g| g.output)
        .sorted()
        .collect();

    let mut result: FxHashSet<String> = FxHashSet::default();
    for i in 3..zs.len() - 1 {
        result = result
            .union(&find_z_predecessor(i, &gates))
            .cloned()
            .collect();
    }

    assert_eq!(
        result.len(),
        8,
        "only {} invalid gate output found",
        result.len()
    );

    Some(result.iter().sorted().join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }
}
