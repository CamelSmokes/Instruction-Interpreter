use crate::interpreter_error::InterpreterError;

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
    Array(ArrayValue),
}

impl Value {
    pub fn get_type(&self) -> VariableType {
        match self {
            Value::U8(_) => VariableType::U8,
            Value::U16(_) => VariableType::U16,
            Value::U32(_) => VariableType::U32,
            Value::U64(_) => VariableType::U64,
            Value::String(_) => VariableType::String,
            Value::Array(array) => array.get_type(),
            Value::Bool(_) => VariableType::Bool,
        }
    }
    pub fn is_number(&self) -> bool {
        matches!(self, Value::U8(_) | Value::U16(_) | Value::U32(_) | Value::U64(_))
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
    pub fn to_usize(&self) -> Result<usize, InterpreterError> {
        Ok(match self {
            Value::U8(v) => *v as usize,
            Value::U16(v) => *v as usize,
            Value::U32(v) => *v as usize,
            Value::U64(v) => *v as usize,
            _ => return Err(InterpreterError::ValueIsNotNumeric(self.clone())),
        })
    }
}

#[derive(Debug, Clone)]
pub enum ArrayValue {
    BoolArray(Vec<bool>), // could use packed bits
    U8Array(Vec<u8>),
    U16Array(Vec<u16>),
    U32Array(Vec<u32>),
    U64Array(Vec<u64>),
    StringArray(Vec<String>),
    ArrayArray(VariableType, Vec<ArrayValue>),
}

impl ArrayValue {
    pub fn new(array_type: VariableType) -> Self {
        match array_type {
            VariableType::U8 => ArrayValue::U8Array(Vec::new()),
            VariableType::U16 => ArrayValue::U16Array(Vec::new()),
            VariableType::U32 => ArrayValue::U32Array(Vec::new()),
            VariableType::U64 => ArrayValue::U64Array(Vec::new()),
            VariableType::String => ArrayValue::StringArray(Vec::new()),
            VariableType::Array(sub_array_type) => ArrayValue::ArrayArray(*sub_array_type, Vec::new()),
            VariableType::Bool => ArrayValue::BoolArray(Vec::new()),
        }
    }
    pub fn get_inner_type(&self) -> VariableType {
        match self {
            ArrayValue::BoolArray(_) => VariableType::Bool,
            ArrayValue::U8Array(_) => VariableType::U8,
            ArrayValue::U16Array(_) => VariableType::U16,
            ArrayValue::U32Array(_) => VariableType::U32,
            ArrayValue::U64Array(_) => VariableType::U64,
            ArrayValue::StringArray(_) => VariableType::String,
            ArrayValue::ArrayArray(a, _) => VariableType::Array(Box::from(a.clone())),
        }
    }
    pub fn get_type(&self) -> VariableType {
        VariableType::Array(Box::from(self.get_inner_type()))
    }
    pub fn set_index(&mut self, index: usize, value: Value) -> Result<(), InterpreterError> {
        match (self, value) {
            (ArrayValue::U8Array(a), Value::U8(v)) => *a.get_mut(index).ok_or(InterpreterError::ArrayIndexBeyondBounds(index))? = v,
            (ArrayValue::U16Array(a), Value::U16(v)) => *a.get_mut(index).ok_or(InterpreterError::ArrayIndexBeyondBounds(index))? = v,
            (ArrayValue::U32Array(a), Value::U32(v)) => *a.get_mut(index).ok_or(InterpreterError::ArrayIndexBeyondBounds(index))? = v,
            (ArrayValue::U64Array(a), Value::U64(v)) => *a.get_mut(index).ok_or(InterpreterError::ArrayIndexBeyondBounds(index))? = v,
            (ArrayValue::BoolArray(a), Value::Bool(v)) => *a.get_mut(index).ok_or(InterpreterError::ArrayIndexBeyondBounds(index))? = v,
            (ArrayValue::StringArray(a), Value::String(v)) => *a.get_mut(index).ok_or(InterpreterError::ArrayIndexBeyondBounds(index))? = v,
            (s, v) => return Err(InterpreterError::ArraySetValueWithIncompatibleType(s.get_type(), v.get_type())),
        }
        Ok(())
    }

    pub fn push(&mut self, value: Value) -> Result<(), InterpreterError> {
        match (self, value) {
            (ArrayValue::U8Array(a), Value::U8(v)) => a.push(v),
            (ArrayValue::U16Array(a), Value::U16(v)) => a.push(v),
            (ArrayValue::U32Array(a), Value::U32(v)) => a.push(v),
            (ArrayValue::U64Array(a), Value::U64(v)) => a.push(v),
            (ArrayValue::BoolArray(a), Value::Bool(v)) => a.push(v),
            (ArrayValue::StringArray(a), Value::String(v)) => a.push(v),
            (s, v) => return Err(InterpreterError::ArrayTypeIncompatibleWithPushValue(s.get_type(), v.get_type())),
        }
        Ok(())
    }
    pub fn get_index(&self, index: usize) -> Result<Value, InterpreterError> {
        fn get_index_internal(array: &ArrayValue, index: usize) -> Option<Value> {
            Some(match array {
                ArrayValue::BoolArray(v) => Value::Bool(*v.get(index)?),
                ArrayValue::U8Array(v) => Value::U8(*v.get(index)?),
                ArrayValue::U16Array(v) => Value::U16(*v.get(index)?),
                ArrayValue::U32Array(v) => Value::U32(*v.get(index)?),
                ArrayValue::U64Array(v) => Value::U64(*v.get(index)?),
                ArrayValue::StringArray(v) => Value::String(v.get(index)?.clone()),
                ArrayValue::ArrayArray(_, v) => Value::Array(v.get(index)?.clone()),
            })
        }
        get_index_internal(self, index).ok_or(InterpreterError::ArrayIndexBeyondBounds(index))
    }

    pub fn len(&self) -> usize {
        match self {
            ArrayValue::BoolArray(a) => a.len(),
            ArrayValue::U8Array(a) => a.len(),
            ArrayValue::U16Array(a) => a.len(),
            ArrayValue::U32Array(a) => a.len(),
            ArrayValue::U64Array(a) => a.len(),
            ArrayValue::StringArray(a) => a.len(),
            ArrayValue::ArrayArray(_, a) => a.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
