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

    use OpCode::*;
    match first_nibble(first_byte) {
        0x00 if second_byte == 0xE0 => Cls,
        0x00 if second_byte == 0xEE => Ret,
        0x01 => Jmp(nnn),
        0x02 => Call(nnn),
        0x03 => SkipEq(x, nn),
        0x04 => SkipNotEq(x, nn),
        0x05 => SkipRegEq(x, y),
        0x09 => SkipRegNotEq(x, y),
        0x06 => Set(x, nn),
        0x07 => AddImd(x, nn),
        0x08 => match second_nibble(second_byte) {
            0x00 => SetReg(x, y),
            0x01 => Or(x, y),
            0x02 => And(x, y),
            0x03 => Xor(x, y),
            0x04 => Add(x, y),
            0x05 => Sub(x, y),
            0x07 => SubInv(x, y),
            0x06 => ShiftR(x, y),
            0x0E => ShiftL(x, y),
            _ => NotImplemented,
        },
        0x0A => SetIndex(nnn),
        0x0B => JmpOff(nnn),
        0x0C => Rand(x, nn),
        0x0D => Disp(x, y, n),
        0x0E => match second_byte {
            0x9E => SkipKeyPress(x),
            0xA1 => SkipKeyNotPress(x),
            _ => NotImplemented,
        },
        0x0F => match second_byte {
            0x07 => SetFromDelay(x),
            0x15 => SetDelay(x),
            0x18 => SetSound(x),
            0x1E => AddIndex(x),
            0x0A => GetKey(x),
            0x29 => Font(x),
            0x33 => DecConv(x),
            0x55 => Store(x),
            0x65 => Load(x),
            _ => NotImplemented,
        },
        _ => NotImplemented,
    }
}
