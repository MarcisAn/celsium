use super::{math_operators::*, StackValue};
use crate::BUILTIN_TYPES;
use num::BigInt;
use std::{collections::LinkedList, str::FromStr};

enum FUNCTION {
    RUST_FUNCTION,
    USER_DEFINED_FUNCTION,
}

pub fn define_function() {}

pub struct VM {
    stack: LinkedList<StackValue>,
}
impl StackValue {}

impl VM {
    pub fn new() -> VM {
        VM {
            stack: LinkedList::new(),
        }
    }
    pub fn push(&mut self, data_type: &BUILTIN_TYPES, data: &String) {
        match data_type {
            BUILTIN_TYPES::MAGIC_INT => self.stack.push_back(StackValue::BIGINT {
                value: BigInt::from_str(&data).unwrap(),
            }),
            BUILTIN_TYPES::BOOL => {
                if (data == "1") {
                    self.stack.push_back(StackValue::BOOL { value: true })
                } else if (data == "0") {
                    self.stack.push_back(StackValue::BOOL { value: false })
                }
            }
            BUILTIN_TYPES::STRING => self.stack.push_back(StackValue::STRING {
                value: data.to_string(),
            }),
        }
    }
    pub fn aritmethics(&mut self, action: &str) {
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
        match self.stack.pop_back().unwrap() {
            StackValue::BOOL { value } => {
                if !newline {
                    if (value) {
                        return "1".to_owned();
                    } else {
                        return "0".to_owned();
                    }
                } else {
                    if (value) {
                        return "1\n".to_owned();
                    } else {
                        return "0\n".to_owned();
                    }
                }
            }
            StackValue::BIGINT { value } => {
                if !newline {
                    return format!("{}", value);
                } else {
                    return format!("{}\n", value);
                }
            }
            StackValue::STRING { value } => {
                if !newline {
                    return format!("{}", value);
                } else {
                    return format!("{}\n", value);
                }
            }
        };
        return "".to_owned();
    }

    pub fn must_jump(&mut self) -> bool {
        return self.stack.pop_back().unwrap() == StackValue::BOOL { value: false };
    }
}
