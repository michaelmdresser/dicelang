use colored::*;
use std::env;
use std::io;

fn main() {
    let mut input = String::new();

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let concated = args
            .iter()
            .skip(1)
            .fold(String::new(), |acc, arg| acc + arg);
        input = concated;
    } else {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    }

    input = input.trim().to_string();
    if input.is_empty() {
        return;
    }

    let expr = match dicelang::parse(&input) {
        Ok(expr) => expr,
        Err(e) => {
            println!("Failed to parse: {e}");
            std::process::exit(1);
        }
    };
    let (result, rolls) = expr.eval();

    let rolls_pretty = rolls.iter().fold(String::new(), |acc, roll| {
        acc + &format!(
            "{}{} ",
            format!("d{}:", roll.die).dimmed(),
            format!("{}", roll.result),
        )
    });
    if !rolls_pretty.is_empty() {
        println!("{rolls_pretty}");
    }
    println!(
        "{} => {}",
        format!("{expr}").yellow(),
        format!("{result}").bold(),
    );
}
