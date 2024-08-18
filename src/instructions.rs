use crate::value::{FunctionIdType, Value, VariableIdType};

#[derive(Debug, Clone)]
#[repr(align(64))]
pub enum Instruction {
    Set(VariableIdType, VariableIdType),
    SetI(VariableIdType, Value),
    SetArrayIndex(VariableIdType, VariableIdType, VariableIdType), // array[a] = b
    SetArrayIndexI(VariableIdType, VariableIdType, Value),         // array[a] = I
    SetArrayIIndex(VariableIdType, Value, VariableIdType),         // array[I] = b
    // SetArrayIIndexI(VariableIdType, Value, Value),                 // array[I] = J
    GetArrayIndex(VariableIdType, VariableIdType, VariableIdType),
    GetArrayIndexI(VariableIdType, VariableIdType, Value),
    // Arithmetic
    Add(VariableIdType, VariableIdType),
    Sub(VariableIdType, VariableIdType),
    Mul(VariableIdType, VariableIdType),
    Div(VariableIdType, VariableIdType),
    Rem(VariableIdType, VariableIdType),
    AddI(VariableIdType, Value),
    SubI(VariableIdType, Value),
    MulI(VariableIdType, Value),
    DivI(VariableIdType, Value),
    RemI(VariableIdType, Value),
    // Comparison
    LessThan(VariableIdType, VariableIdType, VariableIdType),
    LessThanI(VariableIdType, VariableIdType, Value),
    GreaterThan(VariableIdType, VariableIdType, VariableIdType),
    GreaterThanI(VariableIdType, VariableIdType, Value),

    LessThanOrEqual(VariableIdType, VariableIdType, VariableIdType),
    LessThanOrEqualI(VariableIdType, VariableIdType, Value),
    GreaterThanOrEqual(VariableIdType, VariableIdType, VariableIdType),
    GreaterThanOrEqualI(VariableIdType, VariableIdType, Value),

    Equals(VariableIdType, VariableIdType, VariableIdType),
    EqualsI(VariableIdType, VariableIdType, Value),
    NotEquals(VariableIdType, VariableIdType, VariableIdType),
    NotEqualsI(VariableIdType, VariableIdType, Value),
    // Logical
    Or(VariableIdType, VariableIdType),
    And(VariableIdType, VariableIdType),
    Xor(VariableIdType, VariableIdType),
    Not(VariableIdType),
    // Control
    Goto(usize),                       // used for loop breaks and continues
    GotoIfTrue(usize, VariableIdType), // used for

    PushFunctionParameterStack(VariableIdType),
    CallVoidFunction(FunctionIdType),
    CallFunction(FunctionIdType, VariableIdType),
    CallNativeVoidFunction(FunctionIdType),

    CallNativeVoidMethod(VariableIdType, FunctionIdType),
    CallNativeMethod(VariableIdType, VariableIdType, FunctionIdType),

    Return(VariableIdType),
}
