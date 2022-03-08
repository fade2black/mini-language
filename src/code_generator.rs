use crate::ast::{Ast, Function};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub struct CodeGenerator<'a> {
    asts: &'a Vec<Ast>,
    target: File,
}

impl<'a> CodeGenerator<'a> {
    pub fn new(asts: &'a Vec<Ast>, target: File) -> Self {
        Self { asts, target }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        let mut func_names = vec![];

        self.open_module()?;
        for ast in self.asts.iter() {
            self.to_wat(ast)?;
            let Ast::Definition(func) = ast;
            func_names.push(func.get_function_name());
        }
        self.export_functions(&func_names)?;
        self.close_module()?;

        Ok(())
    }

    fn to_wat(&mut self, ast: &Ast) -> std::io::Result<()> {
        match ast {
            Ast::Definition(def_node) => self.def_to_wat(def_node),
        }
    }

    fn def_to_wat(&mut self, function: &Function) -> std::io::Result<()> {
        self.write(function.to_wat().join("").as_str())?;
        Ok(())
    }

    fn export_functions(&mut self, func_names: &Vec<&str>) -> std::io::Result<()> {
        for name in func_names {
            self.write(format!("(export \"{}\" (func ${}))\n", name, name).as_str())?;
        }

        Ok(())
    }

    fn open_module(&mut self) -> std::io::Result<()> {
        self.write("(module\n")?;
        Ok(())
    }

    fn close_module(&mut self) -> std::io::Result<()> {
        self.write(")\n")?;
        Ok(())
    }

    pub fn builtin_funcs() -> HashMap<&'a str, &'a str> {
        let mut funcs = HashMap::new();
        funcs.insert("sqrt", "f32.sqrt");
        funcs.insert("ceil", "f32.ceil");
        funcs.insert("floor", "f32.floor");
        funcs.insert("trunc", "f32.trunc");
        funcs.insert("nearest", "f32.nearest");
        funcs.insert("abs", "f32.abs");
        funcs.insert("neg", "f32.neg");

        funcs
    }

    fn write(&mut self, line: &str) -> std::io::Result<()> {
        self.target.write_all(line.as_bytes())?;
        Ok(())
    }
}
