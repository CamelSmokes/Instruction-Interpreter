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
                VariableType::Bool,                               // conditional 3
            ]);
            main.set_instructions(vec![
                Instruction::SetI(1, Value::Array(ArrayValue::U64Array(vec![2]))), // Set prime number list (var 1) to [2]
                Instruction::SetI(0, Value::U64(3)),                               // Set prime number check (var 0) to 3
                // LABEL: TEST_PRIME
                Instruction::PushFunctionParameter(0), // push prime check value var 0
                Instruction::PushFunctionParameter(1), // push primes list       var 1
                Instruction::CallFunction(1, 3),       // call prime check function with pushed parameters; store result to var 3
                Instruction::GotoIfTrue(10, 3),        // goto PRIME_FOUND if var 3 true
                // LABEL: BACK FROM PRIME_FOUND
                Instruction::AddI(0, Value::U64(2)),            // Increment check value (var 0) by 2
                Instruction::LessThanI(3, 2, Value::U64(1000)), // Check if prime count (var 2) is less than 1000 store result to var 3
                Instruction::GotoIfTrue(2, 3),                  // goto TEST_PRIME if var 3 is true
                Instruction::Goto(14),                          // otherwise, goto PRINT_PRIMES
                // BRANCH: PRIME_FOUND
                Instruction::PushFunctionParameter(0),   // Push found prime (var 0)
                Instruction::CallNativeVoidMethod(1, 0), // Call `array.push()` on var 1
                Instruction::AddI(2, Value::U64(1)),     // Increment prime counter (var 2) by 1
                Instruction::Goto(6),
                // BRANCH: PRINT_PRIMES
                Instruction::PushFunctionParameter(1),  // Push prime list (var 1)
                Instruction::CallNativeVoidFunction(0), // Print prime list
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
                Instruction::CallNativeMethod(1, 3, 1), // store array length to var 2
                // LABEL: FOREACH_LOOP
                Instruction::GetArrayIndex(1, 4, 2), // store array value at index [var 2] to var 4 LOOP
                Instruction::Set(7, 0),              // store parameter var 0 to tmp value var 7
                Instruction::Rem(7, 4),              // Take tmp value var 7 mod var 4
                Instruction::EqualsI(5, 7, Value::U64(0)), // Test if the modulus in var 7 is equal to 0
                Instruction::GotoIfTrue(11, 5),      // if true GOTO RETURN_FALSE
                // loop condition
                Instruction::AddI(2, Value::U64(1)), // increment loop counter
                Instruction::LessThan(5, 2, 3),      // If loop counter is less than array length
                Instruction::GotoIfTrue(1, 5),       // Then goto FOREACH_LOOP
                // FALLTHROUGH:
                // BRANCH: RETURN_TRUE
                Instruction::SetI(6, Value::Bool(true)), // fallthrough return true
                Instruction::Return(6),
                // BRANCH: RETURN_FALSE
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
