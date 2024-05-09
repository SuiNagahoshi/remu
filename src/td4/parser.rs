use super::error::EmulatorError;
use super::token::{Register, Token};
pub struct Parser {
    position: usize,
    source: Vec<String>,
}

impl Parser {
    pub fn new(operations: Vec<String>) -> Parser {
        let mut source = Vec::new();

        for operation in operations {
            let split: Vec<&str> = operation.split(' ').collect();

            println!("{:?}", split);

            for line in split {
                let cloned = line.to_string();
                source.push(cloned);
            }
        }

        Parser {
            position: 0,
            source,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Token>, EmulatorError> {
        let mut result = Vec::new();

        loop {
            let operation = self.source.get(self.position);

            if operation.is_none() {
                break;
            }

            let operation = operation.unwrap();

            if operation == "mov" {
                self.position += 1;

                let lhs = self
                    .source
                    .get(self.position)
                    .ok_or_else(|| EmulatorError::new("Failed to parse mov left hand"))?;

                self.position += 1;

                let rhs = self
                    .source
                    .get(self.position)
                    .ok_or_else(|| EmulatorError::new("Failed to parse mov right hand"))?;

                let token = if lhs == "B" && rhs == "A" {
                    Token::MovBA
                } else if lhs == "A" && rhs == "B" {
                    Token::MovAB
                } else {
                    Token::Mov(
                        Register::from(lhs.to_string()),
                        self.from_binary_to_decimal(rhs)?,
                    )
                };

                result.push(token);
            }

            if operation == "add" {
                self.position += 1;

                let lhs = self
                    .source
                    .get(self.position)
                    .ok_or_else(|| EmulatorError::new("Failed to parse mov left hand"))?;

                self.position += 1;

                let rhs = self
                    .source
                    .get(self.position)
                    .ok_or_else(|| EmulatorError::new("Failed to parse mov right hand"))?;

                let token = Token::Add(
                    Register::from(lhs.to_string()),
                    self.from_binary_to_decimal(rhs)?,
                );

                result.push(token);
            }

            if operation == "jmp" {
                self.position += 1;

                let immediate = self
                    .source
                    .get(self.position)
                    .ok_or_else(|| EmulatorError::new("Failed to parse 'jmp' immediate value"))?;

                result.push(Token::Jmp(self.from_binary_to_decimal(immediate)?));
            }

            if operation == "jnc" {
                self.position += 1;

                let immediate = self
                    .source
                    .get(self.position)
                    .ok_or_else(|| EmulatorError::new("Failed to parse 'jnc' immediate value"))?;

                result.push(Token::Jnc(self.from_binary_to_decimal(immediate)?));
            }

            if operation == "in" {
                self.position += 1;

                let lhs = self.source.get(self.position).unwrap();

                result.push(Token::In(Register::from(lhs.to_string())));
            }

            if operation == "out" {
                self.position += 1;

                let immediate = self
                    .source
                    .get(self.position)
                    .ok_or_else(|| EmulatorError::new("Failed to parse 'out' immediate value"))?;

                if immediate == "B" {
                    result.push(Token::OutB);
                } else {
                    result.push(Token::OutIm(self.from_binary_to_decimal(immediate)?));
                }
            }

            self.position += 1;
        }

        Ok(result)
    }

    fn from_binary_to_decimal(&self, text: impl Into<String>) -> Result<u8, EmulatorError> {
        let ret = text.into();

        let binary_to_decimal = u8::from_str_radix(&ret, 2);

        binary_to_decimal
            .map_err(|_| EmulatorError::new(&format!("Failed to parse string: {}", ret)))
    }
}
