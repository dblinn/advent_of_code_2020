use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
};

use reqwest::blocking::Client;

fn get_client() -> Result<Client, Box<dyn Error>> {
    use reqwest::header;
    let mut headers = header::HeaderMap::new();
    let session = std::env::var("AOC_SESSION_COOKIE")?;
    headers.insert(
        header::COOKIE,
        header::HeaderValue::from_str(&format!("session={}", session))?,
    );
    Ok(Client::builder().default_headers(headers).build()?)
}

pub fn get_puzzle_input(day: u8) -> Result<String, Box<dyn Error>> {
    if day == 0 || day > 25 {
        Err("day out of range")?;
    }

    let path = format!("input/day{}.txt", day);
    if let Ok(mut file) = File::open(&path) {
        let mut input = String::new();
        file.read_to_string(&mut input)?;
        return Ok(input);
    }

    let client = get_client()?;
    let input = client
        .get(&format!("https://adventofcode.com/2020/day/{}/input", day))
        .send()?
        .text()?;

    {
        std::fs::create_dir_all("input")?;
        let mut file = File::create(path)?;
        file.write_all(input.as_bytes())?;
    }

    Ok(input)
}

#[derive(serde::Serialize)]
struct AnswerForm {
    level: u8,
    answer: String,
}

fn submit_answer<S: ToString>(day: u8, level: u8, answer: S) -> Result<bool, Box<dyn Error>> {
    fn inner(day: u8, level: u8, answer: String) -> Result<bool, Box<dyn Error>> {
        if day == 0 || day > 25 {
            Err("day out of range")?;
        }
        if level < 1 || level > 2 {
            Err("level out of range")?;
        }

        let path = format!("answers/day{}l{}.txt", day, level);
        if let Ok(mut file) = File::open(&path) {
            let mut buf = String::new();
            file.read_to_string(&mut buf)?;
            return Ok(answer == buf.trim());
        }

        let client = get_client()?;
        let response = client
            .post(&format!("https://adventofcode.com/2020/day/{}/answer", day))
            .form(&AnswerForm {
                level,
                answer: answer.clone(),
            })
            .send()?
            .text()?;

        let is_correct = !response.contains("That's not the right answer.");
        if is_correct {
            std::fs::create_dir_all("answers")?;
            let mut file = File::create(path)?;
            writeln!(file, "{}", answer)?;
        }

        Ok(is_correct)
    }

    inner(day, level, answer.to_string())
}

pub fn check_answer<S: ToString>(day: u8, level: u8, answer: S) -> Result<(), Box<dyn Error>> {
    let s_answer = answer.to_string();
    let is_correct = submit_answer(day, level, answer)?;
    if is_correct {
        println!(
            "day: {}, level: {}; {} is the correct answer.",
            day, level, s_answer
        );
    } else {
        eprintln!(
            "day: {}, level: {}; {} is NOT the correct answer.",
            day, level, s_answer
        );
        std::process::exit(1);
    }
    Ok(())
}