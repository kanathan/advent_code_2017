

fn main() {
    let instrs = parse(include_str!("input"));
    println!("{}", get_steps(instrs.clone()));
    println!("{}", get_steps_p2(instrs));
}


fn get_steps(mut instrs: Vec<i32>) -> i32 {
    let mut pos = 0;
    let mut steps = 0;

    while pos >= 0 && pos < instrs.len() as i32 {
        steps += 1;
        let new_pos = pos + instrs[pos as usize];
        instrs[pos as usize] += 1;
        pos = new_pos;        
    }

    steps
}


fn get_steps_p2(mut instrs: Vec<i32>) -> i32 {
    let mut pos = 0;
    let mut steps = 0;

    while pos >= 0 && pos < instrs.len() as i32 {
        steps += 1;
        let new_pos = pos + instrs[pos as usize];
        if instrs[pos as usize] >= 3 {
            instrs[pos as usize] -= 1;
        } else {
            instrs[pos as usize] += 1;
        }
        pos = new_pos;        
    }

    steps
}


fn parse(input: &str) -> Vec<i32> {
    let mut instrs = vec![];

    for line in input.lines() {
        instrs.push(line.parse().unwrap());
    }

    instrs
}

#[cfg(test)]
mod test {
    //use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, 1);
    }
}