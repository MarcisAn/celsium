use num::BigInt;

use super::{vm::VM, StackValue};

impl VM {

pub fn get_from_array(&mut self, id: usize) {
    let index_stack = self.stack.pop_back().unwrap();
    let index = match index_stack {
        StackValue::BIGINT { value } => value.to_string().parse::<usize>().unwrap(),
        _ => panic!("Array index is not an int"),
    };

    let getter = self.variables.get(&id);
    if getter.is_none() {
        panic!("Cound not found vairable named {}", id);
    } else {
        let stackvalue = getter.unwrap().value.to_owned();
        match stackvalue {
            StackValue::ARRAY { value } => {
                self.stack.push_back(value[index].clone());
            }
            _ => panic!("{} is not an array", getter.unwrap().id),
        }
        return;
    }
}
pub fn set_at_array(&mut self, id: usize) {
    let index_stack = self.stack.pop_back().unwrap();
    let index = match index_stack {
        StackValue::BIGINT { value } => value.to_string().parse::<usize>().unwrap(),
        _ => panic!("Array index is not an int"),
    };

    let getter = self.variables.get(&id);
    if getter.is_none() {
        panic!("Cound not found vairable named {}", id);
    } else {
        match getter.unwrap().value.to_owned() {
            StackValue::ARRAY { mut value } => {
                let value_to_push = self.stack.pop_back().unwrap();
                value[index] = value_to_push;
                self.variables.get_mut(&id).unwrap().value = StackValue::ARRAY { value: value };
            }
            _ => panic!("{} is not an array", getter.unwrap().id),
        }
        return;
    }
}

pub fn push_to_array(&mut self, id: usize) {
    let getter = self.variables.get(&id);
    if getter.is_none() {
        panic!("Cound not found vairable named {}", id);
    } else {
        match getter.unwrap().value.to_owned() {
            StackValue::ARRAY { mut value } => {
                let value_to_push = self.stack.pop_back().unwrap();
                value.push(value_to_push);
            }
            _ => panic!("{} is not an array", getter.unwrap().id),
        }
        return;
    }
}
pub fn get_array_length(&mut self, id: usize) {
    let getter = self.variables.get(&id);
    if getter.is_none() {
        panic!("Cound not found vairable named {}", id);
    } else {
        match getter.unwrap().value.to_owned() {
            StackValue::ARRAY { value } => {
                self.stack.push_back(StackValue::BIGINT {
                    value: BigInt::from(value.len()),
                });
            }
            _ => panic!("{} is not an array", getter.unwrap().id),
        }
        return;
    }
}
}