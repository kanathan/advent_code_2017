

fn main() {
    println!("{}", spinlock(382));
    println!("{}", part2(382, 50000000));
}


fn spinlock(step: usize) -> u16 {
    let mut buffer = vec![0];
    let mut pos = 0;

    for val in 1..=2017 {
        pos = (pos + step + 1) % buffer.len();
        buffer.insert(pos, val);
    }

    *buffer.get((pos+1)%buffer.len()).unwrap()
}

fn part2(step: usize, cycles: u32) -> u32 {
    let mut cur_idx_1 = 0;
    let mut buffer_size = 1;
    let mut cur_pos = 0;

    for val in 1..=cycles {
        cur_pos = (cur_pos + step) % buffer_size;
        if cur_pos == 0 {
            cur_idx_1 = val;
        }
        buffer_size += 1;
        cur_pos = (cur_pos + 1) % buffer_size;
    }

    cur_idx_1
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(638, spinlock(3));
    }

    #[test]
    fn test_2() {
        assert_eq!(9, part2(3, 9));
    }
}