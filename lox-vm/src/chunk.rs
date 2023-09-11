pub enum OpCode {
    OpReturn,
    OpConstant,
}

pub struct Chunk {
    pub code: Vec<u8>,
    pub lines: Vec<usize>,
}

impl OpCode {
    pub fn name(&self) -> &'static str {
        match self {
            OpCode::OpReturn => "OP_RETURN",
	    OpCode::OpConstant => "OP_CONSTANT",
        }
    }

    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0 => Some(OpCode::OpReturn),
            _ => None,
        }
    }
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk { code: Vec::new(), lines: Vec::new() }
    }

    pub fn write(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
	self.lines.push(line);
    }
    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);
        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);

        if offset >= self.code.len() {
            println!("Attempt to disassemble beyond chunk end");
            return offset + 1;
        }

        let instruction = self.code[offset];
        match OpCode::from_u8(instruction) {
            Some(op) => self.simple_instruction(op.name(), offset),
            None => {
                println!("Unknown opcode {}", instruction);
                offset + 1
            }
        }
    }

    fn simple_instruction(&self, name: &'static str, offset: usize) -> usize {
        println!("{}", name);
        offset + 1
    }
}
