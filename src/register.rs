pub struct Register {
    register_a: u8,
    register_b: u8,
    carry_flag: u8,
    program_counter: u8,
}

impl Default for Register {
    fn default() -> Self {
        Self {
            register_a: u8::default(),
            register_b: u8::default(),
            carry_flag: u8::default(),
            program_counter: u8::default(),
        }
    }
}

impl Register {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn program_counter(&self) -> u8 {
        self.program_counter
    }

    pub fn set_program_counter(&mut self, new_value: u8) {
        self.program_counter = new_value;
    }

    pub fn increment_program_counter(&mut self) {
        self.program_counter += 1;
    }

    pub fn carry_flag(&self) -> u8 {
        self.carry_flag
    }

    pub fn set_carry_flag(&mut self, new_value: u8) {
        self.carry_flag = new_value;
    }

    pub fn register_a(&self) -> u8 {
        self.register_a
    }

    pub fn set_register_a(&mut self, new_value: u8) {
        self.register_a = new_value;
    }

    pub fn register_b(&self) -> u8 {
        self.register_b
    }

    pub fn set_register_b(&mut self, new_value: u8) {
        self.register_b = new_value;
    }
}
