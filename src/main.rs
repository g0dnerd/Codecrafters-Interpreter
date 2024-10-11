use std::env;
use std::fs;
use std::process::ExitCode;

use codecrafters_interpreter::environment::Environment;
use codecrafters_interpreter::expression::Expression;
use codecrafters_interpreter::ast::print_expr;
use codecrafters_interpreter::interpret::interpret_single_expr;
use codecrafters_interpreter::interpret::Interpreter;
use codecrafters_interpreter::parse::Parser;
use codecrafters_interpreter::parse::ParserError;
use codecrafters_interpreter::scan::Scanner;
use codecrafters_interpreter::token::Token;
use codecrafters_interpreter::statement::Statement;

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
                Ok(scanner) => match parse_(scanner.tokens) {
                    Ok(expr) => {
                        print_expr(&expr);
                    },
                    Err(_) => {
                        eprintln!("Damn.");
                        return ExitCode::from(65);
                    }
                },
                Err(_) => return ExitCode::from(65),
            }
        },
        "evaluate" => {
            let file_contents = read_file_contents(filename);
            match tokenize(file_contents) {
                Ok(scanner) => match parse_(scanner.tokens) {
                    Ok(expr) => {
                        let mut environment = Environment::new();
                        match interpret_single_expr(expr, &mut environment) {
                            Ok(_) => return ExitCode::from(0),
                            Err(_) => return ExitCode::from(70)
                        }
                    },
                    Err(_) => return ExitCode::from(70)
                },
                Err(_) => return ExitCode::from(65),
            }
        },
        "run" => {
            let file_contents = read_file_contents(filename);
            match tokenize(file_contents) {
                Ok(scanner) => match parse(scanner.tokens) {
                    Ok(stmts) => {
                        let mut interpreter = Interpreter::new(stmts);
                        match interpreter.interpret() {
                            Ok(_) => return ExitCode::SUCCESS,
                            Err(_) => return ExitCode::from(70)
                        }
                    },
                    Err(_) => return ExitCode::from(65)
                },
                Err(_) => return ExitCode::from(65),
            }
        },
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

fn parse_(tokens: Vec<Token>) -> Result<Box<dyn Expression>, ParserError> {
    let mut parser = Parser::new(tokens);
    match parser.parse_() {
        Ok(expr) => return Ok(expr),
        Err(e) => return Err(e)
    }
}

fn parse(tokens: Vec<Token>) -> Result<Vec<Box<dyn Statement>>, ParserError> {
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(stmts) => return Ok(stmts),
        Err(e) => return Err(e)
    }
}
