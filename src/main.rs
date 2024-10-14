use clap::{Args, Parser, Subcommand};
use std::{fs, process::ExitCode};

use codecrafters_interpreter::{
    ast::print_expr,
    environment::Environment,
    expression::Expression,
    interpret::{interpret_single_expr, Interpreter},
    parse,
    scan::Scanner,
    statement::Statement,
    token::Token,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Tokenize(FilenameArg),
    Parse(FilenameArg),
    Evaluate(FilenameArg),
    Run(FilenameArg),
}

#[derive(Args, Debug)]
struct FilenameArg {
    filename: String,
}

fn main() -> ExitCode {
    let args = Cli::parse();

    let parse_err_exit_code: ExitCode = ExitCode::from(65);
    let runtime_err_exit_code: ExitCode = ExitCode::from(70);

    match &args.command {
        Commands::Tokenize(f) => {
            let file_contents =
                fs::read_to_string(&f.filename).expect("unable to read the given file");
            match tokenize(file_contents) {
                Ok(scanner) => println!("{scanner}"),
                Err(scanner) => {
                    println!("{scanner}");
                    return parse_err_exit_code;
                }
            }
        }
        Commands::Parse(f) => {
            let file_contents =
                fs::read_to_string(&f.filename).expect("unable to read the given file");
            match tokenize(file_contents) {
                Ok(scanner) => match parse_print_single_expr(scanner.tokens) {
                    Ok(expr) => print_expr(&expr),
                    Err(_) => return parse_err_exit_code,
                },
                Err(_) => return parse_err_exit_code,
            }
        }
        Commands::Evaluate(f) => {
            let file_contents =
                fs::read_to_string(&f.filename).expect("unable to read the given file");
            match tokenize(file_contents) {
                Ok(scanner) => match parse_print_single_expr(scanner.tokens) {
                    Ok(expr) => {
                        let mut environment = Environment::new(None);
                        match interpret_single_expr(expr, &mut environment) {
                            Ok(_) => return ExitCode::SUCCESS,
                            Err(_) => return runtime_err_exit_code,
                        }
                    }
                    Err(_) => return runtime_err_exit_code,
                },
                Err(_) => return parse_err_exit_code,
            }
        }
        Commands::Run(f) => {
            let file_contents =
                fs::read_to_string(&f.filename).expect("unable to read the given file");
            match tokenize(file_contents) {
                Ok(scanner) => match parse(scanner.tokens) {
                    Ok(stmts) => {
                        let mut interpreter = Interpreter::new(stmts);
                        match interpreter.interpret() {
                            Ok(_) => return ExitCode::SUCCESS,
                            Err(_) => return runtime_err_exit_code,
                        }
                    }
                    Err(_) => return parse_err_exit_code,
                },
                Err(_) => return parse_err_exit_code,
            }
        }
    }
    ExitCode::SUCCESS
}

fn tokenize(file_contents: String) -> Result<Scanner, Scanner> {
    let mut scanner = Scanner::new(file_contents);
    scanner.scan_tokens();
    if scanner.has_error {
        return Err(scanner);
    }
    Ok(scanner)
}

fn parse_print_single_expr(tokens: Vec<Token>) -> Result<Box<dyn Expression>, parse::ParserError> {
    let mut parser = parse::Parser::new(tokens);
    parser.parse_single_expr()
}

fn parse(tokens: Vec<Token>) -> Result<Vec<Box<dyn Statement>>, parse::ParserError> {
    let mut parser = parse::Parser::new(tokens);
    parser.parse()
}
