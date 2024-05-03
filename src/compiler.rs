use crate::error::EmulatorError;
use crate::token::{Register, Token};

pub struct Compiler;

impl Default for Compiler {
    fn default() -> Self {
        Self
    }
}

impl Compiler {
    pub fn new() -> Self {
        Self
    }

    pub fn compile(&self, tokens: Vec<Token>) -> Result<Vec<u8>, EmulatorError> {
        if tokens.is_empty() {
            return Err(EmulatorError::new(
                "Failed to start to compile: Token list is empty",
            ));
        }

        let mut result = Vec::new();

        for token in tokens {
            let program = match token {
                Token::Mov(Register::A, immediate) => self.generate_binary_code(0b0011, immediate),
                Token::Mov(Register::B, immediate) => self.generate_binary_code(0b0111, immediate),
                Token::MovAB => self.generate_binary_code_with_zero_padding(0b0001),
                Token::MovBA => self.generate_binary_code_with_zero_padding(0b0100),
                Token::Add(Register::A, immediate) => self.generate_binary_code(0b0000, immediate),
                Token::Add(Register::B, immediate) => self.generate_binary_code(0b0101, immediate),
                Token::Jmp(immediate) => self.generate_binary_code(0b1111, immediate),
                Token::Jnc(immediate) => self.generate_binary_code(0b1110, immediate),
                Token::In(Register::A) => self.generate_binary_code_with_zero_padding(0b0010),
                Token::In(Register::B) => self.generate_binary_code_with_zero_padding(0b0110),
                Token::OutB => self.generate_binary_code_with_zero_padding(0b1001),
                Token::OutIm(immediate) => self.generate_binary_code(0b1011, immediate),
            };
            result.push(program);
        }

        Ok(result)
    }

    fn generate_binary_code(&self, operation: u8, immediate: u8) -> u8 {
        let shift_operation = operation << 4;
        let shift_data = immediate & 0x0f;
        shift_operation | shift_data
    }

    #[allow(clippy::erasing_op)]
    fn generate_binary_code_with_zero_padding(&self, operation: u8) -> u8 {
        let shift_operation = operation << 4;
        let zero_padding = 0b0000 & 0x0f;
        shift_operation | zero_padding
    }
}
