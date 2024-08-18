use std::{any::Any, collections::HashMap};

use crate::{
    instructions::Instruction,
    interpreter_error::InterpreterError,
    operations::{op_add, op_equals, op_less_than, op_not_equals, op_rem, op_sub},
    value::{ArrayValue, FunctionIdType, Value, VariableIdType, VariableType},
};

#[derive(Debug, Clone, Default)]
pub struct Function {
    instructions: Vec<Instruction>,
    variables: Vec<VariableType>,
    parameters: Vec<VariableType>,
    return_type: Option<VariableType>,
}

impl Function {
    pub fn new(parameters: &[VariableType], return_type: Option<VariableType>) -> Self {
        let mut func = Function {
            instructions: Vec::new(),
            variables: Vec::new(),
            parameters: parameters.to_vec(),
            return_type,
        };

        for parameter_type in parameters.iter() {
            func.register_variable(parameter_type.clone());
        }

        func
    }
    pub fn set_instructions(&mut self, instructions: Vec<Instruction>) {
        self.instructions = instructions;
    }
    pub fn register_variable(&mut self, var_type: VariableType) -> VariableIdType {
        self.variables.push(var_type);
        self.variables.len() as u16
    }
    pub fn register_variables(&mut self, var_types: &[VariableType]) {
        for var_type in var_types {
            self.register_variable(var_type.clone());
        }
    }
}

#[derive(Debug, Clone)]
struct ExecutionContext {
    variables: Vec<Value>,
    function_parameter_stack: Vec<Value>,
    function_id: FunctionIdType,
    expecting_return_value: Option<VariableIdType>,
    instruction_counter: usize,
}

impl ExecutionContext {
    fn new(func: &Function, function_id: FunctionIdType) -> Self {
        let mut variables: Vec<Value> = Vec::with_capacity(8);
        for b in func.variables.iter() {
            let default_value = match b {
                VariableType::U8 => Value::U8(0),
                VariableType::U16 => Value::U16(0),
                VariableType::U32 => Value::U32(0),
                VariableType::U64 => Value::U64(0),
                VariableType::Bool => Value::Bool(false),
                VariableType::String => Value::String(String::new()),
                VariableType::Array(arr_type) => Value::Array(ArrayValue::new(*arr_type.clone())),
            };

            variables.push(default_value);
        }

        ExecutionContext {
            variables,
            function_parameter_stack: Vec::new(),
            instruction_counter: 0,
            function_id,
            expecting_return_value: None,
        }
    }
    fn get_variable(&self, var_id: VariableIdType) -> Result<&Value, InterpreterError> {
        if let Some(v) = self.variables.get(var_id as usize) {
            Ok(v)
        } else {
            Err(InterpreterError::VariableDoesNotExist(var_id))
        }
    }
    fn get_variable_pair(&self, var1_id: VariableIdType, var2_id: VariableIdType) -> Result<(&Value, &Value), InterpreterError> {
        let var1 = self.get_variable(var1_id)?;
        let var2 = self.get_variable(var2_id)?;
        Ok((var1, var2))
    }
    fn get_variable_mut(&mut self, var_id: VariableIdType) -> Result<&mut Value, InterpreterError> {
        if let Some(v) = self.variables.get_mut(var_id as usize) {
            Ok(v)
        } else {
            Err(InterpreterError::VariableDoesNotExist(var_id))
        }
    }
    fn set_variable(&mut self, var_id: VariableIdType, value: Value) -> Result<(), InterpreterError> {
        let Some(current_value) = self.variables.get(var_id as usize) else {
            return Err(InterpreterError::VariableDoesNotExist(var_id));
        };

        if current_value.get_type() != value.get_type() {
            return Err(InterpreterError::AttemptAssignedDifferentTypes(
                current_value.get_type(),
                value.get_type(),
            ));
        }
        if let Some(v) = self.variables.get_mut(var_id as usize) {
            *v = value;
        } else {
            return Err(InterpreterError::VariableDoesNotExist(var_id));
        }
        Ok(())
    }
    #[allow(dead_code)]
    fn print_state(&self, program: &Program) {
        println!("---");
        for (id, variable) in self.variables.iter().cloned().enumerate() {
            println!(
                "Variable ID {id: >4} Variable value: {:?}, variable type: {:?}",
                variable,
                variable.get_type()
            )
        }
        println!("---");

        let function = program.get_function(self.function_id).unwrap();
        println!("Instruction counter: {}", self.instruction_counter);
        for (no, instruction) in function.instructions.iter().enumerate() {
            if no == self.instruction_counter {
                println!("{no:0>3} ==> {:?}", instruction);
            } else {
                println!("{no:0>3} - {:?}", instruction);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    functions: HashMap<FunctionIdType, Function>,
}

impl Program {
    pub fn new(functions: HashMap<FunctionIdType, Function>) -> Self {
        Self { functions }
    }
    fn get_function(&self, function_id: FunctionIdType) -> Result<&Function, InterpreterError> {
        if let Some(v) = self.functions.get(&function_id) {
            return Ok(v);
        }
        Err(InterpreterError::FunctionDoesNotExist(function_id))
    }
}

#[derive(Debug)]
pub struct Interpreter {
    program: Program,
    callstack: Vec<ExecutionContext>,
    return_value_storage: Option<Value>,
}

impl Interpreter {
    pub fn new(program: Program) -> Self {
        let ctx = ExecutionContext::new(program.get_function(0).unwrap(), 0);
        let mut callstack = Vec::with_capacity(32);
        callstack.push(ctx);
        Interpreter {
            program,
            callstack,
            return_value_storage: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum AfterCycleAction {
    None,
    Goto(usize),
}

impl Interpreter {
    pub fn execute(&mut self) -> Result<(), InterpreterError> {
        'execute_context: while let Some(mut context) = self.callstack.pop() {
            let mut after_cycle: AfterCycleAction = AfterCycleAction::None;
            let function_id = context.function_id;
            let function = self.program.get_function(function_id)?;

            while let Some(instr) = function.instructions.get(context.instruction_counter) {
                if let Some(return_to_var_id) = context.expecting_return_value {
                    let Some(return_value) = self.return_value_storage.take() else {
                        return Err(InterpreterError::NoReturnValue);
                    };
                    context.set_variable(return_to_var_id, return_value)?;

                    self.return_value_storage = None;
                    context.expecting_return_value = None;
                }

                match instr {
                    Instruction::Set(to_var_id, from_var_id) => {
                        let value = context.get_variable(*from_var_id)?;
                        context.set_variable(*to_var_id, value.clone())?
                    }
                    Instruction::SetI(var_id, value) => context.set_variable(*var_id, value.clone())?,
                    Instruction::SetArrayIndex(array_var_id, array_index, new_value_id) => {
                        let array_index = context.get_variable(*array_index)?.to_usize()?;
                        let new_value = context.get_variable(*new_value_id)?.clone();
                        let array = context.get_variable_mut(*array_var_id)?;
                        let Value::Array(values) = array else {
                            return Err(InterpreterError::ArrayOperationOnNonArrayValue(array.get_type()));
                        };
                        values.set_index(array_index, new_value)?;
                    }
                    Instruction::SetArrayIndexI(array_var_id, array_index, value) => {
                        let array_index = context.get_variable(*array_index)?.to_usize()?;
                        let array = context.get_variable_mut(*array_var_id)?;

                        let Value::Array(values) = array else {
                            return Err(InterpreterError::ArrayOperationOnNonArrayValue(array.get_type()));
                        };
                        values.set_index(array_index, value.clone())?;
                    }
                    Instruction::GetArrayIndex(array_var_id, store_var_id, index_var_id) => {
                        let array_index = context.get_variable(*index_var_id)?.to_usize()?;
                        let array = context.get_variable(*array_var_id)?;

                        let Value::Array(values) = array else {
                            return Err(InterpreterError::ArrayOperationOnNonArrayValue(array.get_type()));
                        };

                        let val = values.get_index(array_index)?;
                        context.set_variable(*store_var_id, val)?;
                    }
                    Instruction::Add(lvalue_id, rvalue_id) => {
                        let (lvalue, rvalue) = context.get_variable_pair(*lvalue_id, *rvalue_id)?;

                        let new_value = op_add(lvalue.clone(), rvalue.clone());
                        context.set_variable(*lvalue_id, new_value)?;
                    }
                    Instruction::Sub(lvalue_id, rvalue_id) => {
                        let (lvalue, rvalue) = context.get_variable_pair(*lvalue_id, *rvalue_id)?;
                        let new_value = op_sub(lvalue.clone(), rvalue.clone());
                        context.set_variable(*lvalue_id, new_value)?;
                    }

                    Instruction::Rem(lvalue_id, rvalue_id) => {
                        let (lvalue, rvalue) = context.get_variable_pair(*lvalue_id, *rvalue_id)?;
                        let new_value = op_rem(lvalue.clone(), rvalue.clone());
                        context.set_variable(*lvalue_id, new_value)?;
                    }
                    Instruction::AddI(lvalue_id, rvalue) => {
                        let lvalue = context.get_variable(*lvalue_id)?;
                        let new_value = op_add(lvalue.clone(), rvalue.clone());
                        context.set_variable(*lvalue_id, new_value)?;
                    }
                    Instruction::SubI(lvalue_id, rvalue) => {
                        let lvalue = context.get_variable(*lvalue_id)?;
                        let new_value = op_sub(lvalue.clone(), rvalue.clone());

                        context.set_variable(*lvalue_id, new_value)?;
                    }

                    Instruction::LessThan(bool_var_id, lvalue_id, rvalue_id) => {
                        let (lvalue, rvalue) = context.get_variable_pair(*lvalue_id, *rvalue_id)?;
                        let result = op_less_than(lvalue.clone(), rvalue.clone());
                        context.set_variable(*bool_var_id, result)?;
                    }
                    Instruction::LessThanI(bool_var_id, lvalue_id, rvalue) => {
                        let lvalue = context.get_variable(*lvalue_id)?;
                        let result = op_less_than(lvalue.clone(), rvalue.clone());
                        context.set_variable(*bool_var_id, result)?;
                    }
                    Instruction::Equals(bool_var_id, lvalue_id, rvalue_id) => {
                        let (lvalue, rvalue) = context.get_variable_pair(*lvalue_id, *rvalue_id)?;

                        let result = op_equals(lvalue.clone(), rvalue.clone());
                        context.set_variable(*bool_var_id, result)?;
                    }
                    Instruction::EqualsI(bool_var_id, lvalue_id, rvalue) => {
                        let lvalue = context.get_variable(*lvalue_id)?;
                        let result = op_equals(lvalue.clone(), rvalue.clone());
                        context.set_variable(*bool_var_id, result)?;
                    }
                    Instruction::NotEquals(bool_var_id, lvalue_id, rvalue_id) => {
                        let (lvalue, rvalue) = context.get_variable_pair(*lvalue_id, *rvalue_id)?;

                        let result = op_not_equals(lvalue.clone(), rvalue.clone());
                        context.set_variable(*bool_var_id, result)?;
                    }
                    Instruction::NotEqualsI(bool_var_id, lvalue_id, rvalue) => {
                        let lvalue = context.get_variable(*lvalue_id)?;
                        let result = op_not_equals(lvalue.clone(), rvalue.clone());
                        context.set_variable(*bool_var_id, result)?;
                    }

                    Instruction::Goto(instruction_number) => {
                        after_cycle = AfterCycleAction::Goto(*instruction_number);
                    }
                    Instruction::GotoIfTrue(instruction_number, bool_var_id) => match context.get_variable(*bool_var_id)?.get_bool() {
                        Some(true) => after_cycle = AfterCycleAction::Goto(*instruction_number),
                        Some(false) => {}
                        None => return Err(InterpreterError::GotoNonBoolean),
                    },

                    Instruction::PushFunctionParameter(var_id) => {
                        let value = { context.get_variable(*var_id)? };
                        context.function_parameter_stack.push(value.clone());
                    }

                    Instruction::CallVoidFunction(function_id) => {
                        context.instruction_counter += 1;

                        // push back current context
                        self.callstack.push(context);
                        // then add new context
                        let function = self.program.get_function(*function_id)?;
                        if function.return_type.is_some() {
                            return Err(InterpreterError::VoidCallToNonVoidFunction(*function_id));
                        }
                        let new_context = ExecutionContext::new(function, *function_id);
                        self.callstack.push(new_context);

                        continue 'execute_context;
                    }
                    Instruction::CallFunction(function_id, return_value_destination_id) => {
                        let function_id = *function_id;
                        context.instruction_counter += 1;
                        context.expecting_return_value = Some(*return_value_destination_id);

                        let function = self.program.get_function(function_id)?;
                        if function.return_type.is_none() {
                            return Err(InterpreterError::ExpectingReturnCallToVoidFunction(function_id));
                        }
                        let mut new_context = ExecutionContext::new(function, function_id);
                        for (param_id, param_type) in function.parameters.iter().enumerate().rev() {
                            let param_id = param_id as u16;
                            let Some(param_value) = context.function_parameter_stack.pop() else {
                                return Err(InterpreterError::FunctionCallParameterStackEmptyPop(function_id));
                            };
                            if param_value.get_type() != *param_type {
                                return Err(InterpreterError::FunctionCallParametersInvalid(function_id, false));
                            }
                            new_context.set_variable(param_id, param_value)?;
                        }
                        // push back current context
                        self.callstack.push(context);
                        // then add new context
                        self.callstack.push(new_context);

                        continue 'execute_context;
                    }

                    Instruction::Return(var_id_to_return) => {
                        let value = context.get_variable(*var_id_to_return)?;
                        self.return_value_storage = Some(value.clone());
                        continue 'execute_context;
                    }
                    Instruction::CallNativeVoidFunction(native_function_id) => {
                        // println for now
                        #[allow(clippy::match_single_binding)]
                        match native_function_id {
                            _ => {
                                let Some(value) = context.function_parameter_stack.pop() else {
                                    return Err(InterpreterError::FunctionCallParametersInvalid(*native_function_id, true));
                                };
                                assert!(context.function_parameter_stack.is_empty());
                                println!("Println {:?}", value);
                            }
                        }
                    }
                    Instruction::CallNativeVoidMethod(var_id, method_id) => {
                        #[allow(clippy::match_single_binding)]
                        match method_id {
                            _ => {
                                // array.push() just for now.

                                let Some(push_value) = context.function_parameter_stack.pop() else {
                                    return Err(InterpreterError::FunctionCallParameterStackEmptyPop(*method_id));
                                };
                                let array = context.get_variable_mut(*var_id)?;
                                let Value::Array(values) = array else {
                                    return Err(InterpreterError::ArrayOperationOnNonArrayValue(array.get_type()));
                                };
                                values.push(push_value)?;
                            }
                        }
                    }
                    Instruction::CallNativeMethod(var_id, value_return_store, method_id) => {
                        #[allow(clippy::match_single_binding)]
                        match method_id {
                            _ => {
                                // array.len() just for now.
                                let array = context.get_variable_mut(*var_id)?;
                                let Value::Array(values) = array else {
                                    return Err(InterpreterError::ArrayOperationOnNonArrayValue(array.get_type()));
                                };
                                let len = values.len();
                                context.set_variable(*value_return_store, Value::U64(len as u64))?;
                            }
                        }
                    }
                    // Unimplemented
                    Instruction::Mul(_, _) => unimplemented!(),
                    Instruction::Div(_, _) => unimplemented!(),
                    Instruction::MulI(_, _) => unimplemented!(),
                    Instruction::DivI(_, _) => unimplemented!(),
                    Instruction::RemI(_, _) => unimplemented!(),
                    Instruction::GreaterThanI(_, _, _) => unimplemented!(),
                    Instruction::LessThanOrEqual(_, _, _) => unimplemented!(),
                    Instruction::LessThanOrEqualI(_, _, _) => unimplemented!(),
                    Instruction::GreaterThanOrEqual(_, _, _) => unimplemented!(),
                    Instruction::GreaterThanOrEqualI(_, _, _) => unimplemented!(),
                    Instruction::Or(_, _) => unimplemented!(),
                    Instruction::And(_, _) => unimplemented!(),
                    Instruction::Xor(_, _) => unimplemented!(),
                    Instruction::Not(_) => unimplemented!(),
                    Instruction::SetArrayIIndex(_, _, _) => unimplemented!(),
                    Instruction::GetArrayIndexI(_, _, _) => unimplemented!(),
                    Instruction::GreaterThan(_, _, _) => unimplemented!(),
                }

                // context.print_state(&self.program);

                match after_cycle {
                    AfterCycleAction::None => {
                        context.instruction_counter += 1;
                    }
                    AfterCycleAction::Goto(goto_location) => {
                        context.instruction_counter = goto_location;
                        after_cycle = AfterCycleAction::None;
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::interpreter::*;
    fn run_function(function: Function) {
        let mut functions = HashMap::new();
        functions.insert(0, function);
        let program = Program::new(functions);
        let mut interpreter = Interpreter::new(program);

        interpreter.execute().unwrap();
    }

    #[test]
    fn test_function_call() {
        let mut main = Function::new(&[], None);

        main.set_instructions(vec![
            Instruction::CallFunction(1, 1),
            Instruction::PushFunctionParameter(1),
            Instruction::CallNativeVoidFunction(0),
        ]);
        main.register_variables(&[VariableType::U32, VariableType::Bool]);
        let mut other = Function::new(&[], Some(VariableType::Bool));
        other.register_variables(&[VariableType::Bool]);
        other.set_instructions(vec![
            Instruction::SetI(0, Value::Bool(true)),
            Instruction::PushFunctionParameter(0),
            Instruction::CallNativeVoidFunction(0),
            Instruction::Return(0),
        ]);
        let mut functions = HashMap::new();
        functions.insert(0, main);
        functions.insert(1, other);
        let program = Program::new(functions);
        let mut interpreter = Interpreter::new(program);

        interpreter.execute().unwrap();
    }
    #[test]
    fn test_basic_loop() {
        let instructions = vec![
            Instruction::SetI(1, Value::U64(0)),
            Instruction::AddI(0, Value::U64(32)),
            Instruction::PushFunctionParameter(0),
            Instruction::CallNativeVoidFunction(0),
            Instruction::AddI(1, Value::U64(1)),
            Instruction::LessThanI(2, 1, Value::U64(10)),
            Instruction::GotoIfTrue(1, 2),
        ];
        let mut func = Function::new(&[], None);
        func.register_variables(&[VariableType::U64, VariableType::U64, VariableType::Bool]);
        func.set_instructions(instructions);
        run_function(func);
    }
    #[test]
    fn test_basic_variable_operations() {
        let instructions = vec![
            Instruction::SetI(0, Value::U64(32)),
            Instruction::PushFunctionParameter(0),
            Instruction::CallNativeVoidFunction(0),
        ];
        let mut func = Function::new(&[], None);
        func.register_variables(&[VariableType::U64, VariableType::U64, VariableType::Bool]);
        func.set_instructions(instructions);

        run_function(func);
    }
}
