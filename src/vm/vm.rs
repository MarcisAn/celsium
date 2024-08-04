use super::{ format_for_print::format_for_print, math_operators::*, StackValue };
use crate::{ bytecode::{BINOP, OPTCODE}, BuiltinTypes, CelsiumProgram };
use num::BigInt;
use std::{ collections::HashMap, io::{ self, BufRead, Write }, str::FromStr, vec };

pub struct VM {
    pub(crate) registers: Vec<Option<StackValue>>,
    pub(crate) variables: HashMap<usize, Variable>,
    pub(crate) testing_stack: Vec<StackValue>,
}
#[derive(Clone, Debug)]
pub struct Variable {
    pub(crate) id: usize,
    pub(crate) value: StackValue,
}

impl StackValue {}

impl VM {
    pub fn new(register_count: usize) -> VM {
        let mut vec = vec![None; register_count];
        VM {
            registers: vec,
            variables: HashMap::new(),
            testing_stack: vec![],
        }
    }
    pub fn push(&mut self, data_type: &BuiltinTypes, data: &String, register: usize) {
        let value = match data_type {
            BuiltinTypes::MagicInt =>
                StackValue::BIGINT {
                    value: BigInt::from_str(&data).unwrap(),
                },
            BuiltinTypes::Bool => {
                if data == "1" {
                    StackValue::Bool { value: true }
                } else if data == "0" {
                    StackValue::Bool { value: false }
                } else {
                    panic!("incorrect bool value")
                }
            }
            BuiltinTypes::String =>
                StackValue::String {
                    value: data.to_string(),
                },
            BuiltinTypes::Object { fields: _ } => panic!("object should not appear in bytecode"),
            BuiltinTypes::Float => StackValue::Float { value: data.parse().unwrap() },
            BuiltinTypes::Array { element_type } => todo!(),
        };
        self.registers[register] = Some(value);
    }
    pub fn push_stackvalue(&mut self, stackvalue: StackValue, register: usize) {
        self.registers[register] = Some(stackvalue);
    }
    pub fn push_to_testing_stack(&mut self, register: usize) {
        self.testing_stack.push(self.registers[register].clone().unwrap());
    }
    pub fn get_register(&mut self, register: usize) -> StackValue {
        return self.registers[register].clone().unwrap()
    }
    pub fn aritmethics(&mut self, binop: BINOP, a_reg: usize, b_reg: usize, target: usize) {
        //println!("action {}", action);
        let b = self.registers[a_reg].clone().unwrap();
        let a = self.registers[b_reg].clone().unwrap();
        
        let result: StackValue = match binop {
            BINOP::Add => add(a, b),
            BINOP::Subtract => subtract(a, b),
            BINOP::Multiply => multiply(a, b),
            BINOP::Divide => divide(a, b),
            BINOP::Remainder => remainder(a, b),
            BINOP::LessThan => less_than(a, b),
            BINOP::LargerThan => larger_than(a, b),
            BINOP::LessOrEq => less_or_eq(a, b),
            BINOP::LargerOrEq => larger_or_eq(a, b),
            BINOP::NotEq => not_eq(a, b),
            BINOP::Eq => eq(a, b),
            BINOP::And => and(a, b),
            BINOP::Or => or(a, b),
            BINOP::Xor => xor(a, b),
        };

        self.registers[target] = Some(result);
    }
    pub fn format_for_print(&mut self, newline: bool, register: usize) -> String {
        return format_for_print(self.registers[register].clone().unwrap(), newline);
    }

    pub fn must_jump(&mut self, register: usize) -> bool {
        //checks if expresion is false
        let value = self.registers[register].clone();
        if
            value ==
                (Some(StackValue::BIGINT {
                    value: BigInt::from(0),
                })) ||
            value ==
                (Some(StackValue::String {
                    value: "".to_string(),
                }))
        {
            return true;
        }
        return value == Some(StackValue::Bool { value: false });
    }

    pub fn define_var(&mut self, id: usize, register: usize) {
        self.variables.insert(id, Variable { id, value: self.registers[register].clone().unwrap() });
    }

    pub fn define_var_with_stackvalue(&mut self, id: usize, value: StackValue) {
        self.variables.insert(id, Variable { id, value });
        
    }

    pub fn assign_var(&mut self, id: usize, value_reg: usize) {
        let value = self.registers[value_reg].clone();
        let getter = self.variables.get(&id);
        if getter.is_none() {
            panic!("Cound not found vairable with ID {}", id);
        } else {
            self.variables.get_mut(&id).unwrap().value = value.unwrap();
        }
    }

    pub fn load_var(&mut self, id: usize, target_reg: usize) {
        let getter = self.variables.get(&id);
        if getter.is_none() {
            panic!("Cound not found vairable id {}", id);
        } else {
            let value = getter.unwrap().value.clone();
            self.registers[target_reg] = Some(value);
        }
    }

    pub fn input(&mut self, prompt: &str, target_reg: usize) {
        print!("{}", prompt);
        let _ = io::stdout().flush();
        let res = io
            ::stdin()
            .lock()
            .lines()
            .next()
            .unwrap()
            .map(|x| x.trim_end().to_owned());
        self.registers[target_reg] = Some(StackValue::String {
            value: res.unwrap(),
        });
    }

    pub fn call_function(&mut self, name: &String, program: &mut CelsiumProgram) {
        for function in &program.modules.clone()[0].functions {
            if function.signature.name == name.to_string() {
                program.run(self, &function.body.bytecode);
            }
        }
    }
    pub fn simple_loop(&mut self, program: &mut CelsiumProgram, loop_block: Vec<OPTCODE>, count_reg: usize) {
        let count = self.registers[count_reg].clone();
        match count {
            Some(StackValue::BIGINT { value }) => {
                let mut counter = BigInt::from(0);
                while counter < value {
                    program.run(self, &loop_block);
                    counter += 1;
                }
            }
            _ => panic!(),
        }
    }
    pub fn get_object_field(&mut self, field_name: &str, object_reg: usize) {
        let object = self.registers[object_reg].clone();
        match object {
            Some(StackValue::Object { value }) => {
                for field in value {
                    if field.name == field_name {
                        self.registers[object_reg] = Some(field.value);
                        break;
                    }
                }
            }
            _ => panic!("not an object"),
        }
    }
}
