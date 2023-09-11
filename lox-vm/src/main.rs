mod chunk;

use chunk::{Chunk, OpCode};

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(OpCode::OpReturn as u8, 1);
    chunk.write(OpCode::OpConstant as u8, 1);
    chunk.disassemble("test chunk");
}
