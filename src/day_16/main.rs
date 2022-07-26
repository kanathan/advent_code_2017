use std::collections::HashMap;

use itertools::Itertools;



fn main() {
    let instructions = parse(include_str!("input"));
    println!("{:?}", part_1(16, &instructions));
    println!("{:?}", part_2(16, &instructions));
}


#[derive(Clone, Copy, Debug)]
enum Instructions {
    Spin(usize),
    SwapIdx((usize, usize)),
    Swap((char, char)),
}


fn parse(input: &str) -> Vec<Instructions> {
    let mut instructions = vec![];

    for instr in input.split(',') {
        instructions.push(
            match instr.chars().next().unwrap() {
                's' => Instructions::Spin(instr[1..].parse().unwrap()),
                'x' => {
                    let (idx1_s, idx2_s) = instr[1..].trim().split('/').collect_tuple().unwrap();
                    let idx1 = if let Ok(i) = idx1_s.parse() {i} else {panic!("Unable to parse {idx1_s} in {instr}")};
                    let idx2 = if let Ok(i) = idx2_s.parse() {i} else {panic!("Unable to parse {idx2_s} in {instr}")};
                    Instructions::SwapIdx((idx1, idx2))
                }
                'p' => Instructions::Swap((instr.chars().nth(1).unwrap(), instr.chars().nth(3).unwrap())),
                _ => unreachable!(),
            }
        );
    }

    instructions
}


fn part_1(length: u8, instructions: &[Instructions]) -> String {
    let input: String = (0..length).map(|i| (b'a' + i) as char).collect();
    dance(&input, instructions)
}


fn part_2(length: u8, instructions: &[Instructions]) -> String {
    let mut programs: String = (0..length).map(|i| (b'a' + i) as char).collect();
    let mut history: HashMap<String, String> = HashMap::new();

    for i in 0..1000000000 {
        
        programs = if history.contains_key(&programs) {
            history.get(&programs).unwrap().clone()
        } else {
            let new_program = dance(&programs, instructions);
            history.insert(programs.clone(), new_program.clone());
            new_program
        };
        if i % 10000000 == 0 {
            println!("{}% complete with {} combinations",(i as f32 / 1000000000.0 * 100.0), history.len());
        }
    }

    programs
}


fn dance(input: &str, instructions: &[Instructions]) -> String {
    let mut programs: Vec<char> = input.chars().collect();
    let mut offset = 0;
    let len = programs.len();

    for instr in instructions {
        match *instr {
            Instructions::Spin(i) => offset = (offset + i) % programs.len(),
            Instructions::SwapIdx((i1, i2)) => {
                programs.swap((len + i1 - offset) % len, (len + i2 - offset) % len)
            },
            Instructions::Swap((c1, c2)) => {
                let i1 = programs.iter().position(|&x| x==c1).unwrap();
                let i2 = programs.iter().position(|&x| x==c2).unwrap();
                programs.swap(i1, i2);
            },
        }
    }

    programs.rotate_right(offset);
    programs.iter().collect()
}



#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "s1,x3/4,pe/b";

    #[test]
    fn test_1() {
        println!("{:?}", part_1(5, &parse(INPUT)));
    }

    #[test]
    fn test_2() {
        parse("s15,x12/7");
    }
}