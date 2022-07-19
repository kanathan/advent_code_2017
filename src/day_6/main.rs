use std::collections::{HashSet, HashMap};

fn main() {
    let mem_state = parse(include_str!("input"));
    let (cycles, loop_size) = find_loop(&mem_state);
    //println!("{}", part_1(&mem_state));
    println!("{} {}", cycles, loop_size);
}


fn parse(input: &str) -> Vec<u16> {
    let mut mem_state = vec![];
    for val_str in input.split_ascii_whitespace() {
        mem_state.push(val_str.parse().unwrap());
    }
    mem_state
}


fn part_1(init_mem_state: &Vec<u16>) -> u32 {
    let mut mem_states: HashSet<Vec<u16>> = HashSet::new();
    let mut mem_state = init_mem_state.clone();
    let mut cycle_count = 0;

    while !mem_states.contains(&mem_state) {
        cycle_count += 1;
        mem_states.insert(mem_state.clone());
        redistribute(&mut mem_state);
    }

    cycle_count
}

fn find_loop(init_mem_state: &Vec<u16>) -> (u32, u32) {
    let mut mem_states: HashMap<Vec<u16>, u32> = HashMap::new();
    let mut mem_state = init_mem_state.clone();
    let mut cycle_count = 0;

    while !mem_states.contains_key(&mem_state) {
        mem_states.insert(mem_state.clone(), cycle_count);
        cycle_count += 1;
        redistribute(&mut mem_state);
    }

    (cycle_count, cycle_count - *mem_states.get(&mem_state).unwrap())
}

fn redistribute(mem_state: &mut Vec<u16>) {
    let mut idx = 0;
    let mut max_blocks = 0;
    for (cur_idx, mem_block) in mem_state.iter().enumerate() {
        if max_blocks < *mem_block {
            idx = cur_idx;
            max_blocks = *mem_block;
        }
    }
    mem_state[idx] = 0;
    while max_blocks > 0 {
        idx = (idx + 1) % mem_state.len();
        mem_state[idx] += 1;
        max_blocks -= 1;
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let init_mem_state: Vec<u16> = vec![0, 2, 7, 0];

        assert_eq!(5, part_1(&init_mem_state));
    }
}