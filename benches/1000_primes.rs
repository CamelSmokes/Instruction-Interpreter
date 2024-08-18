use std::collections::HashMap;

use criterion::{criterion_group, criterion_main, Criterion};
use instructions::Instruction;
use interpreter::{Function, Interpreter, Program};
use new_interp::*;
use value::{ArrayValue, Value, VariableType};

pub fn benchmark_primes(c: &mut Criterion) {
    c.bench_function("Find 1000 primes", |b| {
        b.iter(|| {
            let mut main = Function::new(&[], None);
            main.register_variables(&[
                VariableType::U64,                                // Check value 0
                VariableType::Array(Box::new(VariableType::U64)), // primes list 1
                VariableType::U64,                                // prime count 2
                VariableType::Bool,                               // prime cond  3
            ]);
            main.set_instructions(vec![
                Instruction::SetI(0, Value::U64(3)),
                Instruction::SetI(1, Value::Array(ArrayValue::U64Array(vec![2]))),
                // BRANCH: TEST_PRIME
                Instruction::PushFunctionParameterStack(0),
                Instruction::PushFunctionParameterStack(1),
                Instruction::CallFunction(1, 3),
                Instruction::GotoIfTrue(10, 3), // TO: PRIME_FOUND
                // BRANCH: BACK FROM PRIME_FOUND
                Instruction::AddI(0, Value::U64(2)),
                Instruction::LessThanI(3, 2, Value::U64(1000)),
                Instruction::GotoIfTrue(2, 3), // TO: TEST_PRIME
                Instruction::Goto(14),         // TO: PRINT_PRIMES
                // BRANCH: PRIME_FOUND
                Instruction::PushFunctionParameterStack(0),
                Instruction::CallNativeVoidMethod(1, 0),
                Instruction::AddI(2, Value::U64(1)),
                Instruction::Goto(6),
                // BRANCH: PRINT_PRIMES
                Instruction::PushFunctionParameterStack(1),
                Instruction::CallNativeVoidFunction(0),
            ]);

            let mut prime_finder = Function::new(
                &[VariableType::U64, VariableType::Array(Box::new(VariableType::U64))],
                Some(VariableType::Bool),
            );
            prime_finder.register_variables(&[
                VariableType::U64,  // array index  2
                VariableType::U64,  // array length 3
                VariableType::U64,  // array value  4
                VariableType::Bool, // if_condition 5
                VariableType::Bool, // return value 6
                VariableType::U64,  // tmp value    7
            ]);
            prime_finder.set_instructions(vec![
                Instruction::CallNativeMethod(1, 3, 1),    // store array length to var 2
                Instruction::GetArrayIndex(1, 4, 2),       // store array value at index [var 2] to var 4 LOOP
                Instruction::Set(7, 0),                    // store parameter var 0 to tmp value var 7
                Instruction::Rem(7, 4),                    // Take tmp value var 7 mod var 4
                Instruction::EqualsI(5, 7, Value::U64(0)), // Test if the modulus in var 7 is equal to 0
                // IF true, return false
                Instruction::GotoIfTrue(11, 5),
                // loop condition
                Instruction::AddI(2, Value::U64(1)), // increment loop counter
                Instruction::LessThan(5, 2, 3),      // If loop counter is less than array length
                Instruction::GotoIfTrue(1, 5),       // Then goto return true
                // return conditions
                Instruction::SetI(6, Value::Bool(true)), // fallthrough return true
                Instruction::Return(6),
                Instruction::SetI(6, Value::Bool(false)), //return false
                Instruction::Return(6),
            ]);
            let mut functions = HashMap::new();
            functions.insert(0, main);
            functions.insert(1, prime_finder);
            let program = Program::new(functions);
            let mut interpreter = Interpreter::new(program);

            interpreter.execute().unwrap();
        });
    });
}

criterion_group!(benches, benchmark_primes);
criterion_main!(benches);
