use super::{math_operators::*, StackValue};
use crate::{bytecode::OPTCODE, module::VISIBILITY, CelsiumProgram, BUILTIN_TYPES};
use num::BigInt;
use rand::Rng;
use std::{
    collections::{HashMap, LinkedList},
    io::{self, BufRead, Write},
    str::FromStr,
};

fn generate_rand_varname(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789\
                            ~!@#$%^&*()-_+=";

    let mut rng = rand::thread_rng();
    let randstring: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    randstring
}

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
                if value {
                    return "1".to_owned();
                } else {
                    return "0".to_owned();
                }
            } else {
                if value {
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
                    StackValue::STRING { value: _ } => {
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
        StackValue::OBJECT {
            name,
            value: fields,
        } => {
            let mut printable_object = format!("{} {{\n", name);
            let mut index = 0;
            let length = &fields.len();
            for field in fields {
                printable_object += &format!(
                    "   {}: {}",
                    field.name,
                    format_for_print(field.value, false)
                );
                if &(index + 2) == length {
                    printable_object += "\n";
                }
                index += 1;
            }
            printable_object += "\n}";
            if !newline {
                return format!("{}", printable_object);
            } else {
                return format!("{}\n", printable_object);
            }
        }
        StackValue::FLOAT { value } => {
            if !newline {
                return format!("{}", value.to_string().replace(".", ","));
            } else {
                return format!("{}\n", value.to_string().replace(".", ","));
            }
        },
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
                if data == "1" {
                    self.stack.push_back(StackValue::BOOL { value: true })
                } else if data == "0" {
                    self.stack.push_back(StackValue::BOOL { value: false })
                }
            }
            BUILTIN_TYPES::STRING => self.stack.push_back(StackValue::STRING {
                value: data.to_string(),
            }),
            BUILTIN_TYPES::OBJECT => panic!(),
            BUILTIN_TYPES::FLOAT => self.stack.push_back(StackValue::FLOAT { value: data.parse().unwrap() }),
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
    }

    pub fn define_array(
        &mut self,
        module_id: usize,
        name: String,
        visibility: &VISIBILITY,
        init_value_count: usize,
    ) {
        let mut init_values = vec![];
        for _ in 0..init_value_count {
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
                    StackValue::ARRAY { value } => {
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
        for var in &self.variables {
            if var.name == name {
                self.stack.push_back(var.value.clone());
                return;
            }
        }
        panic!("Cound not found vairable named {}", name);
    }

    pub fn input(&mut self, prompt: &str) {
        print!("{}", prompt);
        let _ = io::stdout().flush();
        let res = io::stdin()
            .lock()
            .lines()
            .next()
            .unwrap()
            .map(|x| x.trim_end().to_owned());
        self.stack.push_back(StackValue::STRING {
            value: res.unwrap(),
        });
    }

    pub fn call_function(&mut self, name: &String, program: &mut CelsiumProgram) {
        for function in &program.modules.clone()[0].functions {
            if function.signature.name == name.to_string() {
                let mut argument_names_to_replace = HashMap::new();
                let mut func_args = function.clone().signature.args;
                func_args.reverse();
                for arg in func_args {
                    let var_name =
                        "__".to_string() + &arg.name.to_string() + &generate_rand_varname(5);
                    self.define_var(0, var_name.clone(), &VISIBILITY::PRIVATE);
                    argument_names_to_replace.insert(arg.clone().name, var_name);
                }
                let mut replaced_bytecode: Vec<OPTCODE> = vec![];
                for optcode in &function.body.bytecode.clone() {
                    match optcode {
                        OPTCODE::LOAD_VAR { name } => match argument_names_to_replace.get(name) {
                            Some(ref new_name) => replaced_bytecode.push(OPTCODE::LOAD_VAR {
                                name: new_name.to_string(),
                            }),
                            None => replaced_bytecode.push(OPTCODE::LOAD_VAR {
                                name: name.to_string(),
                            }),
                        },
                        _ => replaced_bytecode.push(optcode.clone()),
                    }
                }
                program.run(self, &replaced_bytecode);
            }
        }
    }
}
