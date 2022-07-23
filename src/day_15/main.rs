

fn main() {
    println!("{}", part_1(883, 879, 40_000_000));
    println!("{}", part_2(883, 879, 5_000_000))
}


fn part_1(a_init: usize, b_init: usize, samples: usize) -> usize {
    let mut matches = 0;

    let mut a = a_init;
    let mut b = b_init;

    for _ in 0..samples {
        a = (a * 16807) % 2147483647;
        b = (b * 48271) % 2147483647;

        if (a & 0xFFFF) == (b & 0xFFFF) {
            matches += 1;
        }
    }

    matches
}


fn part_2(a_init: usize, b_init: usize, samples: usize) -> usize {
    let mut matches = 0;

    let mut a = a_init;
    let mut b = b_init;

    for _ in 0..samples {
        loop {
            a = (a * 16807) % 2147483647;
            if a % 4 == 0 { break }
        }
        loop {
            b = (b * 48271) % 2147483647;
            if b % 8 == 0 { break }
        }

        if (a & 0xFFFF) == (b & 0xFFFF) {
            matches += 1;
        }
    }

    matches
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, part_1(65, 8921, 5));
    }

    #[test]
    fn test_2() {
        assert_eq!(588, part_1(65, 8921, 40_000_000));
    }

    #[test]
    fn test_3() {
        assert_eq!(309, part_2(65, 8921, 5_000_000));
    }
}