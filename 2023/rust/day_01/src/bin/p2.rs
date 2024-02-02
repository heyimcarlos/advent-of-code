use day_01::{util, INPUT_ENDPOINT};

fn main() {
    let input_exists = std::path::Path::new("input.txt").exists();
    if !input_exists {
        let contents = util::fetch(INPUT_ENDPOINT).unwrap();
        let _ = std::fs::write("input.txt", contents);
    }
    let contents = std::fs::read_to_string("input.txt").unwrap();
    println!("p2 result: {}", p2(contents));
}

fn p2(contents: String) -> String {
    let output = contents
        .lines()
        .map(|line| {
            let mut result = vec![];
            for (idx, char) in line.chars().enumerate() {
                let reduced_line = &line[idx..];

                if reduced_line.starts_with("one") {
                    result.push(1);
                } else if reduced_line.starts_with("two") {
                    result.push(2);
                } else if reduced_line.starts_with("three") {
                    result.push(3);
                } else if reduced_line.starts_with("four") {
                    result.push(4);
                } else if reduced_line.starts_with("five") {
                    result.push(5);
                } else if reduced_line.starts_with("six") {
                    result.push(6);
                } else if reduced_line.starts_with("seven") {
                    result.push(7);
                } else if reduced_line.starts_with("eight") {
                    result.push(8);
                } else if reduced_line.starts_with("nine") {
                    result.push(9);
                } else if char.is_digit(10) {
                    let val = char.to_digit(10).unwrap() as i32;
                    result.push(val);
                }
            }
            let first = result.first().expect("should be a number");
            let last = result.last().expect("should be a number");
            let total = format!("{first}{last}")
                .parse::<i32>()
                .expect("should be a valid number");
            total
        })
        .sum::<i32>();

    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] // turns function into a test;
    fn test_p2() {
        let result = p2("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            .to_string());
        assert_eq!(result, "281".to_string());
    }
}
