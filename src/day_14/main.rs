use std::{collections::VecDeque, thread::panicking};


fn main() {
    println!("{}", part_1("amgozmfv"));
    println!("{}", part_2("amgozmfv"));
}


fn part_1(input: &str) -> usize {
    (0..128)
        .map(|i| {
            let output_hash = knot_hash(&format!("{input}-{i}"));
            format!("{:b}", output_hash)
                .chars()
                .filter(|&c| c == '1')
                .count()
        })
        .sum()
}


fn part_2(input: &str) -> i16 {
    let mut disk_map: Vec<Vec<i16>> =
        (0..128)
        .map(|i| {
            let output_hash = knot_hash(&format!("{input}-{i}"));
            format!("{:0128b}", output_hash)
                .chars()
                .map(|c| {
                    match c {
                        '0' => 0,
                        '1' => -1,
                        _ => unreachable!("Invalid char {c}"),
                    }
                })
                .collect()
        })
        .collect();
    
    let mut region_count = 0;
    for row in 0..128 {
        for col in 0..128 {
            if disk_map[row][col] == -1 {
                region_count += 1;
                map_region(&mut disk_map, region_count, row, col);
            }
        }
    }
    region_count
}


fn map_region(disk_map: &mut Vec<Vec<i16>>, region_num: i16, row: usize, col: usize) {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::from([(row, col)]);
    
    while let Some((cur_row, cur_col)) = queue.pop_front() {
        if disk_map[cur_row][cur_col] == -1 {
            disk_map[cur_row][cur_col] = region_num;
            if cur_row > 0 && disk_map[cur_row-1][cur_col] == -1 {
                queue.push_back((cur_row-1, cur_col));
            }
            if cur_col > 0 && disk_map[cur_row][cur_col-1] == -1 {
                queue.push_back((cur_row, cur_col-1));
            }
            if cur_row < 127 && disk_map[cur_row+1][cur_col] == -1 {
                queue.push_back((cur_row+1, cur_col));
            }
            if cur_col < 127 && disk_map[cur_row][cur_col+1] == -1 {
                queue.push_back((cur_row, cur_col+1));
            }
        }
    }    
}


fn knot_hash(input: &str) -> u128 {
    let mut list: Vec<_> = (0..256).collect();
    let len = list.len();

    let mut pos = 0;
    let mut skip = 0;

    for _round in 0..64 {
        for val in input.chars().map(|c| c as u8).chain(vec![17, 31, 73, 47, 23]) {
            for i in 0..(val as usize / 2) {
                list.swap((pos as usize + i) % len, (pos as usize + val as usize - i - 1) % len);
            }

            pos += val as usize + skip;
            skip += 1;
        }
    }

    let mut output: u128 = 0;
    for chunk in list.chunks(16) {
        output <<= 8;
        output |= (chunk.iter().fold(0, |acc, &x| acc ^ x) as u8) as u128
    }

    output
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(knot_hash("flqrgnkx-0") >> 120, 0b11010100);
        assert_eq!(knot_hash("flqrgnkx-3") >> 120, 0b10101101);
        assert_eq!(knot_hash("flqrgnkx-7") >> 120, 0b11010110);
    }

    #[test]
    fn test_2() {
        assert_eq!(part_1("flqrgnkx"), 8108);
    }

    #[test]
    fn test_3() {
        assert_eq!(part_2("flqrgnkx"), 1242);
    }
}