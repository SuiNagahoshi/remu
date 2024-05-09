use crate::td4::compiler::Compiler;
use crate::td4::emulator::CpuEmulator;
use crate::td4::io::Port;
use crate::td4::parser;
use crate::td4::register::Register;
use crate::td4::rom::Rom;

pub fn td4(operations: Vec<String>) {
    let mut parser = parser::Parser::new(operations);
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

    let rom = Rom::new(program);
    let register = Register::new();
    let port = Port::new(0b0000, 0b0000);
    let mut emulator = CpuEmulator::with(register, port, rom);
    match emulator.exec() {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    }
}
