use remu::emulator::CpuEmulator;
use remu::io::Port;
use remu::register::Register;
use remu::rom::Rom;
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
    for i in &operations {
        println!("{}", i);
    }

    let rom = Rom::new(program);
    let register = Register::new();
    let port = Port::new(0b0000, 0b0000);
    let mut emulator = CpuEmulator::with(register, port, rom);
    match emulator.exec() {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    }
}
