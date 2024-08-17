use std::borrow::Cow;

pub type StringIdType = u16;

pub type VariableIdType = u16;
pub type ArrayIdType = u16;
pub type FunctionIdType = u16;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableType {
    U8,
    U16,
    U32,
    U64,
    String,
    Array(Box<VariableType>),
    Bool,
}

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    String(String),
    Array(VariableType, Vec<Value>),
}

impl Value {
    pub fn get_type(&self) -> VariableType {
        match self {
            Value::U8(_) => VariableType::U8,
            Value::U16(_) => VariableType::U16,
            Value::U32(_) => VariableType::U32,
            Value::U64(_) => VariableType::U64,
            Value::String(_) => VariableType::String,
            Value::Array(arr_type, _) => VariableType::Array(Box::new(arr_type.clone())),
            Value::Bool(_) => VariableType::Bool,
        }
    }
    pub fn is_number(&self) -> bool {
        matches!(
            self,
            Value::U8(_) | Value::U16(_) | Value::U32(_) | Value::U64(_)
        )
    }
    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Bool(_))
    }
    pub fn get_bool(&self) -> Option<bool> {
        if let Value::Bool(b) = self {
            return Some(*b);
        }
        None
    }
    pub fn to_usize(&self) -> Result<usize, ()> {
        Ok(match self {
            Value::U8(v) => *v as usize,
            Value::U16(v) => *v as usize,
            Value::U32(v) => *v as usize,
            Value::U64(v) => *v as usize,
            _ => todo!(),
        })
    }
}
