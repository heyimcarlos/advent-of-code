use std::{fs::File, io::Write};
mod util;

fn main() {
    let input_exists = std::path::Path::new("src/bin/input1.txt").exists();
    if !input_exists {
        util::fetch().unwrap();
    }

    let input = read_input("src/bin/input1.txt");
    let output = p1(input);
    dbg!(output);
}

fn read_input(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}

fn p1(input: String) -> i32 {
    let mut sum: i32 = 0;
    for line in input.lines() {
        let mut current_digits = vec![];
        for ch in line.chars() {
            if ch.is_digit(10) {
                current_digits.push(ch);
            }
        }
        let first = current_digits.first().unwrap();
        let last = current_digits.last().unwrap();
        let combined = format!("{}{}", first, last).parse::<i32>();
        sum += combined.unwrap();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let result = p1(read_input("src/bin/input1.txt"));
        assert_eq!(result, 56465);
    }
}
