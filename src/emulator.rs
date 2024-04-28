use crate::error::EmulatorError;
use crate::io::Port;
use crate::opcode::Opcode;
use crate::register::Register;
use crate::rom::Rom;
use num_traits::FromPrimitive;

pub struct CpuEmulator {
    register: Register,
    port: Port,
    rom: Rom,
}

impl CpuEmulator {
    pub fn with(register: Register, port: Port, rom: Rom) -> Self {
        assert!(
            rom.size() <= 16,
            "Maximum memory size is 16. This program can't work."
        );
        Self {
            register,
            port,
            rom,
        }
    }

    fn fetch(&self) -> u8 {
        let program_counter = self.register.program_counter();
        if self.rom.size() <= program_counter {
            return 0;
        }

        self.rom.read(program_counter)
    }

    fn decode(&self, data: u8) -> Result<(Opcode, u8), EmulatorError> {
        let opelation = data >> 4;
        let immediate = data & 0x0f;

        if let Some(opcode) = FromPrimitive::from_u8(opelation) {
            match opcode {
                Opcode::AddA
                | Opcode::AddB
                | Opcode::MovA
                | Opcode::MovB
                | Opcode::MovA2B
                | Opcode::MovB2A
                | Opcode::Jmp
                | Opcode::Jnc
                | Opcode::OutIm => Ok((opcode, immediate)),
                Opcode::InA | Opcode::InB | Opcode::OutB => Ok((opcode, 0)),
            }
        } else {
            Err(EmulatorError::new("No match for opcode."))
        }
    }

    pub fn exec(&mut self) -> Result<(), EmulatorError> {
        loop {
            let data = self.fetch();
            let (opcode, immediate) = self.decode(data)?;

            match opcode {
                Opcode::MovA => self.mov_a(immediate),
                Opcode::MovB => self.mov_b(immediate),
                Opcode::AddA => self.add_a(immediate),
                Opcode::AddB => self.add_b(immediate),
                Opcode::MovA2B => self.mov_a2b(),
                Opcode::MovB2A => self.mov_b2a(),
                Opcode::Jmp => self.jmp(immediate),
                Opcode::Jnc => self.jnc(immediate),
                Opcode::InA => self.in_a(),
                Opcode::InB => self.in_b(),
                Opcode::OutB => self.out_b(),
                Opcode::OutIm => self.out_immedilate(immediate),
            };

            if opcode != Opcode::Jmp && opcode != Opcode::Jnc {
                self.register.increment_program_counter();
            }

            if self.does_halt() {
                return Ok(());
            }
        }
    }

    fn does_halt(&self) -> bool {
        self.register.program_counter() >= self.rom.size()
    }

    fn mov_a(&mut self, immediate: u8) {
        self.register.set_register_a(immediate);
        self.register.set_carry_flag(0);
    }

    fn mov_b(&mut self, immediate: u8) {
        self.register.set_register_b(immediate);
        self.register.set_carry_flag(0);
    }

    fn mov_a2b(&mut self) {
        let register_b = self.register.register_b();
        self.register.set_register_a(register_b);
        self.register.set_carry_flag(0);
    }

    fn mov_b2a(&mut self) {
        let register_a = self.register.register_a();
        self.register.set_register_b(register_a);
        self.register.set_carry_flag(0);
    }

    fn add_a(&mut self, immediate: u8) {
        let existence = self.register.register_a();
        let new_value = existence + immediate;

        if new_value > 0x0f {
            self.register.set_carry_flag(1);
        }

        self.register.set_register_a(new_value & 0x0f);
    }

    fn add_b(&mut self, immediate: u8) {
        let existence = self.register.register_b();
        let new_value = existence + immediate;

        if new_value > 0x0f {
            self.register.set_carry_flag(1);
        }

        self.register.set_register_b(new_value & 0x0f);
    }

    fn jmp(&mut self, immediate: u8) {
        self.register.set_program_counter(immediate);
        self.register.set_carry_flag(0)
    }

    fn jnc(&mut self, immediate: u8) {
        if self.register.carry_flag() == 0 {
            self.register.set_program_counter(immediate);
        }
        self.register.set_carry_flag(0)
    }

    fn in_a(&mut self) {
        let input_port = self.port.input();
        self.register.set_register_a(input_port);
        self.register.set_carry_flag(0);
    }

    fn in_b(&mut self) {
        let input_port = self.port.input();
        self.register.set_register_b(input_port);
        self.register.set_carry_flag(0);
    }

    fn out_b(&mut self) {
        let register_b = self.register.register_b();
        self.port.set_output(register_b);
        self.register.set_carry_flag(0);
        println!("Port B Output: {}", self.port.output());
    }

    fn out_immediate(&mut self, immediate: u8) {
        self.port.set_output(immediate);
        self.register.set_carry_flag(0);
        println!("Port Output: {}", self.port.output());
    }
}
