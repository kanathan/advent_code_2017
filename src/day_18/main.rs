use std::{collections::{HashMap, VecDeque}, usize};

fn main() {
    let instrs = parse(include_str!("input"));
    println!("{}", part_1(&instrs));
    println!("{}", part_2(&instrs));
}


fn part_1(instrs: &[Instructions]) -> isize {
    let mut reg: HashMap<char, isize> = HashMap::new();
    let mut snd = 0;

    let mut pc = 0;
    while let Some(instr) = instrs.get(pc as usize) {
        pc += 1;
        match *instr {
            Instructions::Snd(r1) => snd = *reg.entry(r1).or_default(),
            Instructions::Snd_d(val) => snd = val,
            Instructions::Set(r1, r2) => {
                let val = *reg.entry(r2).or_default();
                reg.insert(r1, val); 
            },
            Instructions::Set_d(r1, val) => { reg.insert(r1, val); },
            Instructions::Add(r1, r2) => {
                let val = *reg.entry(r2).or_default();
                *reg.entry(r1).or_default() += val;
            },
            Instructions::Add_d(r1, val) => *reg.entry(r1).or_default() += val,
            Instructions::Mul(r1, r2) => {
                let val = *reg.entry(r2).or_default();
                *reg.entry(r1).or_default() *= val;
            },
            Instructions::Mul_d(r1, val) => *reg.entry(r1).or_default() *= val,
            Instructions::Mod(r1, r2) => {
                let val = *reg.entry(r2).or_default();
                *reg.entry(r1).or_default() %= val;
            },
            Instructions::Mod_d(r1, val) => *reg.entry(r1).or_default() %= val,
            Instructions::Rcv(r1) => if *reg.entry(r1).or_default() != 0 { return snd },
            Instructions::Rcv_d(val) => if val != 0 { return snd },
            Instructions::Jgz(r1, r2) => if *reg.entry(r1).or_default() > 0 {
                let val = *reg.entry(r2).or_default();
                pc += val - 1;
                if pc < 0 { panic!() }
            },
            Instructions::Jgz_rd(r1, val) => if *reg.entry(r1).or_default() > 0 {
                pc += val - 1;
                if pc < 0 { panic!() }
            },
            Instructions::Jgz_dr(v1, r2) => if v1 > 0 {
                let offset = *reg.entry(r2).or_default();
                pc += offset - 1;
                if pc < 0 { panic!() }
            },
            Instructions::Jgz_dd(v1, v2) => if v1 > 0 {
                pc += v2 - 1;
                if pc < 0 { panic!() }
            }
        };
    }

    unreachable!()
}


fn part_2(instrs: &[Instructions]) -> isize {
    let mut prog_0 = Program::new(0);
    let mut prog_1 = Program::new(1);

    let mut q0_1 = VecDeque::new();
    let mut q1_0 = VecDeque::new();

    let mut loop_count = 0;

    while !prog_0.locked || !q1_0.is_empty() || !prog_1.locked || !q0_1.is_empty()  {
        loop_count += 1;
        if loop_count % 10000 == 0 {
            println!("Loop count at {loop_count}");
        }
        if !prog_0.locked || !q1_0.is_empty() {
            run(instrs, &mut prog_0, &mut q0_1, &mut q1_0);
        }
        if !prog_1.locked || !q0_1.is_empty() {
            run(instrs, &mut prog_1, &mut q1_0, &mut q0_1);
        }
        if prog_0.terminated && prog_1.terminated { break }
    }

    prog_1.send_count
}


fn run(instrs: &[Instructions], prog: &mut Program, tx: &mut VecDeque<isize>, rx: &mut VecDeque<isize>) {
    let mut loop_count = 0;
    prog.locked = false;
    while let Some(instr) = instrs.get(prog.pc as usize) {
        loop_count += 1;
        prog.pc += 1;
        match *instr {
            Instructions::Snd(r1) => {
                tx.push_back(*prog.reg.entry(r1).or_default());
                prog.send_count += 1;
            },
            Instructions::Snd_d(val) => {
                tx.push_back(val);
                prog.send_count += 1;
            },
            Instructions::Set(r1, r2) => {
                let val = *prog.reg.entry(r2).or_default();
                prog.reg.insert(r1, val); 
            },
            Instructions::Set_d(r1, val) => { prog.reg.insert(r1, val); },
            Instructions::Add(r1, r2) => {
                let val = *prog.reg.entry(r2).or_default();
                *prog.reg.entry(r1).or_default() += val;
            },
            Instructions::Add_d(r1, val) => *prog.reg.entry(r1).or_default() += val,
            Instructions::Mul(r1, r2) => {
                let val = *prog.reg.entry(r2).or_default();
                *prog.reg.entry(r1).or_default() *= val;
            },
            Instructions::Mul_d(r1, val) => *prog.reg.entry(r1).or_default() *= val,
            Instructions::Mod(r1, r2) => {
                let val = *prog.reg.entry(r2).or_default();
                *prog.reg.entry(r1).or_default() %= val;
            },
            Instructions::Mod_d(r1, val) => *prog.reg.entry(r1).or_default() %= val,
            Instructions::Rcv(r1) => {
                if let Some(val) = rx.pop_front() {
                    prog.reg.insert(r1, val);
                } else {
                    prog.locked = true;
                    prog.pc -= 1;
                    break;
                }
            },
            Instructions::Rcv_d(val) => unimplemented!(),
            Instructions::Jgz(r1, r2) => if *prog.reg.entry(r1).or_default() > 0 {
                let val = *prog.reg.entry(r2).or_default();
                prog.pc += val - 1;
                if prog.pc < 0 { panic!() }
            },
            Instructions::Jgz_rd(r1, val) => if *prog.reg.entry(r1).or_default() > 0 {
                prog.pc += val - 1;
                if prog.pc < 0 { panic!() }
            },
            Instructions::Jgz_dr(v1, r2) => if v1 > 0 {
                let offset = *prog.reg.entry(r2).or_default();
                prog.pc += offset - 1;
                if prog.pc < 0 { panic!() }
            },
            Instructions::Jgz_dd(v1, v2) => if v1 > 0 {
                prog.pc += v2 - 1;
                if prog.pc < 0 { panic!() }
            }
        };
        if loop_count >= 100 {
            break
        }
    }
    if prog.pc < 0 || prog.pc as usize >= instrs.len() {
        prog.terminated = true;
    }
}


#[derive(Clone)]
struct Program {
    reg: HashMap<char, isize>,
    locked: bool,
    terminated: bool,
    pc: isize,
    send_count: isize,
}

impl Program {
    fn new(id: isize) -> Self {
        Self {
            reg: HashMap::from([('p', id)]),
            locked: false,
            terminated: false,
            pc: 0,
            send_count: 0,
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
enum Instructions {
    Snd(char),
    Snd_d(isize),
    Set(char, char),
    Set_d(char, isize),
    Add(char, char),
    Add_d(char, isize),
    Mul(char, char),
    Mul_d(char, isize),
    Mod(char, char),
    Mod_d(char, isize),
    Rcv(char),
    Rcv_d(isize),
    Jgz(char, char),
    Jgz_rd(char, isize),
    Jgz_dr(isize, char),
    Jgz_dd(isize, isize),
}


fn parse(input: &str) -> Vec<Instructions> {
    let mut instrs = vec![];
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let inst_str = parts.next().unwrap();
        let v1 = parts.next().unwrap();
        let v2 = parts.next();
        instrs.push(
            match inst_str {
                "snd" => {
                    if let Ok(val) = v1.parse() {
                        Instructions::Snd_d(val)
                    } else {
                        Instructions::Snd(v1.chars().next().unwrap())
                    }
                },
                "set" => {
                    if let Ok(val) = v2.unwrap().parse() {
                        Instructions::Set_d(v1.chars().next().unwrap(), val)
                    } else {
                        Instructions::Set(v1.chars().next().unwrap(), v2.unwrap().chars().next().unwrap())
                    }
                },
                "add" => {
                    if let Ok(val) = v2.unwrap().parse() {
                        Instructions::Add_d(v1.chars().next().unwrap(), val)
                    } else {
                        Instructions::Add(v1.chars().next().unwrap(), v2.unwrap().chars().next().unwrap())
                    }
                },
                "mul" => {
                    if let Ok(val) = v2.unwrap().parse() {
                        Instructions::Mul_d(v1.chars().next().unwrap(), val)
                    } else {
                        Instructions::Mul(v1.chars().next().unwrap(), v2.unwrap().chars().next().unwrap())
                    }
                },
                "mod" => {
                    if let Ok(val) = v2.unwrap().parse() {
                        Instructions::Mod_d(v1.chars().next().unwrap(), val)
                    } else {
                        Instructions::Mod(v1.chars().next().unwrap(), v2.unwrap().chars().next().unwrap())
                    }
                },
                "rcv" => {
                    if let Ok(val) = v1.parse() {
                        Instructions::Rcv_d(val)
                    } else {
                        Instructions::Rcv(v1.chars().next().unwrap())
                    }
                },
                "jgz" => {
                    if let Ok(val1) = v1.parse() {
                        if let Ok(val2) = v2.unwrap().parse() {
                            Instructions::Jgz_dd(val1, val2)
                        } else {
                            Instructions::Jgz_dr(val1, v2.unwrap().chars().next().unwrap())
                        }
                    } else if let Ok(val2) = v2.unwrap().parse() {
                        Instructions::Jgz_rd(v1.chars().next().unwrap(), val2)
                    } else {
                        Instructions::Jgz(v1.chars().next().unwrap(), v2.unwrap().chars().next().unwrap())
                    }
                },
                _ => unreachable!(),
            }
        );
    }
    instrs
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = 
    "set a 1\n\
    add a 2\n\
    mul a a\n\
    mod a 5\n\
    snd a\n\
    set a 0\n\
    rcv a\n\
    jgz a -1\n\
    set a 1\n\
    jgz a -2";

    const INPUT2: &str =
    "snd 1\n\
    snd 2\n\
    snd p\n\
    rcv a\n\
    rcv b\n\
    rcv c\n\
    rcv d";

    #[test]
    fn test_1() {
        assert_eq!(4, part_1(&parse(INPUT)));
    }

    #[test]
    fn test_2() {
        assert_eq!(3, part_2(&parse(INPUT2)));
    }
}