use super::{ format_for_print::format_for_print, math_operators::*, StackValue };
use crate::{ bytecode::OPTCODE, CelsiumProgram, BuiltinTypes };
use num::BigInt;
use std::{ collections::{ HashMap, LinkedList }, io::{ self, BufRead, Write }, str::FromStr };



pub struct VM {
    pub(crate) stack: LinkedList<StackValue>,
    pub(crate) variables: HashMap<usize, Variable>,
}
#[derive(Clone, Debug)]
pub struct Variable {
    pub(crate) id: usize,
    pub(crate) value: StackValue,
}

impl StackValue {}

impl VM {
    pub fn new() -> VM {
        VM {
            stack: LinkedList::new(),
            variables: HashMap::new(),
        }
    }
    pub fn push(&mut self, data_type: &BuiltinTypes, data: &String) {
        match data_type {
            BuiltinTypes::MagicInt =>
                self.stack.push_back(StackValue::BIGINT {
                    value: BigInt::from_str(&data).unwrap(),
                }),
            BuiltinTypes::Bool => {
                if data == "1" {
                    self.stack.push_back(StackValue::Bool { value: true })
                } else if data == "0" {
                    self.stack.push_back(StackValue::Bool { value: false })
                }
            }
            BuiltinTypes::String =>
                self.stack.push_back(StackValue::String {
                    value: data.to_string(),
                }),
            BuiltinTypes::Object{ fields: _} => panic!("object should not appear in bytecode"),
            BuiltinTypes::Float =>
                self.stack.push_back(StackValue::Float { value: data.parse().unwrap() }),
        }
    }
    pub fn push_stackvalue(&mut self, stackvalue: StackValue) {
        self.stack.push_back(stackvalue);
    }
    pub fn pop(&mut self) -> StackValue {
        return self.stack.pop_back().unwrap();
    }
    pub fn aritmethics(&mut self, action: &str) {
        //println!("action {}", action);
        let b = self.stack.pop_back().unwrap();
        let a = self.stack.pop_back().unwrap();
        match action {
            "+" => self.stack.push_back(add(a, b)),
            "-" => self.stack.push_back(subtract(a, b)),
            "*" => self.stack.push_back(multiply(a, b)),
            "/" => self.stack.push_back(divide(a, b)),
            "%" => self.stack.push_back(remainder(a, b)),
            "<" => self.stack.push_back(less_than(a, b)),
            ">" => self.stack.push_back(larger_than(a, b)),
            "<=" => self.stack.push_back(less_or_eq(a, b)),
            ">=" => self.stack.push_back(larger_or_eq(a, b)),
            "!=" => self.stack.push_back(not_eq(a, b)),
            "==" => self.stack.push_back(eq(a, b)),
            "and" => self.stack.push_back(and(a, b)),
            "or" => self.stack.push_back(or(a, b)),
            "xor" => self.stack.push_back(xor(a, b)),

            _ => panic!("Unknown arithmetics operator"),
        }
    }
    pub fn format_for_print(&mut self, newline: bool) -> String {
        return format_for_print(self.stack.pop_back().unwrap(), newline);
    }

    pub fn must_jump(&mut self) -> bool {
        let value = self.stack.pop_back().unwrap();
        if
            value ==
                (StackValue::BIGINT {
                    value: BigInt::from(0),
                }) ||
            value ==
                (StackValue::String {
                    value: "".to_string(),
                })
        {
            return true;
        }
        return value == StackValue::Bool { value: false };
    }

    pub fn define_var(&mut self, id: usize) {
        self.variables.insert(id, Variable { id, value: self.stack.pop_back().unwrap() });
    }

    pub fn assign_var(&mut self, id: usize) {
        let value = self.stack.pop_back().unwrap();
        let getter = self.variables.get(&id);
        if getter.is_none() {
            panic!("Cound not found vairable with ID {}", id);
        } else {
            self.variables.get_mut(&id).unwrap().value = value;
        }
    }

    pub fn load_var(&mut self, id: usize) {
        let getter = self.variables.get(&id);
        if getter.is_none() {
            panic!("Cound not found vairable id {}", id);
        } else {
            let value = getter.unwrap().value.clone();
            self.stack.push_back(value);
        }
    }

    pub fn input(&mut self, prompt: &str) {
        print!("{}", prompt);
        let _ = io::stdout().flush();
        let res = io
            ::stdin()
            .lock()
            .lines()
            .next()
            .unwrap()
            .map(|x| x.trim_end().to_owned());
        self.stack.push_back(StackValue::String {
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
    pub fn simple_loop(&mut self, program: &mut CelsiumProgram, loop_block: Vec<OPTCODE>) {
        let count = self.stack.pop_back().unwrap();
        match count {
            StackValue::BIGINT { value } => {
                let mut counter = BigInt::from(0);
                while counter < value {
                    program.run(self, &loop_block);
                    counter += 1;
                }
            }
            _ => panic!(),
        }
    }
}
