use anyhow::Result;
use clap::Parser;
use anyhow::anyhow;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, value_parser)]
    input: String,
}

fn parse(mut input: &str) -> Result<String> {
    let result = String::new();

    while let Some(start) = input.find("{") {
        let end = input.find("}")
            .ok_or(anyhow!("Missing closing bracket"))?;

        if start > end {
            return Err(anyhow!("Error while parsing"));
        }

        let inner = &input[start + 1..end];
        input = &input[end + 1..];
        println!("{}", inner);
    }

    Ok(result)
}

fn main() -> Result<()> {
    let Args { mut input } = Args::parse();
    parse(&mut input)?;
    Ok(())
}


