use super::{math_operators::*, StackValue};
use crate::{module::VISIBILITY, BUILTIN_TYPES};
use num::BigInt;
use rand::seq::index;
use std::{collections::LinkedList, env::var, str::FromStr};

enum FUNCTION {
    RUST_FUNCTION,
    USER_DEFINED_FUNCTION,
}

pub fn define_function() {}

pub struct VM {
    stack: LinkedList<StackValue>,
    variables: Vec<Variable>,
}
#[derive(Clone, Debug)]
struct Variable {
    module_id: usize,
    name: String,
    value: StackValue,
    visibility: VISIBILITY,
}

fn format_for_print(value: StackValue, newline: bool) -> String {
    match value {
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
        StackValue::ARRAY { value } => {
            let mut printable_str: String = "[".to_string();
            let mut counter = 0;
            for i in &value {
                let formated: String = format_for_print(i.clone(), false).as_str().to_owned();
                match i {
                    StackValue::STRING { value } => {
                        printable_str = printable_str + "\"" + &formated + "\"";
                    }
                    _ => printable_str += &formated,
                }

                if counter != value.len() - 1 {
                    printable_str += ";";
                }
                counter += 1;
            }
            if newline {
                printable_str += "]\n";
            } else {
                printable_str += "]";
            }
            return printable_str;
        }
    };
}

impl StackValue {}

impl VM {
    pub fn new() -> VM {
        VM {
            stack: LinkedList::new(),
            variables: vec![],
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
        return "".to_owned();
    }

    pub fn must_jump(&mut self) -> bool {
        let value = self.stack.pop_back().unwrap();
        if value
            == (StackValue::BIGINT {
                value: BigInt::from(0),
            })
            || value
                == (StackValue::STRING {
                    value: "".to_string(),
                })
        {
            return true;
        }
        return value == StackValue::BOOL { value: false };
    }

    pub fn define_var(&mut self, module_id: usize, name: String, visibility: &VISIBILITY) {
        self.variables.push(Variable {
            module_id,
            name: name.clone(),
            value: self.stack.pop_back().unwrap(),
            visibility: visibility.clone(),
        });
        println!("defining {}", name);
        println!("{:?}", self.variables);
    }

    pub fn define_array(
        &mut self,
        module_id: usize,
        name: String,
        visibility: &VISIBILITY,
        init_value_count: usize,
    ) {
        let mut init_values = vec![];
        for i in 0..init_value_count {
            init_values.push(self.stack.pop_back().unwrap());
        }
        init_values.reverse();
        self.variables.push(Variable {
            module_id,
            name,
            value: StackValue::ARRAY { value: init_values },
            visibility: visibility.clone(),
        })
    }
    pub fn get_from_array(&mut self, name: &String) {
        let index_stack = self.stack.pop_back().unwrap();
        let index = match index_stack {
            StackValue::BIGINT { value } => value.to_string().parse::<usize>().unwrap(),
            _ => panic!("Array index is not an int"),
        };
        for var in &self.variables {
            if var.clone().name == name.to_string() {
                match var.value.clone() {
                    StackValue::ARRAY { value } => {
                        if value.len() > index {
                            self.stack.push_back(value[index].clone());
                        } else {
                            panic!("The index  is too high")
                        }
                    }
                    _ => panic!("{} is not an array", var.name),
                };
                return;
            }
        }
        panic!("Cound not found vairable named {}", name);
    }
    fn get_index(&mut self, name: &String) -> i32 {
        let mut counter = 0;
        for var in &mut self.variables {
            if &var.name.clone() == &name.to_string() {
                return counter;
            }
            counter += 1;
        }
        panic!("Cound not found vairable named {}", name);
    }
    pub fn push_to_array(&mut self, name: &String) {
        let index = self.get_index(&name.clone());
        match self.variables[index as usize].value {
            StackValue::ARRAY { ref mut value } => {
                value.push(self.stack.pop_back().unwrap());
            }
            _ => panic!("The variable is not an array"),
        };
        return;
    }
    pub fn get_array_length(&mut self, name: &String) {
        for var in &self.variables {
            if var.clone().name == name.to_string() {
                match var.value.clone() {
                    StackValue::ARRAY { mut value } => {
                        self.stack.push_back(StackValue::BIGINT {
                            value: BigInt::from(value.len()),
                        });
                    }
                    _ => panic!("{} is not an array", var.name),
                };
                return;
            }
        }
        panic!("Cound not found vairable named {}", name);
    }
    pub fn assign_var(&mut self, name: &str) {
        let value = self.stack.pop_back().unwrap();
        for var in &mut self.variables {
            if var.name == name {
                var.value = value.clone();
                return;
            }
        }
        panic!("Cound not found vairable named {}", name);
    }

    pub fn load_var(&mut self, name: &str) {
        println!("{:?}", self.variables);

        for var in &self.variables {
            if var.name == name {
                self.stack.push_back(var.value.clone());
                return;
            }
        }
        panic!("Cound not found vairable named {}", name);
    }
}
