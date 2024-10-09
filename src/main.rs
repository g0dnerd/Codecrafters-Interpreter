use std::env;
use std::fs;
use std::process::ExitCode;

use codecrafters_interpreter::ast::print_expr;
use codecrafters_interpreter::expression::Expression;
use codecrafters_interpreter::parse::Parser;
use codecrafters_interpreter::scan::Scanner;
use codecrafters_interpreter::token::Token;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return ExitCode::SUCCESS;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            // eprintln!("Logs from your program will appear here!");
            let file_contents = read_file_contents(filename);
            match tokenize(file_contents) {
                Ok(scanner) => println!("{scanner}"),
                Err(scanner) => {
                    println!("{scanner}");
                    return ExitCode::from(65);
                }
            }
        }
        "parse" => {
            let file_contents = read_file_contents(filename);
            match tokenize(file_contents) {
                Ok(scanner) => match parse(scanner.tokens) {
                    Ok(expr) => print_expr(expr),
                    Err(_) => {
                        eprintln!("Damn.");
                        return ExitCode::from(65);
                    }
                },
                Err(_) => return ExitCode::from(65),
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}

fn read_file_contents(filename: &String) -> String {
    return fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    });
}

fn tokenize(file_contents: String) -> Result<Scanner, Scanner> {
    let mut scanner = Scanner::new(file_contents);
    scanner.scan_tokens();
    if scanner.has_error {
        return Err(scanner);
    }
    Ok(scanner)
}

fn parse(tokens: Vec<Token>) -> Result<Box<dyn Expression>, ()> {
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(expr) => return Ok(expr),
        Err(_) => return Err(()),
    }
}
