use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
};

use reqwest::blocking::Client;

fn get_client() -> Result<Client, Box<dyn Error>> {
    use reqwest::header;
    let mut headers = header::HeaderMap::new();
    let session = get_cookie("secrets/aoc.cookie")?;
    headers.insert(
        header::COOKIE,
        header::HeaderValue::from_str(&format!("session={}", session))?,
    );
    Ok(Client::builder().default_headers(headers).build()?)
}

fn get_cookie(path: &str) -> Result<String, Box<dyn Error>> {
    if let Ok(mut file) = File::open(path) {
        let mut input = String::new();
        file.read_to_string(&mut input)?;
        return Ok(input);
    }
    Err(format!("No cookie available. Please insert AOC cookie into {:?}. \
    You can find the cookie by viewing the Cookies panel in the Application \
    tab of your browser debugger while on the AOC site.", path).into())
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