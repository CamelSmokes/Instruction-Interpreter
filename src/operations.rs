use num_traits::Num;

use crate::{interpreter_error::InterpreterError, value::Value};

pub fn op_add(left: Value, right: Value) -> Result<Value, InterpreterError> {
    if !(left.is_number() && right.is_number()) {
        return Err(InterpreterError::OperandNotNumeric);
    }
    Ok(match (left, right) {
        (Value::U8(lvalue), Value::U8(rvalue)) => Value::U8(lvalue.wrapping_add(rvalue)),
        (Value::U16(lvalue), Value::U16(rvalue)) => Value::U16(lvalue.wrapping_add(rvalue)),
        (Value::U32(lvalue), Value::U32(rvalue)) => Value::U32(lvalue.wrapping_add(rvalue)),
        (Value::U64(lvalue), Value::U64(rvalue)) => Value::U64(lvalue.wrapping_add(rvalue)),
        (Value::String(_), Value::String(_)) => unimplemented!(),
        _ => return Err(InterpreterError::OperandsNotSameType),
    })
}
pub fn op_sub(left: Value, right: Value) -> Result<Value, InterpreterError> {
    if !(left.is_number() && right.is_number()) {
        return Err(InterpreterError::OperandNotNumeric);
    }
    Ok(match (left, right) {
        (Value::U8(lvalue), Value::U8(rvalue)) => Value::U8(lvalue.wrapping_sub(rvalue)),
        (Value::U16(lvalue), Value::U16(rvalue)) => Value::U16(lvalue.wrapping_sub(rvalue)),
        (Value::U32(lvalue), Value::U32(rvalue)) => Value::U32(lvalue.wrapping_sub(rvalue)),
        (Value::U64(lvalue), Value::U64(rvalue)) => Value::U64(lvalue.wrapping_sub(rvalue)),
        _ => return Err(InterpreterError::OperandsNotSameType),
    })
}
#[inline]
fn internal_rem<T: Num>(l: T, r: T) -> Result<T, InterpreterError> {
    if r.is_zero() {
        return Err(InterpreterError::OperatorDivideByZero);
    };
    Ok(l % r)
}

pub fn op_rem(left: Value, right: Value) -> Result<Value, InterpreterError> {
    if !(left.is_number() && right.is_number()) {
        return Err(InterpreterError::OperandNotNumeric);
    }
    Ok(match (left, right) {
        (Value::U8(lvalue), Value::U8(rvalue)) => Value::U8(internal_rem(lvalue, rvalue)?),
        (Value::U16(lvalue), Value::U16(rvalue)) => Value::U16(internal_rem(lvalue, rvalue)?),
        (Value::U32(lvalue), Value::U32(rvalue)) => Value::U32(internal_rem(lvalue, rvalue)?),
        (Value::U64(lvalue), Value::U64(rvalue)) => Value::U64(internal_rem(lvalue, rvalue)?),
        _ => return Err(InterpreterError::OperandsNotSameType),
    })
}

pub fn op_less_than(left: Value, right: Value) -> Result<Value, InterpreterError> {
    if !(left.is_number() && right.is_number()) {
        return Err(InterpreterError::OperandNotNumeric);
    }
    Ok(match (left, right) {
        (Value::U8(lvalue), Value::U8(rvalue)) => Value::Bool(lvalue < rvalue),
        (Value::U16(lvalue), Value::U16(rvalue)) => Value::Bool(lvalue < rvalue),
        (Value::U32(lvalue), Value::U32(rvalue)) => Value::Bool(lvalue < rvalue),
        (Value::U64(lvalue), Value::U64(rvalue)) => Value::Bool(lvalue < rvalue),
        _ => return Err(InterpreterError::OperandsNotSameType),
    })
}
pub fn op_equals(left: Value, right: Value) -> Result<Value, InterpreterError> {
    Ok(match (left, right) {
        (Value::U8(lvalue), Value::U8(rvalue)) => Value::Bool(lvalue == rvalue),
        (Value::U16(lvalue), Value::U16(rvalue)) => Value::Bool(lvalue == rvalue),
        (Value::U32(lvalue), Value::U32(rvalue)) => Value::Bool(lvalue == rvalue),
        (Value::U64(lvalue), Value::U64(rvalue)) => Value::Bool(lvalue == rvalue),
        (Value::Bool(lvalue), Value::Bool(rvalue)) => Value::Bool(lvalue == rvalue),
        (Value::String(_), Value::String(_)) => unimplemented!(),
        (Value::Array(_), Value::Array(_)) => unimplemented!(),
        _ => return Err(InterpreterError::OperandsNotSameType),
    })
}
pub fn op_not_equals(left: Value, right: Value) -> Result<Value, InterpreterError> {
    Ok(match (left, right) {
        (Value::U8(lvalue), Value::U8(rvalue)) => Value::Bool(lvalue != rvalue),
        (Value::U16(lvalue), Value::U16(rvalue)) => Value::Bool(lvalue != rvalue),
        (Value::U32(lvalue), Value::U32(rvalue)) => Value::Bool(lvalue != rvalue),
        (Value::U64(lvalue), Value::U64(rvalue)) => Value::Bool(lvalue != rvalue),
        (Value::Bool(lvalue), Value::Bool(rvalue)) => Value::Bool(lvalue != rvalue),
        (Value::Array(_), Value::Array(_)) => unimplemented!(),
        (Value::String(_), Value::String(_)) => unimplemented!(),
        _ => unimplemented!(),
    })
}
