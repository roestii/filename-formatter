use std::collections::HashMap;

use anyhow::Result;
use chrono::{Datelike, Local};
use clap::Parser;
use anyhow::anyhow;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, value_parser)]
    input: String,
}

fn parse(mut input: &str) -> Result<String> {
    let mut result = String::new();

    while let Some(start) = input.find("{") {
        let end = input.find("}")
            .ok_or(anyhow!("Missing closing bracket"))?;

        if start > end {
            return Err(anyhow!("Error while parsing"));
        }

        let inner = &input[start + 1..end];
        let parsed = parse_to_date(inner)?;

        result += &input[0..start];
        result += &parsed;
        input = &input[end + 1..];
    }

    if !input.is_empty() {
        result += &input;
    }

    Ok(result)
}

fn parse_to_date(input: &str) -> Result<String> {
    let mut parsed = String::new();
    let mut buf = String::new();

    let now = Local::now(); 
    
    for c in input.chars() {
        buf += &c.to_string(); 

        match buf.as_str() {
            "MM" => {
                let month = format!("{}", now.format("%m"));
                parsed += &month;
                buf = String::new();
            },
            "yyyy" => {
                let year = format!("{}", now.format("%Y"));
                parsed += &year;
                buf = String::new();
            },
            _ => {
                continue;
            },
        }
    }
    
    if !buf.is_empty() {
        return Err(anyhow!("Failed to parse date"));
    }

    Ok(parsed)
}

fn main() -> Result<()> {
    let Args { mut input } = Args::parse();
    let parsed = parse(&mut input)?;
    println!("{}", parsed);
    Ok(())
}


