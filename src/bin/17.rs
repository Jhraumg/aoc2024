use itertools::Itertools;

advent_of_code::solution!(17);

type Val = isize;

#[derive(Debug, Default, Clone)]
struct Computer {
    a: Val,
    b: Val,
    c: Val,
    sp: usize,
    instrs: Vec<u8>,
    out: Vec<u8>,
}

impl Computer {
    fn read(input: &str) -> Computer {
        let mut lines = input.lines();

        let a = lines.next().unwrap()["Register A:".len()..]
            .trim()
            .parse()
            .unwrap();
        let b = lines.next().unwrap()["Register A:".len()..]
            .trim()
            .parse()
            .unwrap();
        let c = lines.next().unwrap()["Register A:".len()..]
            .trim()
            .parse()
            .unwrap();
        lines.next();
        let instrs: Vec<u8> = lines.next().unwrap()["Program: ".len()..]
            .trim()
            .split(',')
            .map(|i| i.parse().unwrap())
            .collect();

        assert!(instrs.iter().all(|i| *i < 8));

        Computer {
            a,
            b,
            c,
            instrs,
            sp: 0,
            out: vec![],
        }
    }
    fn get_operand(&mut self) -> Option<u8> {
        if self.sp >= self.instrs.len() {
            println!("end of instrs reach !");
            return None;
        }
        self.sp += 1;

        Some(self.instrs[self.sp - 1])
    }
    fn combo(&self, v: u8) -> Val {
        match v {
            v if v <= 3 => v as Val,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => panic!("7 combo found !!"),
            _ => unreachable!("no operand > 7"),
        }
    }
    fn adv(&mut self) {
        if let Some(operand) = self.get_operand() {
            self.a >>= self.combo(operand); //TODO : check truncated
        }
    }
    fn bxl(&mut self) {
        if let Some(operand) = self.get_operand() {
            self.b ^= operand as Val;
        }
    }
    fn bst(&mut self) {
        if let Some(operand) = self.get_operand() {
            self.b = self.combo(operand) % 8;
        }
    }
    fn jnz(&mut self) {
        if let Some(operand) = self.get_operand() {
            if self.a != 0 {
                self.sp = operand as usize;
            }
        }
    }
    fn bxc(&mut self) {
        if self.get_operand().is_some() {
            self.b ^= self.c;
        }
    }
    fn out(&mut self) {
        if let Some(operand) = self.get_operand() {
            self.out.push((self.combo(operand) % 8) as u8);
        }
    }
    fn bdv(&mut self) {
        if let Some(operand) = self.get_operand() {
            self.b = self.a >> self.combo(operand);
        }
    }
    fn cdv(&mut self) {
        if let Some(operand) = self.get_operand() {
            self.c = self.a >> self.combo(operand);
        }
    }
    fn tick(&mut self) -> bool {
        if self.sp >= self.instrs.len() {
            return false;
        }
        let opcode = self.instrs[self.sp];
        self.sp += 1;
        match opcode {
            0 => {
                self.adv();
            }
            1 => {
                self.bxl();
            }
            2 => {
                self.bst();
            }
            3 => {
                self.jnz();
            }
            4 => {
                self.bxc();
            }
            5 => {
                self.out();
            }
            6 => {
                self.bdv();
            }
            7 => {
                self.cdv();
            }
            _ => {
                unreachable!("no >7 opcode")
            }
        }
        self.sp < self.instrs.len()
    }
    fn process(&mut self) -> String {
        while self.tick() {}
        self.out.iter().join(",").trim().to_string()
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut computer = Computer::read(input);

    Some(computer.process())
}

pub fn part_two(input: &str) -> Option<Val> {
    let computer = Computer::read(input);

    // FIXME : this works only with A sliding by 3 bits each turn
    // this could be adapted to slide bit by bit though
    let mut result: Vec<Val> = vec![0; computer.instrs.len()];
    result[0] = 1;

    'main: loop {
        let a = result.iter().copied().reduce(|acc, r| acc * 8 + r).unwrap();
        let mut cp = Computer {
            a,
            ..computer.clone()
        };
        let out = cp.process();
        println!("{a:48b} => {:?} / {:?}", out, &computer.instrs);

        for (i, (r, o)) in computer
            .instrs
            .iter()
            .copied()
            .zip(cp.out.iter().copied())
            .rev()
            .enumerate()
        {
            if r != o {
                result[i] += 1;
                if result[i] == 8 {
                    println!(
                        "ERROR {:?} => {:?} instead of {:?}",
                        result, &cp.out, &cp.instrs
                    );
                    panic!("TODO");
                }
                continue 'main;
            }
        }
        break;
    }

    result.iter().copied().reduce(|acc, r| acc * 8 + r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_instr() {
        let mut computer = Computer {
            c: 9,
            instrs: vec![2, 6],
            ..Default::default()
        };
        computer.process();
        assert_eq!(computer.b, 1);

        // let mut computer = Computer{a:10, instrs: vec![5,0,5,1,5,4],..Default::default()};
        // assert_eq!("0,1,2",computer.process());

        let mut computer = Computer {
            a: 2024,
            instrs: vec![0, 1, 5, 4, 3, 0],
            ..Default::default()
        };
        assert_eq!("4,2,5,6,7,7,7,7,3,1,0", computer.process());
        assert_eq!(computer.a, 0);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let mut computer = Computer {
            a: 117440,
            instrs: vec![0, 3, 5, 4, 3, 0],
            ..Default::default()
        };
        assert_eq!("0,3,5,4,3,0".to_string(), computer.process());
    }
}
