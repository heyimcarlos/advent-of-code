use std::{fs::File, io::Write};

fn main() {
    let input_exists = std::path::Path::new("src/bin/input1.txt").exists();
    if !input_exists {
        fetch_input().unwrap();
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

async fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::COOKIE,
        reqwest::header::HeaderValue::from_static("session=53616c7465645f5fb7ed99a37c6d30273f56ef9dc63fc6089c05d0fe59c2eaa39164e5b93634fd5e381afc6970e822a2ebd062eeb861e2da2abcd9548afc140b;")
    );

    let res = client
        .get(url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}

fn fetch_input() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = tokio::runtime::Runtime::new()?;
    let future = fetch_url("https://adventofcode.com/2023/day/1/input");
    let res = runtime.block_on(future)?;

    if res.len() > 0 {
        let mut file = File::create("src/bin/input1.txt")?;
        let _ = file.write_all(res.as_bytes());
    }
    println!("fetch_input res: {res}");

    Ok(())
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
