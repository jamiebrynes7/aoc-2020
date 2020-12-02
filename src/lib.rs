use anyhow::Result;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn input_lines(
    day: u8,
) -> Result<impl Iterator<Item = std::result::Result<String, io::Error>>> {
    let file = File::open(format!("./src/input/day{:0width$}.txt", day, width = 2))?;

    Ok(BufReader::new(file).lines())
}
