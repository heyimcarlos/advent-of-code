use day_01::{util, INPUT_ENDPOINT};

fn main() {
    let input_exists = std::path::Path::new("input.txt").exists();
    if !input_exists {
        let contents = util::fetch(INPUT_ENDPOINT).unwrap();
        let _ = std::fs::write("input.txt", contents);
    }

    let contents = std::fs::read_to_string("input.txt").unwrap();
    let output = p1(contents);
    dbg!(output);
}

fn p1(input: String) -> String {
    let output = input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter_map(|character| character.to_digit(10));
            let first = digits.next().expect("should be a number");
            let last = digits.last();

            match last {
                Some(num) => format!("{first}{num}"),
                None => format!("{first}{first}"),
            }
            .parse::<u32>()
            .expect("should be a valid number")
        })
        .sum::<u32>();
    output.to_string()

    // @INFO: first implementation

    // let mut sum: i32 = 0;
    // for line in input.lines() {
    //     let mut current_digits = vec![];
    //     for ch in line.chars() {
    //         if ch.is_digit(10) {
    //             current_digits.push(ch);
    //         }
    //     }
    //     let first = current_digits.first().unwrap();
    //     let last = current_digits.last().unwrap();
    //     let combined = format!("{}{}", first, last).parse::<i32>();
    //     sum += combined.unwrap();
    // }
    // sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let file_exists = std::path::Path::new("input.txt").exists();
        if !file_exists {
            let contents = util::fetch(INPUT_ENDPOINT).unwrap();
            let _ = std::fs::write("input.txt", contents);
        }
        let result = p1(std::fs::read_to_string("input.txt").unwrap());
        assert_eq!(result, "56465");
    }
}
