use num::BigInt;

use super::{ vm::VM, StackValue };

impl VM {
    pub fn get_from_array(&mut self) {
        let index_stack = self.stack.pop_back().unwrap();
        let index = match index_stack {
            StackValue::Int { value } => value.to_string().parse::<usize>().unwrap(),
            _ => panic!("Array index is not an int"),
        };
        let array_stack = self.stack.pop_back().unwrap();
        let array = match array_stack {
            StackValue::Array { value } => value,
            _ => panic!("Atempto index non-array"),
        };
        self.stack.push_back(array[index].clone());

    }
    pub fn set_at_array(&mut self, id: usize) {
        let index_stack = self.stack.pop_back().unwrap();
        let index = match index_stack {
            StackValue::Int { value } => value.to_string().parse::<usize>().unwrap(),
            _ => panic!("Array index is not an int"),
        };

        let getter = self.variables.get(&id);
        if getter.is_none() {
            panic!("Cound not found vairable named {}", id);
        } else {
            match getter.unwrap().value.to_owned() {
                StackValue::Array { mut value } => {
                    let value_to_push = self.stack.pop_back().unwrap();
                    value[index] = value_to_push;
                    self.variables.get_mut(&id).unwrap().value = StackValue::Array { value: value };
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
                StackValue::Array { mut value } => {
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
                StackValue::Array { value } => {
                    self.stack.push_back(StackValue::Int {
                        value: value.len() as i64,
                    });
                }
                _ => panic!("{} is not an array", getter.unwrap().id),
            }
            return;
        }
    }
}
