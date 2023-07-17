/*
These are type aliases just for convenience and readability
*/
type X = u8;
type Y = u8;
type N = u8;
type NN = u8;
type NNN = u16;

pub enum OpCode {
    Cls,
    Ret,
    Jmp(NNN),
    Call(NNN),
    SkipEq(X, NN),
    SkipNotEq(X, NN),
    SkipRegEq(X, Y),
    SkipRegNotEq(X, Y),
    Set(X, NN),
    AddImd(X, NN),
    SetReg(X, Y),
    Or(X, Y),
    And(X, Y),
    Xor(X, Y),
    Add(X, Y),
    Sub(X, Y),
    SubInv(X, Y),
    ShiftR(X, Y),
    ShiftL(X, Y),
    SetIndex(NNN),
    JmpOff(NNN),
    Rand(X, NN),
    Disp(X, Y, N),
    SkipKeyPress(X),
    SkipKeyNotPress(X),
    SetFromDelay(X),
    SetDelay(X),
    SetSound(X),
    AddIndex(X),
    GetKey(X),
    Font(X),
    DecConv(X),
    Store(X),
    Load(X),
    NotImplemented,
}
