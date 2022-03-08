use minilang::code_generator::CodeGenerator;
use minilang::parser::Parser;
use std::process::Command;

use std::env;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Not enough arguments. Please specify input and output files names.");
        return Ok(());
    }

    let src = File::open(args[1].as_str())?;
    let target = File::create(args[2].as_str())?;

    let mut parser = Parser::new(src);
    parser.main_loop();

    let err_logger = parser.get_error_logger();

    if err_logger.has_errors() {
        for error in err_logger.iter() {
            println!("SYNTAX ERROR: {}", error);
        }
    } else {
        // Generate WebAssembly text
        CodeGenerator::new(parser.get_asts(), target).run()?;
        // Generate binary WebAssembly
        Command::new("sh")
            .arg("-c")
            .arg(format!("wat2wasm {}", args[2].as_str()))
            .output()
            .expect("failed to execute 'wat2wasm'");
    }

    Ok(())
}
