advent_of_code::solution!(3);

use std::num::ParseIntError;
use std::str::FromStr;
use regex::Regex;

const RAW_REGEXP: &str = "mul\\(([0-9]{1,3}),([0-9]{1,3})\\)";
pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new("mul\\(([0-9]{1,3}),([0-9]{1,3})\\)").unwrap();
    Some(
        re.captures_iter(input)
            .map(|c| {
                let v1: u32 = c.get(1).unwrap().as_str().parse().unwrap();
                let v2: u32 = c.get(2).unwrap().as_str().parse().unwrap();
                v1 * v2
            })
            .sum(),
    )
}

enum Instruction{
    Do,
    Dont,
    Mul(u32,u32)
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mul_re=Regex::new("mul\\(([0-9]{1,3}),([0-9]{1,3})\\)").unwrap();
        match s{
            "do()" => Ok(Instruction::Do),
            "don't()" => Ok(Instruction::Dont),
            mul => {
                let c = mul_re.captures(mul).ok_or(format!("Invalid instruction: {}", mul))?;
                let v1: u32 = c.get(1).unwrap().as_str().parse().map_err(|e:ParseIntError|e.to_string())?;
                let v2: u32 = c.get(2).unwrap().as_str().parse().map_err(|e:ParseIntError|e.to_string())?;
                Ok(Instruction::Mul(v1, v2))
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let main_re = Regex::new("mul\\([0-9]{1,3},[0-9]{1,3}\\)|do(?:n\'t)?\\(\\)").unwrap();

    let instrs: Vec<_> = main_re.find_iter(input).map(|m| m.as_str()).collect();

    Some(instrs.into_iter().map(|instr|instr.parse::<Instruction>().unwrap())
        .fold((true,0u32),|(active,sum),instr|match instr{
            Instruction::Do => {(true,sum)}
            Instruction::Dont => {(false,sum)}
            Instruction::Mul(v1, v2) => (active,if active {sum+v1*v2}else { sum })}).1
        )
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY,1));
        assert_eq!(result, Some(48));
    }
}
