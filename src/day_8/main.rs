use regex::Regex;
use std::collections::HashMap;




fn main() {
    let instrs = parse(include_str!("input"));
    let output = part_1(&instrs);
    println!("{} {}", output.0, output.1);
}


#[derive(Clone)]
struct Instruction {
    reg: String,
    op: Op,
    val: i32,
    test_reg: String,
    test: Test,
    test_val: i32,
}

#[derive(Clone)]
enum Op {
    Add,
    Sub,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone)]
enum Test {
    LT,
    GT,
    LTE,
    GTE,
    EQ,
    NEQ,
}


fn parse(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r"(?P<reg>\w+) (?P<op>\w+) (?P<val>-?\d+) if (?P<test_reg>\w+) (?P<test>\S+) (?P<test_val>-?\d+)").unwrap();
    let mut instructions = vec![];

    for line in input.lines() {
        let cap = re.captures(line).unwrap();

        let op = match cap.name("op").unwrap().as_str() {
            "inc" => Op::Add,
            "dec" => Op::Sub,
            _ => unimplemented!("{}", cap.name("op").unwrap().as_str()),
        };
        let test = match cap.name("test").unwrap().as_str() {
            "<" => Test::LT,
            "<=" => Test::LTE,
            ">" => Test::GT,
            ">=" => Test::GTE,
            "==" => Test::EQ,
            "!=" => Test::NEQ,
            _ => unimplemented!("{}", cap.name("test").unwrap().as_str()),
        };
        instructions.push(Instruction {
            reg: cap.name("reg").unwrap().as_str().to_string(),
            op,
            val: cap.name("val").unwrap().as_str().parse().unwrap(),
            test_reg: cap.name("test_reg").unwrap().as_str().to_string(),
            test,
            test_val: cap.name("test_val").unwrap().as_str().parse().unwrap(),
        });
    }

    instructions
}


fn part_1(instrs: &[Instruction]) -> (i32, i32) {
    let mut registers: HashMap<String, i32> = HashMap::new();
    let mut max_val = i32::MIN;

    for instr in instrs.iter() {
        let test_reg_val = *registers.entry(instr.test_reg.clone()).or_insert(0);
        let test_result = match instr.test {
            Test::LT => test_reg_val < instr.test_val,
            Test::GT => test_reg_val > instr.test_val,
            Test::LTE => test_reg_val <= instr.test_val, 
            Test::GTE => test_reg_val >= instr.test_val,
            Test::EQ => test_reg_val == instr.test_val,
            Test::NEQ => test_reg_val != instr.test_val, 
        };
        if test_result {
            let reg_val = registers.entry(instr.reg.clone()).or_insert(0);
            match instr.op {
                Op::Add => *reg_val += instr.val,
                Op::Sub => *reg_val -= instr.val,
            };
            max_val = max_val.max(*registers.get(&instr.reg).unwrap());
        }
    }

    (
        *registers.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap().1,
        max_val
    )
    
}


#[cfg(test)]
mod test {
    //use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, 1);
    }
}