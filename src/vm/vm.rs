use num::BigInt;
use std::{collections::LinkedList, str::FromStr};

use crate::BUILTIN_TYPES;

use super::StackValue;

enum FUNCTION {
    RUST_FUNCTION,
    USER_DEFINED_FUNCTION,
}

pub fn define_function() {}

pub struct VM {
    stack: LinkedList<StackValue>,
}
impl StackValue {
    fn add_to_bigint(a: BigInt, b: StackValue) -> StackValue {
        match b {
            StackValue::BOOL { value: _ } => panic!("Cannot add BOOL to INT"),
            StackValue::BIGINT { value } => StackValue::BIGINT { value: a + value },
            StackValue::STRING { value } => StackValue::STRING {
                value: a.to_string() + &value,
            },
        }
    }
    fn add_to_string(a: String, b: StackValue) -> StackValue {
        match b {
            StackValue::BOOL { value: _ } => panic!("Cannot add BOOL to STRING"),
            StackValue::BIGINT { value } => StackValue::STRING {
                value: a + &value.to_string(),
            },
            StackValue::STRING { value } => StackValue::STRING { value: a + &value },
        }
    }

    pub fn add(a: StackValue, b: StackValue) -> StackValue {
        match a {
            StackValue::BOOL { value: _ } => panic!("Cannot do addition with bool's"),
            StackValue::BIGINT { value } => Self::add_to_bigint(value, b),
            StackValue::STRING { value } => Self::add_to_string(value, b),
        }
    }
    fn subtract_from_bigint(a: BigInt, b: StackValue) -> StackValue {
        match b {
            StackValue::BOOL { value: _ } => panic!("Cannot subtract BOOL to INT"),
            StackValue::BIGINT { value } => StackValue::BIGINT { value: a - value },
            StackValue::STRING { value: _ } => panic!("Cannot subtract string from MAGICINT"),
        }
    }
    pub fn subtract(a: StackValue, b: StackValue) -> StackValue {
        match a {
            StackValue::BOOL { value: _ } => panic!("Cannot do subtraction with BOOL's"),
            StackValue::BIGINT { value } => Self::subtract_from_bigint(value, b),
            StackValue::STRING { value: _ } => panic!("Cannot do subtraction with STRING's"),
        }
    }
    fn multiply_with_bigint(a: BigInt, b: StackValue) -> StackValue {
        match b {
            StackValue::BOOL { value: _ } => panic!("Cannot multiply BOOL with MAGICINT"),
            StackValue::BIGINT { value } => StackValue::BIGINT { value: a * value },
            StackValue::STRING { value: _ } => panic!("Cannot multiply string with MAGICINT"),
        }
    }
    pub fn multiply(a: StackValue, b: StackValue) -> StackValue {
        match a {
            StackValue::BOOL { value: _ } => panic!("Cannot do multiplication with BOOL's"),
            StackValue::BIGINT { value } => Self::multiply_with_bigint(value, b),
            StackValue::STRING { value: _ } => panic!("Cannot do multiplication with STRING's"),
        }
    }
    fn divide_with_bigint(a: BigInt, b: StackValue) -> StackValue {
        match b {
            StackValue::BOOL { value: _ } => panic!("Cannot divide BOOL with MAGICINT"),
            StackValue::BIGINT { value } => StackValue::BIGINT { value: a / value },
            StackValue::STRING { value: _ } => panic!("Cannot divide STRING with MAGICINT"),
        }
    }
    pub fn divide(a: StackValue, b: StackValue) -> StackValue {
        match a {
            StackValue::BOOL { value: _ } => panic!("Cannot do division with BOOL's"),
            StackValue::BIGINT { value } => Self::divide_with_bigint(value, b),
            StackValue::STRING { value: _ } => panic!("Cannot do division with STRING's"),
        }
    }
    fn get_remainder_with_bigint(a: BigInt, b: StackValue) -> StackValue {
        match b {
            StackValue::BOOL { value: _ } => panic!("Cannot divide BOOL with MAGICINT"),
            StackValue::BIGINT { value } => StackValue::BIGINT { value: a % value },
            StackValue::STRING { value: _ } => panic!("Cannot divide STRING with MAGICINT"),
        }
    }
    pub fn remainder(a: StackValue, b: StackValue) -> StackValue {
        match a {
            StackValue::BOOL { value: _ } => panic!("Cannot do division with BOOL's"),
            StackValue::BIGINT { value } => Self::get_remainder_with_bigint(value, b),
            StackValue::STRING { value: _ } => panic!("Cannot do division with STRING's"),
        }
    }
    pub fn less_than(a: StackValue, b: StackValue) -> StackValue {
        match a {
            StackValue::BOOL { value: val_a } => match b {
                StackValue::BOOL { value } => StackValue::BOOL {
                    value: val_a < value,
                },
                StackValue::BIGINT { value } => panic!("Cannot compare MAGICINT with BOOL"),
                StackValue::STRING { value } => panic!("Cannot compare MAGICINT with STRING"),
            },
            StackValue::BIGINT { value: val_a } => match b {
                StackValue::BOOL { value } => panic!("Cannot compare MAGICINT with BOOL"),
                StackValue::BIGINT { value } => StackValue::BOOL {
                    value: val_a < value,
                },
                StackValue::STRING { value } => panic!("Cannot compare MAGICINT with STRING"),
            },
            StackValue::STRING { value: _ } => panic!("Cannot do comparisons  with STRING's"),
        }
    }
    pub fn larger_than(a: StackValue, b: StackValue) -> StackValue {
        match a {
            StackValue::BOOL { value: val_a } => match b {
                StackValue::BOOL { value } => StackValue::BOOL {
                    value: val_a > value,
                },
                StackValue::BIGINT { value } => panic!("Cannot compare MAGICINT with BOOL"),
                StackValue::STRING { value } => panic!("Cannot compare MAGICINT with STRING"),
            },
            StackValue::BIGINT { value: val_a } => match b {
                StackValue::BOOL { value } => panic!("Cannot compare MAGICINT with BOOL"),
                StackValue::BIGINT { value } => StackValue::BOOL {
                    value: val_a > value,
                },
                StackValue::STRING { value } => panic!("Cannot compare MAGICINT with STRING"),
            },
            StackValue::STRING { value: _ } => panic!("Cannot do comparisons  with STRING's"),
        }
    }
    pub fn less_or_eq(a: StackValue, b: StackValue) -> StackValue {
        match a {
            StackValue::BOOL { value: val_a } => match b {
                StackValue::BOOL { value } => StackValue::BOOL {
                    value: val_a <= value,
                },
                StackValue::BIGINT { value } => panic!("Cannot compare MAGICINT with BOOL"),
                StackValue::STRING { value } => panic!("Cannot compare MAGICINT with STRING"),
            },
            StackValue::BIGINT { value: val_a } => match b {
                StackValue::BOOL { value } => panic!("Cannot compare MAGICINT with BOOL"),
                StackValue::BIGINT { value } => StackValue::BOOL {
                    value: val_a <= value,
                },
                StackValue::STRING { value } => panic!("Cannot compare MAGICINT with STRING"),
            },
            StackValue::STRING { value: _ } => panic!("Cannot do comparisons  with STRING's"),
        }
    }
    pub fn larger_or_eq(a: StackValue, b: StackValue) -> StackValue {
        match a {
            StackValue::BOOL { value: val_a } => match b {
                StackValue::BOOL { value } => StackValue::BOOL {
                    value: val_a >= value,
                },
                StackValue::BIGINT { value } => panic!("Cannot compare MAGICINT with BOOL"),
                StackValue::STRING { value } => panic!("Cannot compare MAGICINT with STRING"),
            },
            StackValue::BIGINT { value: val_a } => match b {
                StackValue::BOOL { value } => panic!("Cannot compare MAGICINT with BOOL"),
                StackValue::BIGINT { value } => StackValue::BOOL {
                    value: val_a >= value,
                },
                StackValue::STRING { value } => panic!("Cannot compare MAGICINT with STRING"),
            },
            StackValue::STRING { value: _ } => panic!("Cannot do comparisons  with STRING's"),
        }
    }
    pub fn not_eq(a: StackValue, b: StackValue) -> StackValue {
        match a {
            StackValue::BOOL { value: val_a } => match b {
                StackValue::BOOL { value } => StackValue::BOOL {
                    value: val_a != value,
                },
                StackValue::BIGINT { value } => panic!("Cannot compare MAGICINT with BOOL"),
                StackValue::STRING { value } => panic!("Cannot compare MAGICINT with STRING"),
            },
            StackValue::BIGINT { value: val_a } => match b {
                StackValue::BOOL { value } => panic!("Cannot compare MAGICINT with BOOL"),
                StackValue::BIGINT { value } => StackValue::BOOL {
                    value: val_a != value,
                },
                StackValue::STRING { value } => panic!("Cannot compare MAGICINT with STRING"),
            },
            StackValue::STRING { value: val_a } => match b {
                StackValue::BOOL { value } => panic!("Cannot compare STRING with BOOL"),
                StackValue::BIGINT { value } => panic!("Cannot compare STRING with BIGINT"),
                StackValue::STRING { value } => StackValue::BOOL {
                    value: val_a != value,
                },
            },
        }
    }
    pub fn eq(a: StackValue, b: StackValue) -> StackValue {
        match a {
            StackValue::BOOL { value: val_a } => match b {
                StackValue::BOOL { value } => StackValue::BOOL {
                    value: val_a == value,
                },
                StackValue::BIGINT { value } => panic!("Cannot compare MAGICINT with BOOL"),
                StackValue::STRING { value } => panic!("Cannot compare MAGICINT with STRING"),
            },
            StackValue::BIGINT { value: val_a } => match b {
                StackValue::BOOL { value } => panic!("Cannot compare MAGICINT with BOOL"),
                StackValue::BIGINT { value } => StackValue::BOOL {
                    value: val_a == value,
                },
                StackValue::STRING { value } => panic!("Cannot compare MAGICINT with STRING"),
            },
            StackValue::STRING { value: val_a } => match b {
                StackValue::BOOL { value } => panic!("Cannot compare STRING with BOOL"),
                StackValue::BIGINT { value } => panic!("Cannot compare STRING with BIGINT"),
                StackValue::STRING { value } => StackValue::BOOL {
                    value: val_a == value,
                },
            },
        }
    }
}

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
        let a = self.stack.pop_back().unwrap();
        let b = self.stack.pop_back().unwrap();
        match action {
            "+" => self.stack.push_back(StackValue::add(a, b)),
            "-" => self.stack.push_back(StackValue::subtract(a, b)),
            "*" => self.stack.push_back(StackValue::multiply(a, b)),
            "/" => self.stack.push_back(StackValue::divide(a, b)),
            "%" => self.stack.push_back(StackValue::remainder(a, b)),
            "<" => self.stack.push_back(StackValue::less_than(a, b)),
            ">" => self.stack.push_back(StackValue::larger_than(a, b)),
            "<=" => self.stack.push_back(StackValue::less_or_eq(a, b)),
            ">=" => self.stack.push_back(StackValue::larger_or_eq(a, b)),
            "!=" => self.stack.push_back(StackValue::not_eq(a, b)),
            "==" => self.stack.push_back(StackValue::eq(a, b)),

            _ => panic!("Unknown arithmetics operator"),
        }
    }
    pub fn print_function(&mut self, newline: bool) {
        match self.stack.pop_back().unwrap() {
            StackValue::BOOL { value } => {
                if (value) {
                    print!("1")
                } else {
                    print!("0")
                }
            }
            StackValue::BIGINT { value } => print!("{}", value),
            StackValue::STRING { value } => print!("{}", value),
        };
        if newline {
            print!("\n");
        }
    }
    pub fn must_jump(&mut self) -> bool {
        return self.stack.pop_back().unwrap() == StackValue::BOOL { value: false };
    }
}
