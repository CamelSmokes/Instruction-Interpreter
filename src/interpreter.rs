use std::{borrow::Cow, collections::HashMap};

use crate::{
    op_rem,
    operations::{op_add, op_equals, op_less_than, op_not_equals, op_sub},
    value::{FunctionIdType, Value, VariableIdType, VariableType},
    Instruction,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterpreterError {
    ReferencedVariableDoesNotExist(VariableIdType),
    AttemptAssignedDifferentTypes(VariableType, VariableType),
    FunctionDoesNotExist(FunctionIdType),
    VoidCallToNonVoidFunction(FunctionIdType),
    ExpectingReturnCallToVoidFunction(FunctionIdType),
}

#[derive(Debug, Clone, Default)]
pub struct Function {
    instructions: Vec<Instruction>,
    variables: HashMap<VariableIdType, VariableType>,
    parameters: Vec<VariableType>,
    return_type: Option<VariableType>,
    variable_count: u16,
}

impl Function {
    pub fn new(parameters: &[VariableType], return_type: Option<VariableType>) -> Self {
        let mut func = Function {
            instructions: Vec::new(),
            variables: HashMap::new(),
            parameters: parameters.to_vec(),
            return_type,
            variable_count: 0,
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
        self.variables.insert(self.variable_count, var_type);
        let var_id = self.variable_count;
        self.variable_count += 1;
        var_id
    }
    pub fn register_variables(&mut self, var_types: &[VariableType]) {
        for var_type in var_types {
            self.register_variable(var_type.clone());
        }
    }

    pub fn set_variable_type(&mut self, var_id: VariableIdType, var_type: VariableType) {
        assert!(!self.variables.contains_key(&var_id));
        self.variables.insert(var_id, var_type);
    }
    // pub fn set_all_variable_types(&mut self, types: HashMap<VariableIdType, VariableType>) {
    //     self.variables = types;
    // }
}

#[derive(Debug, Clone)]
struct ExecutionContext {
    variables: HashMap<u16, Value>,
    function_parameter_stack: Vec<Value>,
    function_id: FunctionIdType,
    instruction_counter: usize,
    expecting_return_value: Option<VariableIdType>,
}

impl ExecutionContext {
    fn new(func: &Function, function_id: FunctionIdType) -> Self {
        let mut variables: HashMap<u16, Value> = HashMap::new();
        for (a, b) in func.variables.iter() {
            let default_value = match b {
                VariableType::U8 => Value::U8(0),
                VariableType::U16 => Value::U16(0),
                VariableType::U32 => Value::U32(0),
                VariableType::U64 => Value::U64(0),
                VariableType::Bool => Value::Bool(false),
                VariableType::String => Value::String(String::new()),
                VariableType::Array(arr_type) => Value::Array(*arr_type.to_owned(), Vec::new()),
            };
            variables.insert(*a, default_value);
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
        if let Some(v) = self.variables.get(&var_id) {
            Ok(v)
        } else {
            Err(InterpreterError::ReferencedVariableDoesNotExist(var_id))
        }
    }
    fn get_variable_mut(&mut self, var_id: VariableIdType) -> Result<&mut Value, InterpreterError> {
        if let Some(v) = self.variables.get_mut(&var_id) {
            Ok(v)
        } else {
            Err(InterpreterError::ReferencedVariableDoesNotExist(var_id))
        }
    }
    fn set_variable(
        &mut self,
        var_id: VariableIdType,
        value: Value,
    ) -> Result<(), InterpreterError> {
        let Some(current_value) = self.variables.get(&var_id) else {
            return Err(InterpreterError::ReferencedVariableDoesNotExist(var_id));
        };

        if current_value.get_type() != value.get_type() {
            return Err(InterpreterError::AttemptAssignedDifferentTypes(
                current_value.get_type(),
                value.get_type(),
            ));
        }

        self.variables.insert(var_id, value);
        Ok(())
    }

    fn print_state(&self, program: &Program) {
        let mut variable_list = vec![Value::U8(255); self.variables.len()];
        for (id, variable) in self.variables.iter() {
            variable_list[*id as usize] = variable.clone();
        }
        println!("---");
        for (id, variable) in variable_list.into_iter().enumerate() {
            println!(
                "Variable ID {id: >4} Variable value: {:?}, variable type: {:?}",
                variable,
                variable.get_type()
            )
        }
        println!("---");
        let function = program.functions.get(&self.function_id).unwrap();
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
    entry_point_id: FunctionIdType,
}

impl Program {
    pub fn new(
        functions: HashMap<FunctionIdType, Function>,
        entry_function: FunctionIdType,
    ) -> Self {
        Self {
            functions,
            entry_point_id: entry_function,
        }
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
        let ctx = ExecutionContext::new(
            program.functions.get(&program.entry_point_id).unwrap(),
            program.entry_point_id,
        );
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
            // println!("New context {:?}", context);
            while let Some(instr) = self
                .program
                .get_function(function_id)
                .unwrap()
                .instructions
                .get(context.instruction_counter)
            {
                if let Some(return_to_var_id) = context.expecting_return_value {
                    let return_value = self.return_value_storage.take().unwrap();
                    context.set_variable(return_to_var_id, return_value)?;

                    self.return_value_storage = None;
                    context.expecting_return_value = None;
                }

                match instr {
                    Instruction::Set(to_var_id, from_var_id) => {
                        let value = context.get_variable(*from_var_id)?;
                        context.set_variable(*to_var_id, value.clone())?
                    }
                    Instruction::SetI(var_id, value) => {
                        context.set_variable(*var_id, value.clone())?
                    }
                    Instruction::SetArrayIndex(array_var_id, array_index, new_value_id) => {
                        let array_index = context.get_variable(*array_index)?.to_usize().unwrap();
                        let new_value = context.get_variable(*new_value_id)?.clone();
                        let array = context.get_variable_mut(*array_var_id)?;

                        if let Value::Array(array_type, values) = array {
                            if new_value.get_type() != *array_type {
                                todo!();
                            }
                            if let Some(ind) = values.get_mut(array_index) {
                                *ind = new_value;
                            } else {
                                todo!()
                            }
                        } else {
                            todo!();
                        }
                    }
                    Instruction::SetArrayIndexI(array_var_id, array_index, value) => {
                        let array_index = context.get_variable(*array_index)?.to_usize().unwrap();
                        let array = context.get_variable_mut(*array_var_id)?;

                        if let Value::Array(array_type, values) = array {
                            if value.get_type() != *array_type {
                                todo!();
                            }
                            if let Some(ind) = values.get_mut(array_index) {
                                *ind = value.clone();
                            } else {
                                todo!()
                            }
                        } else {
                            todo!();
                        }
                    }
                    Instruction::SetArrayIIndex(_, _, _) => todo!(),
                    // Instruction::SetArrayIIndexI(_, _, _) => todo!(),
                    Instruction::GetArrayIndex(array_var_id, store_var_id, index_var_id) => {
                        let array_index = context.get_variable(*index_var_id)?.to_usize().unwrap();
                        let store_type = context.get_variable(*store_var_id)?.get_type();
                        let array = context.get_variable(*array_var_id)?;
                        if let Value::Array(array_type, values) = array {
                            if store_type != *array_type {
                                todo!();
                            }
                            if let Some(val) = values.get(array_index) {
                                context.set_variable(*store_var_id, val.clone())?;
                            } else {
                                todo!();
                            }
                        } else {
                            todo!();
                        }
                    }
                    Instruction::GetArrayIndexI(_, _, _) => todo!(),
                    Instruction::Add(lvalue_id, rvalue_id) => {
                        let lvalue = context.get_variable(*lvalue_id)?;
                        let rvalue = context.get_variable(*rvalue_id)?;
                        let new_value = op_add(lvalue.clone(), rvalue.clone());
                        context.variables.insert(*lvalue_id, new_value);
                    }
                    Instruction::Sub(lvalue_id, rvalue_id) => {
                        let lvalue = context.get_variable(*lvalue_id)?;
                        let rvalue = context.get_variable(*rvalue_id)?;
                        let new_value = op_sub(lvalue.clone(), rvalue.clone());
                        context.variables.insert(*lvalue_id, new_value);
                    }
                    Instruction::Mul(_, _) => todo!(),
                    Instruction::Div(_, _) => todo!(),
                    Instruction::Rem(lvalue_id, rvalue_id) => {
                        let lvalue = context.get_variable(*lvalue_id)?;
                        let rvalue = context.get_variable(*rvalue_id)?;
                        let new_value = op_rem(lvalue.clone(), rvalue.clone());
                        context.variables.insert(*lvalue_id, new_value);
                    }
                    Instruction::AddI(lvalue_id, rvalue) => {
                        let lvalue = context.get_variable(*lvalue_id)?;
                        let new_value = op_add(lvalue.clone(), rvalue.clone());
                        context.variables.insert(*lvalue_id, new_value);
                    }
                    Instruction::SubI(lvalue_id, rvalue) => {
                        let lvalue = context.get_variable(*lvalue_id)?;
                        let new_value = op_sub(lvalue.clone(), rvalue.clone());
                        context.variables.insert(*lvalue_id, new_value);
                    }
                    Instruction::MulI(_, _) => todo!(),
                    Instruction::DivI(_, _) => todo!(),
                    Instruction::RemI(_, _) => todo!(),
                    Instruction::Goto(instruction_number) => {
                        after_cycle = AfterCycleAction::Goto(*instruction_number);
                    }
                    Instruction::GotoIfTrue(instruction_number, bool_var_id) => {
                        let bool_value = context.get_variable(*bool_var_id)?;
                        if let Some(bool) = bool_value.get_bool() {
                            if bool {
                                after_cycle = AfterCycleAction::Goto(*instruction_number);
                            }
                        } else {
                            todo!();
                        }
                    }

                    Instruction::PushFunctionParameterStack(var_id) => {
                        let value = { context.get_variable(*var_id)? };
                        context.function_parameter_stack.push(value.clone());
                    }

                    Instruction::LessThan(bool_var_id, lvalue_id, rvalue_id) => {
                        let lvalue = context.get_variable(*lvalue_id)?;
                        let rvalue = context.get_variable(*rvalue_id)?;
                        let result = op_less_than(lvalue.clone(), rvalue.clone());
                        context.set_variable(*bool_var_id, result)?;
                    }
                    Instruction::LessThanI(bool_var_id, lvalue_id, rvalue) => {
                        let lvalue = context.get_variable(*lvalue_id)?;
                        let result = op_less_than(lvalue.clone(), rvalue.clone());
                        context.set_variable(*bool_var_id, result)?;
                    }
                    Instruction::GreaterThan(_, _, _) => todo!(),
                    Instruction::Equals(bool_var_id, lvalue_id, rvalue_id) => {
                        let lvalue = context.get_variable(*lvalue_id)?;
                        let rvalue = context.get_variable(*rvalue_id)?;
                        let result = op_equals(lvalue.clone(), rvalue.clone());
                        context.set_variable(*bool_var_id, result)?;
                    }
                    Instruction::EqualsI(bool_var_id, lvalue_id, rvalue) => {
                        let lvalue = context.get_variable(*lvalue_id)?;
                        let result = op_equals(lvalue.clone(), rvalue.clone());
                        context.set_variable(*bool_var_id, result)?;
                    }
                    Instruction::NotEquals(bool_var_id, lvalue_id, rvalue_id) => {
                        let lvalue = context.get_variable(*lvalue_id)?;
                        let rvalue = context.get_variable(*rvalue_id)?;
                        let result = op_not_equals(lvalue.clone(), rvalue.clone());
                        context.set_variable(*bool_var_id, result)?;
                    }
                    Instruction::NotEqualsI(bool_var_id, lvalue_id, rvalue) => {
                        let lvalue = context.get_variable(*lvalue_id)?;
                        let result = op_not_equals(lvalue.clone(), rvalue.clone());
                        context.set_variable(*bool_var_id, result)?;
                    }
                    Instruction::Or(_, _) => todo!(),
                    Instruction::And(_, _) => todo!(),
                    Instruction::Xor(_, _) => todo!(),
                    Instruction::Not(_) => todo!(),
                    Instruction::LessThanOrEqual(_, _, _) => todo!(),
                    Instruction::LessThanOrEqualI(_, _, _) => todo!(),
                    Instruction::GreaterThanOrEqual(_, _, _) => todo!(),
                    Instruction::GreaterThanOrEqualI(_, _, _) => todo!(),

                    Instruction::CallVoidFunction(function_id) => {
                        context.instruction_counter += 1;

                        // push back current context
                        self.callstack.push(context);
                        // then add new context
                        // TODO: Handle function parameters
                        let function = self.program.get_function(*function_id)?;
                        if function.return_type.is_some() {
                            return Err(InterpreterError::VoidCallToNonVoidFunction(*function_id));
                        }
                        let new_context = ExecutionContext::new(function, *function_id);
                        self.callstack.push(new_context);

                        continue 'execute_context;
                    }
                    Instruction::CallFunction(function_id, return_value_destination_id) => {
                        context.instruction_counter += 1;
                        context.expecting_return_value = Some(*return_value_destination_id);
                        // TODO: Handle function parameters

                        let function = self.program.get_function(*function_id)?;
                        if function.return_type.is_none() {
                            return Err(InterpreterError::ExpectingReturnCallToVoidFunction(
                                *function_id,
                            ));
                        }
                        let mut new_context = ExecutionContext::new(function, *function_id);
                        for (param_id, param_type) in function.parameters.iter().enumerate().rev() {
                            let param_id = param_id as u16;
                            let param_value = context.function_parameter_stack.pop().unwrap();
                            if param_value.get_type() != *param_type {
                                todo!();
                            }
                            new_context.set_variable(param_id, param_value)?;
                        }
                        // push back current context
                        self.callstack.push(context);
                        // then add new context
                        self.callstack.push(new_context);

                        continue 'execute_context;
                    }
                    Instruction::CallVoidNativeFunction(native_function_id) => {
                        // println for now
                        match native_function_id {
                            0 => {
                                let value = context.function_parameter_stack.pop().unwrap();
                                assert!(context.function_parameter_stack.is_empty());
                                println!("Println {:?}", value);
                            }
                            _ => unimplemented!(),
                        }
                    }
                    Instruction::Return(var_id_to_return) => {
                        let value = context.get_variable(*var_id_to_return)?;
                        self.return_value_storage = Some(value.clone());
                        continue 'execute_context;
                    }
                    Instruction::CallNativeVoidMethod(var_id, method_id) => {
                        match method_id {
                            0 => {
                                // array.push() just for now.

                                let push_value = context.function_parameter_stack.pop().unwrap();
                                let array = context.get_variable_mut(*var_id)?;
                                if let Value::Array(array_type, values) = array {
                                    if push_value.get_type() != *array_type {
                                        todo!();
                                    }

                                    values.push(push_value);
                                } else {
                                    todo!();
                                }
                            }
                            _ => todo!(),
                        }
                    }
                    Instruction::CallNativeMethod(var_id, value_return_store, method_id) => {
                        match method_id {
                            1 => {
                                // array.len() just for now.

                                let array = context.get_variable_mut(*var_id)?;
                                if let Value::Array(_, values) = array {
                                    let len = values.len();
                                    context.expecting_return_value = Some(*value_return_store);
                                    self.return_value_storage = Some(Value::U64(len as u64));
                                } else {
                                    todo!();
                                }
                            }
                            _ => todo!(),
                        }
                    }
                    Instruction::GreaterThanI(_, _, _) => todo!(),
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

pub fn create_var_map(var_list: &[VariableType]) -> HashMap<VariableIdType, VariableType> {
    let mut map = HashMap::new();
    for (id, var_type) in var_list.iter().cloned().enumerate() {
        let id = id as VariableIdType;
        map.insert(id, var_type);
    }
    map
}
#[cfg(test)]
mod test {
    use crate::interpreter::*;
    fn run_function(function: Function) {
        let mut functions = HashMap::new();
        functions.insert(0, function);
        let program = Program::new(functions, 0);
        let mut interpreter = Interpreter::new(program);

        interpreter.execute().unwrap();
    }

    #[test]
    fn test_function_call() {
        let mut main = Function::new(&[], None);

        main.set_instructions(vec![
            Instruction::CallFunction(1, 1),
            Instruction::PushFunctionParameterStack(1),
            Instruction::CallVoidNativeFunction(0),
        ]);
        main.register_variables(&[VariableType::U32, VariableType::Bool]);
        let mut other = Function::new(&[], Some(VariableType::Bool));
        other.register_variables(&[VariableType::Bool]);
        other.set_instructions(vec![
            Instruction::SetI(0, Value::Bool(true)),
            Instruction::PushFunctionParameterStack(0),
            Instruction::CallVoidNativeFunction(0),
            Instruction::Return(0),
        ]);
        let mut functions = HashMap::new();
        functions.insert(0, main);
        functions.insert(1, other);
        let program = Program::new(functions, 0);
        let mut interpreter = Interpreter::new(program);

        interpreter.execute().unwrap();
    }
    #[test]
    fn test_basic_loop() {
        let instructions = vec![
            Instruction::SetI(1, Value::U64(0)),
            Instruction::AddI(0, Value::U64(32)),
            Instruction::PushFunctionParameterStack(0),
            Instruction::CallVoidNativeFunction(0),
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
            Instruction::PushFunctionParameterStack(0),
            Instruction::CallVoidNativeFunction(0),
        ];
        let mut func = Function::new(&[], None);
        func.register_variables(&[VariableType::U64, VariableType::U64, VariableType::Bool]);
        func.set_instructions(instructions);

        run_function(func);
    }
}
