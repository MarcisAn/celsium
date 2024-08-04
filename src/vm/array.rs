use num::BigInt;

use super::{vm::VM, StackValue};

impl VM {

pub fn get_from_array(&mut self, id: usize, register: usize) {
    let index_stackvalue = &self.registers[register];
    let index = match <std::option::Option<StackValue> as Clone>::clone(&index_stackvalue).unwrap() {
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
                self.registers[register] = Some(value[index].clone());
            }
            _ => panic!("{} is not an array", getter.unwrap().id),
        }
        return;
    }
}
pub fn set_at_array(&mut self, id: usize, value_reg: usize, index_reg: usize) {
    let index_stackvalue = &self.registers[index_reg];
    let index = match <std::option::Option<StackValue> as Clone>::clone(&index_stackvalue).unwrap() {
        StackValue::BIGINT { value } => value.to_string().parse::<usize>().unwrap(),
        _ => panic!("Array index is not an int"),
    };

    let getter = self.variables.get(&id);
    if getter.is_none() {
        panic!("Cound not found vairable named {}", id);
    } else {
        match getter.unwrap().value.to_owned() {
            StackValue::ARRAY { mut value } => {
                let value_to_push = &self.registers[value_reg];
                value[index] = value_to_push.clone().unwrap();
                self.variables.get_mut(&id).unwrap().value = StackValue::ARRAY { value: value };
            }
            _ => panic!("{} is not an array", getter.unwrap().id),
        }
        return;
    }
}

pub fn push_to_array(&mut self, id: usize, value_reg: usize) {
    let getter = self.variables.get(&id);
    if getter.is_none() {
        panic!("Cound not found vairable named {}", id);
    } else {
        match getter.unwrap().value.to_owned() {
            StackValue::ARRAY { mut value } => {
                let value_to_push = &self.registers[value_reg];
                value.push(value_to_push.clone().unwrap());
            }
            _ => panic!("{} is not an array", getter.unwrap().id),
        }
        return;
    }
}
pub fn get_array_length(&mut self, id: usize, target_reg: usize) {
    let getter = self.variables.get(&id);
    if getter.is_none() {
        panic!("Cound not found vairable named {}", id);
    } else {
        match getter.unwrap().value.to_owned() {
            StackValue::ARRAY { value } => {
                self.registers[target_reg] = Some(StackValue::BIGINT {
                    value: BigInt::from(value.len()),
                });
            }
            _ => panic!("{} is not an array", getter.unwrap().id),
        }
        return;
    }
}
}