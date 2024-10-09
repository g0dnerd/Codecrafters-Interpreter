use crate::expression::*;
use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
    path::PathBuf,
};

pub struct AstGenerator {
    output_dir: String,
    types: Vec<String>,
}

impl AstGenerator {
    pub fn new(output_dir: String, types: Vec<String>) -> Self {
        Self { output_dir, types }
    }

    pub fn define_ast(&self, base_name: &str) {
        let file_name = format!("{base_name}.rs");
        let file_path: PathBuf = [&self.output_dir, &file_name].iter().collect();

        let mut stream = BufWriter::new(File::create(&file_path).unwrap());

        stream
            .write(b"use crate::expression::Expression;\n\n")
            .unwrap();
        stream
            .write(b"use crate::token::{ Token, LiteralValue };\n\n")
            .unwrap();
        stream.flush().unwrap();

        for t in &self.types {
            let typedef = t.split_once(':').expect("Invalid type definition: {t}");
            let name = typedef.0.trim();
            let fields = typedef.1.trim();
            self.define_type(&file_path, name, fields);
        }
    }

    fn define_type(&self, file_path: &PathBuf, base_name: &str, fields: &str) {
        let options = OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_path)
            .unwrap();
        let mut stream = BufWriter::new(options);

        let struct_name = format!("pub struct {base_name} {{");
        stream.write(struct_name.as_bytes()).unwrap();

        let field_vec = fields.split(", ");
        for field in field_vec {
            let field_def = field
                .split_once(' ')
                .expect("to be able to find a name in field");

            let field_type = match field_def.0 {
                "Expr" => "Box<dyn Expression>".as_bytes(),
                "Literal" => "Box<dyn LiteralValue>".as_bytes(),
                _ => field_def.0.as_bytes(),
            };
            let field_name = field_def.1.as_bytes();

            stream.write(b"\n\t").unwrap();
            stream.write(field_name).unwrap();
            stream.write(b": ").unwrap();
            stream.write(field_type).unwrap();
            stream.write(b",").unwrap();
        }
        stream.write(b"\n}\n\n").unwrap();
        stream.flush().unwrap();
    }
}

pub fn print_expr(expr: Box<dyn Expression>) {
    println!("{}", expr.accept());
}
