use crate::Operation::*;
use std::str::FromStr;

advent_of_code::solution!(7);

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    Mult,
    Add,
    Concat,
}

struct Equation {
    result: usize,
    operands: Vec<usize>,
}

impl FromStr for Equation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (result, operands) = s.split_once(':').unwrap();
        let result: usize = result.trim().parse().unwrap();

        let operands: Vec<usize> = operands
            .split_whitespace()
            .map(|w| w.parse().unwrap())
            .collect();
        Ok(Self { result, operands })
    }
}
impl Equation {
    fn can_solve(&self, operators: &[Operation]) -> bool {
        if self.operands.len() == 2 {
            return self.result == self.operands[0] + self.operands[1]
                || self.result == self.operands[0] * self.operands[1]
                || {
                    operators.contains(&Concat) && {
                        let &[o1, o2] = &self.operands[..2] else {
                            unreachable!("there's 2 operands")
                        };
                        let p10 = 10usize.pow(o2.ilog10() + 1);
                        self.result == o1 * p10 + o2
                    }
                };
        }

        let remaining = self.operands.len();
        let end = self.operands[remaining - 1];
        operators.iter().any(|op| match op {
            Mult => {
                self.result % end == 0 && {
                    let new_eq = Equation {
                        result: self.result / end,
                        operands: self.operands[..remaining - 1].to_vec(),
                    };
                    new_eq.can_solve(operators)
                }
            }
            Add => {
                self.result >= end && {
                    let new_eq = Equation {
                        result: self.result - end,
                        operands: self.operands[..remaining - 1].to_vec(),
                    };
                    new_eq.can_solve(operators)
                }
            }
            Concat => {
                let p10 = 10usize.pow(end.ilog10() + 1);

                self.result % p10 == end && {
                    let new_eq = Equation {
                        result: self.result / p10,
                        operands: self.operands[..remaining - 1].to_vec(),
                    };
                    new_eq.can_solve(operators)
                }
            }
        })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let equations: Vec<Equation> = input.lines().map(|l| l.parse().unwrap()).collect();

    Some(
        equations
            .iter()
            .filter(|e| e.can_solve(&[Mult, Add]))
            .map(|e| e.result)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let equations: Vec<Equation> = input.lines().map(|l| l.parse().unwrap()).collect();
    Some(
        equations
            .iter()
            .filter(|e| e.can_solve(&[Mult, Add, Concat]))
            .map(|e| e.result)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
