use crate::value::Value;

pub fn op_add(left: Value, right: Value) -> Value {
    match (left, right) {
        (Value::U8(lvalue), Value::U8(rvalue)) => Value::U8(lvalue + rvalue),
        (Value::U16(lvalue), Value::U16(rvalue)) => Value::U16(lvalue + rvalue),
        (Value::U32(lvalue), Value::U32(rvalue)) => Value::U32(lvalue + rvalue),
        (Value::U64(lvalue), Value::U64(rvalue)) => Value::U64(lvalue + rvalue),
        (Value::String(_), Value::String(_)) => unimplemented!(),
        _ => {
            unimplemented!()
        }
    }
}
pub fn op_sub(left: Value, right: Value) -> Value {
    match (left, right) {
        (Value::U8(lvalue), Value::U8(rvalue)) => Value::U8(lvalue - rvalue),
        (Value::U16(lvalue), Value::U16(rvalue)) => Value::U16(lvalue - rvalue),
        (Value::U32(lvalue), Value::U32(rvalue)) => Value::U32(lvalue - rvalue),
        (Value::U64(lvalue), Value::U64(rvalue)) => Value::U64(lvalue - rvalue),
        _ => {
            unimplemented!()
        }
    }
}
pub fn op_rem(left: Value, right: Value) -> Value {
    match (left, right) {
        (Value::U8(lvalue), Value::U8(rvalue)) => Value::U8(lvalue % rvalue),
        (Value::U16(lvalue), Value::U16(rvalue)) => Value::U16(lvalue % rvalue),
        (Value::U32(lvalue), Value::U32(rvalue)) => Value::U32(lvalue % rvalue),
        (Value::U64(lvalue), Value::U64(rvalue)) => Value::U64(lvalue % rvalue),
        _ => {
            unimplemented!()
        }
    }
}

pub fn op_less_than(left: Value, right: Value) -> Value {
    match (left, right) {
        (Value::U8(lvalue), Value::U8(rvalue)) => Value::Bool(lvalue < rvalue),
        (Value::U16(lvalue), Value::U16(rvalue)) => Value::Bool(lvalue < rvalue),
        (Value::U32(lvalue), Value::U32(rvalue)) => Value::Bool(lvalue < rvalue),
        (Value::U64(lvalue), Value::U64(rvalue)) => Value::Bool(lvalue < rvalue),
        _ => {
            unimplemented!()
        }
    }
}
pub fn op_equals(left: Value, right: Value) -> Value {
    match (left, right) {
        (Value::U8(lvalue), Value::U8(rvalue)) => Value::Bool(lvalue == rvalue),
        (Value::U16(lvalue), Value::U16(rvalue)) => Value::Bool(lvalue == rvalue),
        (Value::U32(lvalue), Value::U32(rvalue)) => Value::Bool(lvalue == rvalue),
        (Value::U64(lvalue), Value::U64(rvalue)) => Value::Bool(lvalue == rvalue),
        (Value::Bool(lvalue), Value::Bool(rvalue)) => Value::Bool(lvalue == rvalue),
        (Value::String(_), Value::String(_)) => unimplemented!(),
        (Value::Array(_), Value::Array(_)) => unimplemented!(),
        _ => unimplemented!(),
    }
}
pub fn op_not_equals(left: Value, right: Value) -> Value {
    match (left, right) {
        (Value::U8(lvalue), Value::U8(rvalue)) => Value::Bool(lvalue != rvalue),
        (Value::U16(lvalue), Value::U16(rvalue)) => Value::Bool(lvalue != rvalue),
        (Value::U32(lvalue), Value::U32(rvalue)) => Value::Bool(lvalue != rvalue),
        (Value::U64(lvalue), Value::U64(rvalue)) => Value::Bool(lvalue != rvalue),
        (Value::Bool(lvalue), Value::Bool(rvalue)) => Value::Bool(lvalue != rvalue),
        (Value::Array(_), Value::Array(_)) => unimplemented!(),
        (Value::String(_), Value::String(_)) => unimplemented!(),
        _ => unimplemented!(),
    }
}
