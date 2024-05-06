use remu::td4::compiler::Compiler;
use remu::td4::emulator::CpuEmulator;
use remu::td4::io::Port;
use remu::td4::parser::Parser;
use remu::td4::register::Register;
use remu::td4::rom::Rom;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("ERROR: Invalid args.");
    }

    let file = BufReader::new(File::open(args.get(1).unwrap()).expect("ERROR: File not found."));
    let operations = file
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    //for i in &operations {
    //    println!("{}", i);
    //}
    println!("{:?}", operations);

    let mut parser = Parser::new(operations);
    let tokens = match parser.parse() {
        Ok(tokens) => tokens,
        Err(err) => panic!("{:?}", err),
    };

    println!("{:?}", tokens);

    let compiler = Compiler::new();
    let program = match compiler.compile(tokens) {
        Ok(program) => program,
        Err(err) => panic!("{:?}", err),
    };

    println!("{:?}", program);

    /*
    let operations = file
        .chars()
        .into_iter()
        .map(|char| char.to_digit(10000).unwrap() as u8)
        .collect();
    */

    let rom = Rom::new(program);
    let register = Register::new();
    let port = Port::new(0b0000, 0b0000);
    let mut emulator = CpuEmulator::with(register, port, rom);
    match emulator.exec() {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    }
}
