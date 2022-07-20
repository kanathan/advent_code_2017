

fn main() {
    let cleaned_input = clean_garbage(include_str!("input"));
    println!("{}", part1(&cleaned_input));
}


fn part1(input: &str) -> u32 {

    let mut level = 0;
    let mut score = 0;

    for ch in input.chars() {
        match ch {
            '{' => level += 1,
            '}' => { score += level; level -= 1 },
            _ => {},
        }
    }

    if level != 0 {
        println!("Invalid level: {}", level);
    }

    score
}


fn clean_garbage(input: &str) -> String {
    let mut output = String::new();
    let mut garbage_score = 0;

    let mut ignore_flag = false;
    let mut garbage_active = false;
    for ch in input.chars() {
        if ignore_flag {
            ignore_flag = false;
            continue;
        }

        if ch == '!' {
            ignore_flag = true;
            continue;
        }

        if garbage_active {
            if ch == '>' { garbage_active = false; }
            else { garbage_score += 1; }
            continue;
        }

        if ch == '<' {
            garbage_active = true;
            continue;
        }

        output.push(ch);
    }

    println!("Garbage score: {}", garbage_score);

    output
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!("", clean_garbage("<>"));
        assert_eq!("", clean_garbage("<random characters>"));
        assert_eq!("", clean_garbage("<<<<>"));
        assert_eq!("", clean_garbage("<{!>}>"));
        assert_eq!("", clean_garbage("<!!>"));
        assert_eq!("", clean_garbage("<!!!>>"));
        assert_eq!("", clean_garbage("<{o\"i!a,<{i<a>"));
    }
}