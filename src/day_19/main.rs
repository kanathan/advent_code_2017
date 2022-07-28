

fn main() {
    let grid = parse(include_str!("input"));
    println!("{:?}", travel(&grid));
}


fn travel(grid: &[Vec<char>]) -> (String, usize) {
    let mut output = String::new();
    let mut steps = 1;

    // Find starting point
    let mut row = 0;
    let mut col = grid[0].iter().position(|&c| c == '|').unwrap();
    let mut dir = Dir::Down;

    loop {
        match dir {
            Dir::Up => {
                if row > 0 {
                    row -= 1
                } else {
                    break
                }
            },
            Dir::Down => {
                if row < grid.len() - 1 {
                    row += 1
                } else {
                    break
                }
            },
            Dir::Left => {
                if col > 0 {
                    col -= 1
                } else {
                    break
                }
            },
            Dir::Right => {
                if col < grid[row].len() - 1 {
                    col += 1
                } else {
                    break
                }
            },
        };

        let next_pos = grid[row][col];

        match next_pos {
            '|' | '-' => (), // Keep going same direction
            'A'..='Z' => output.push(next_pos),
            '+' => {
                match dir {
                    Dir::Up | Dir::Down => {
                        if col > 0 && grid[row][col-1] != ' ' {
                            dir = Dir::Left;
                        } else {
                            dir = Dir::Right;
                        }
                    },
                    Dir::Left | Dir::Right => {
                        if row > 0 && grid[row-1][col] != ' ' {
                            dir = Dir::Up;
                        } else {
                            dir = Dir::Down;
                        }
                    }
                }
            },
            ' ' => break,
            _ => unimplemented!()
        }
        steps += 1;


    }


    (output, steps)
}


enum Dir {
    Up,
    Down,
    Left,
    Right,
}


fn parse(input: &str) -> Vec<Vec<char>> {
    let mut grid = vec![];

    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    grid
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "\
\x20   |          \n\
\x20   |  +--+    \n\
\x20   A  |  C    \n\
   F---|----E|--+ \n\
\x20   |  |  |  D \n\
\x20   +B-+  +--+ ";

    #[test]
    fn test_1() {
        let grid = parse(INPUT);
        let output = travel(&grid);
        assert_eq!("ABCDEF", output.0);
        assert_eq!(38, output.1);
    }
}