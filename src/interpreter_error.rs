use crate::value::{ArrayValue, FunctionIdType, Value, VariableIdType, VariableType};

#[derive(Debug, Clone)]
pub enum InterpreterError {
    VariableDoesNotExist(VariableIdType),
    FunctionDoesNotExist(FunctionIdType),
    AttemptAssignedDifferentTypes(VariableType, VariableType),

    OperandsNotSameType,
    OperandNotNumeric,

    // Function Calling/Callstack/Return/ControlFlow
    NoEntryFunction,
    NoReturnValue,
    ExpectingReturnCallToVoidFunction(FunctionIdType),
    VoidCallToNonVoidFunction(FunctionIdType),
    CallstackReferencesUnknownFunction(FunctionIdType),
    FunctionCallParameterStackEmptyPop(FunctionIdType),
    FunctionCallParametersInvalid(FunctionIdType, bool),
    GotoNonBoolean,

    // Value related
    ValueIsNotNumeric(Value),

    // Array related
    ArraySetValueWithIncompatibleType(VariableType, VariableType),
    ArrayIndexWithNonNumericType(Value),
    ArrayIndexBeyondBounds(usize),
    ArrayTypeIncompatibleWithPushValue(VariableType, VariableType),
    ArrayOperationOnNonArrayValue(VariableType),
}
