use crate::opcode::OpCode;

/// Decodes opcode from bytes into [OpCode](crate::opcode::Opcode), for easier use
/// Chip-8 instructions are 16 bits (2 bytes), hence 2 parameters.
/// Two seperates bytes are used in favour of single 2 byte integer for simplicity
pub fn decode_op(first_byte: u8, second_byte: u8) -> OpCode {
    let first_nibble = |byte| (byte & 0xF0) >> 4;
    let second_nibble = |byte| byte & 0x0F;

    let x = second_nibble(first_byte);
    let y = first_nibble(second_byte);
    let n = second_nibble(second_byte);
    let nn = second_byte;
    let nnn = u16::from_be_bytes([second_nibble(first_byte), second_byte]);

    match first_nibble(first_byte) {
        0x00 if second_byte == 0xE0 => OpCode::Cls,
        0x01 => OpCode::Jmp(nnn),
        0x06 => OpCode::Set(x, nn),
        0x07 => OpCode::AddImd(x, nn),
        0x0A => OpCode::SetIndex(nnn),
        0x0D => OpCode::Disp(x, y, n),
        _ => OpCode::NotImplemented,
    }
}
