/*
These are type aliases just for convenience and readability
*/
type X = u8;
type Y = u8;
type N = u8;
type NN = u8;
type NNN = u16;

pub enum OpCode {
    Cls,                // 00E0
    Ret,                // 00EE
    Jmp(NNN),           // 1NNN
    Call(NNN),          // 2NNN
    SkipEq(X, NN),      // 3XNN
    SkipNotEq(X, NN),   // 4XNN
    SkipRegEq(X, Y),    // 5XY0
    SkipRegNotEq(X, Y), // 9XY0
    Set(X, NN),         // 6XNN
    AddImd(X, NN),      // 7XNN
    SetReg(X, Y),       // 8XY0
    Or(X, Y),           // 8XY1
    And(X, Y),          // 8XY2
    Xor(X, Y),          // 8XY3
    Add(X, Y),          // 8XY4
    Sub(X, Y),          // 8XY5
    SubInv(X, Y),       // 8XY7
    ShiftR(X, Y),       // 8XY6
    ShiftL(X, Y),       // 8XYE
    SetIndex(NNN),      // ANNN
    JmpOff(NNN),        // BNNN
    Rand(X, NN),        // CXNN
    Disp(X, Y, N),      // DXYN
    SkipKeyPress(X),    // EX9E
    SkipKeyNotPress(X), // EXA1
    SetFromDelay(X),    // FX07
    SetDelay(X),        // FX15
    SetSound(X),        // FX18
    AddIndex(X),        // FX1E
    GetKey(X),          // FX0A
    Font(X),            // FX29
    DecConv(X),         // FX33
    Store(X),           // FX55
    Load(X),            // FX65
    NotImplemented,
}
