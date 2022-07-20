

fn main() {
    let lengths: Vec<usize> = include_str!("input")
        .lines().next().unwrap()
        .split(',')
        .filter_map(|s| s.parse().ok()).collect();

    println!("{}", part1(&lengths, 256));

    println!("{:x}", part2(&parse(include_str!("input"))));

}


fn parse(input: &str) -> Vec<u8> {
    let mut output: Vec<u8> = input
        .lines().next().unwrap()
        .chars()
        .map(|c| c as u8)
        .collect();
    output.append(&mut vec![17, 31, 73, 47, 23]);
    output
}


fn part1(lengths: &[usize], size: usize) -> usize {
    let mut list: Vec<usize> = (0..size).collect();

    let mut pos = 0;

    for (skip, &length) in lengths.iter().enumerate() {
        for i in 0..(length / 2) {
            list.swap((pos + i) % size, (pos + length - i - 1) % size);
        }

        pos += length + skip;
    }

    list[0] * list[1]
}


fn part2(lengths: &[u8]) -> u128 {
    let mut list: Vec<_> = (0..256).collect();
    let len = list.len();

    let mut pos = 0;
    let mut skip = 0;

    for _round in 0..64 {
        for &length in lengths {
            for i in 0..(length as usize / 2) {
                list.swap((pos as usize + i) % len, (pos as usize + length as usize - i - 1) % len);
            }

            pos += length as usize + skip;
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
        assert_eq!(12, part1(&[3, 4, 1, 5], 5));
    }
}