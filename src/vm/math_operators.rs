use num::bigint::ToBigUint;
use num::ToPrimitive;
use num::{BigInt, BigUint};

use super::StackValue;

fn truncate_biguint_to_f64(a: &BigUint) -> f64 {
    use std::u32;
    let mask = BigUint::from(u32::MAX);
    (a & mask).to_u32().unwrap() as f64
}

fn add_to_bigint(a: BigInt, b: StackValue) -> StackValue {
    match b {
        StackValue::Bool { value: _ } => panic!("Cannot add Bool to INT"),
        StackValue::BIGINT { value } => StackValue::BIGINT { value: a + value },
        StackValue::String { value } => StackValue::String {
            value: a.to_string() + &value,
        },
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
        StackValue::Float { value } => {
            if value.trunc() == value {
                StackValue::BIGINT {
                    value: a + value as i64,
                }
            } else {
                StackValue::Float {
                    value: truncate_biguint_to_f64(&a.to_biguint().unwrap()) + value,
                }
            }
        },
        StackValue::Object { value: _} => panic!("Cannot do math with objects.")
    }
}
fn add_to_string(a: String, b: StackValue) -> StackValue {
    match b {
        StackValue::Bool { value: _ } => panic!("Cannot add Bool to String"),
        StackValue::BIGINT { value } => StackValue::String {
            value: a + &value.to_string(),
        },
        StackValue::String { value } => StackValue::String { value: a + &value },
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
        StackValue::Float { value } => StackValue::String {
            value: a + &value.to_string().replace(".", ","),
        },
        StackValue::Object { value: _} => panic!("Cannot do math with objects.")
    }
}
fn add_to_float(a: f64, b: StackValue) -> StackValue {
    match b {
        StackValue::Bool { value: _ } => panic!("Cannot add Bool to Float"),
        StackValue::BIGINT { value } => StackValue::Float {
            value: a + truncate_biguint_to_f64(&value.to_biguint().unwrap()),
        },
        StackValue::String { value } => StackValue::String {
            value: a.to_string().replace(".", ",") + &value,
        },
        StackValue::ARRAY { value: _ } => panic!("Cannot do math with arrays"),
        StackValue::Float { value } => StackValue::Float { value: a + value },
        StackValue::Object { value: _} => panic!("Cannot do math with objects.")
    }
}
pub fn add(a: StackValue, b: StackValue) -> StackValue {
    match a {
        StackValue::Bool { value: _ } => panic!("Cannot do addition with bool's"),
        StackValue::BIGINT { value } => add_to_bigint(value, b),
        StackValue::String { value } => add_to_string(value, b),
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
        StackValue::Float { value } => add_to_float(value, b),
        StackValue::Object { value: _} => panic!("Cannot do math with objects.")
    }
}
fn subtract_from_bigint(a: BigInt, b: StackValue) -> StackValue {
    match b {
        StackValue::Bool { value: _ } => panic!("Cannot subtract Bool to INT"),
        StackValue::BIGINT { value } => StackValue::BIGINT { value: a - value },
        StackValue::String { value: _ } => panic!("Cannot subtract string from MAGICINT"),
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),

        StackValue::Float { value } => StackValue::Float {
            value: truncate_biguint_to_f64(&a.to_biguint().unwrap()) - value,
        },
        StackValue::Object { value: _} => panic!("Cannot do math with objects.")
    }
}
fn subtract_from_float(a: f64, b: StackValue) -> StackValue {
    match b {
        StackValue::Bool { value: _ } => panic!("Cannot subtract Bool from Float"),
        StackValue::BIGINT { value } => StackValue::Float {
            value: a - truncate_biguint_to_f64(&value.to_biguint().unwrap()),
        },
        StackValue::String { value: _ } => panic!("Cannot subtract strings"),
        StackValue::ARRAY { value: _ } => panic!("Cannot do math with arrays"),
        StackValue::Float { value } => StackValue::Float { value: a - value },
        StackValue::Object { value: _} => panic!("Cannot do math with objects.")
    }
}
pub fn subtract(a: StackValue, b: StackValue) -> StackValue {
    match a {
        StackValue::Bool { value: _ } => panic!("Cannot do subtraction with Bool's"),
        StackValue::BIGINT { value } => subtract_from_bigint(value, b),
        StackValue::String { value: _ } => panic!("Cannot do subtraction with String's"),
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
        StackValue::Float { value } => subtract_from_float(value, a),
        StackValue::Object { value: _} => panic!("Cannot do math with objects.")
    }
}
fn multiply_with_bigint(a: BigInt, b: StackValue) -> StackValue {
    match b {
        StackValue::Bool { value: _ } => panic!("Cannot multiply Bool with MAGICINT"),
        StackValue::BIGINT { value } => StackValue::BIGINT { value: a * value },
        StackValue::String { value: _ } => panic!("Cannot multiply string with MAGICINT"),
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
        StackValue::Float { value } => StackValue::Float {
            value: value * truncate_biguint_to_f64(&a.to_biguint().unwrap()),
        },
        StackValue::Object { value: _} => panic!("Cannot do math with objects.")
    }
}
fn multiply_with_float(a: f64, b: StackValue) -> StackValue {
    match b {
        StackValue::Bool { value: _ } => panic!("Cannot multiply Bool with Float"),
        StackValue::BIGINT { value } => StackValue::Float {
            value: a * truncate_biguint_to_f64(&value.to_biguint().unwrap()),
        },
        StackValue::String { value: _ } => panic!("Cannot multiply string with MAGICINT"),
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
        StackValue::Float { value } => StackValue::Float { value: value * a },
        StackValue::Object { value: _} => panic!("Cannot do math with objects.")
    }
}
pub fn multiply(a: StackValue, b: StackValue) -> StackValue {
    match a {
        StackValue::Bool { value: _ } => panic!("Cannot do multiplication with Bool's"),
        StackValue::BIGINT { value } => multiply_with_bigint(value, b),
        StackValue::String { value: _ } => panic!("Cannot do multiplication with String's"),
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
        StackValue::Float { value } => multiply_with_float(value, b),
        StackValue::Object { value: _} => panic!("Cannot do math with objects.")
    }
}
fn divide_with_bigint(a: BigInt, b: StackValue) -> StackValue {
    match b {
        StackValue::Bool { value: _ } => panic!("Cannot divide Bool with MAGICINT"),
        StackValue::BIGINT { value } => StackValue::BIGINT { value: a / value },
        StackValue::String { value: _ } => panic!("Cannot divide String with MAGICINT"),
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
        StackValue::Float { value } => StackValue::Float {
            value: value / truncate_biguint_to_f64(&a.to_biguint().unwrap()),
            
        },
        StackValue::Object { value: _} => panic!("Cannot do math with objects.")
    }
}
fn divide_with_float(a: f64, b: StackValue) -> StackValue {
    match b {
        StackValue::Bool { value: _ } => panic!("Cannot multiply Bool with Float"),
        StackValue::BIGINT { value } => StackValue::Float {
            value: truncate_biguint_to_f64(&value.to_biguint().unwrap()) / a,
        },
        StackValue::String { value: _ } => panic!("Cannot multiply string with MAGICINT"),
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
        StackValue::Float { value } => StackValue::Float { value: a / value },
        StackValue::Object { value: _} => panic!("Cannot do math with objects.")
    }
}
pub fn divide(a: StackValue, b: StackValue) -> StackValue {
    match a {
        StackValue::Bool { value: _ } => panic!("Cannot do division with Bool's"),
        StackValue::BIGINT { value } => divide_with_bigint(value, b),
        StackValue::String { value: _ } => panic!("Cannot do division with String's"),
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
        StackValue::Float { value } => divide_with_float(value, b),
        StackValue::Object { value: _} => panic!("Cannot do math with objects.")
    }
}
fn get_remainder_with_bigint(a: BigInt, b: StackValue) -> StackValue {
    match b {
        StackValue::Bool { value: _ } => panic!("Cannot divide Bool with MAGICINT"),
        StackValue::BIGINT { value } => StackValue::BIGINT { value: a % value },
        StackValue::String { value: _ } => panic!("Cannot divide String with MAGICINT"),
        StackValue::ARRAY { value: _ } => panic!("Cannot do math with arrays"),
        StackValue::Float { value } => StackValue::Float {
            value: truncate_biguint_to_f64(&a.to_biguint().unwrap()) % value,
        },
        StackValue::Object { value: _} => panic!("Cannot do math with objects.")
    }
}
fn get_remainder_with_float(a: f64, b: StackValue) -> StackValue {
    match b {
        StackValue::Bool { value: _ } => panic!("Cannot divide Bool with MAGICINT"),
        StackValue::BIGINT { value } => StackValue::Float {
            value: a % truncate_biguint_to_f64(&value.to_biguint().unwrap()),
        },
        StackValue::String { value: _ } => panic!("Cannot divide String with MAGICINT"),
        StackValue::ARRAY { value: _ } => panic!("Cannot do math with arrays"),
        StackValue::Float { value } => StackValue::Float {
            value: truncate_biguint_to_f64(&a.to_biguint().unwrap()) % value,
        },
        StackValue::Object { value: _} => panic!("Cannot do math with objects.")
    }
}
pub fn remainder(a: StackValue, b: StackValue) -> StackValue {
    match a {
        StackValue::Bool { value: _ } => panic!("Cannot do division with Bool's"),
        StackValue::BIGINT { value } => get_remainder_with_bigint(value, b),
        StackValue::String { value: _ } => panic!("Cannot do division with String's"),
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
        StackValue::Float { value } => get_remainder_with_float(value, b),
        StackValue::Object { value: _} => panic!("Cannot do math with objects.")
    }
}
pub fn less_than(a: StackValue, b: StackValue) -> StackValue {
    match a {
        StackValue::Bool { value: val_a } => match b {
            StackValue::Bool { value } => StackValue::Bool {
                value: val_a < value,
            },
            StackValue::BIGINT { value: _ } => panic!("Cannot compare MAGICINT with Bool"),
            StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
            StackValue::String { value: _ } => panic!("Cannot compare MAGICINT with String"),
            StackValue::Float { value: _ } => panic!("Cannot compare Bool and Float"),
            StackValue::Object { value: _} => panic!("Cannot do math with objects.")
        },
        StackValue::Object { value: _} => panic!("Cannot do math with objects."),
        StackValue::BIGINT { value: val_a } => match b {
            StackValue::Bool { value: _ } => panic!("Cannot compare MAGICINT with Bool"),
            StackValue::BIGINT { value } => StackValue::Bool {
                value: val_a < value,
            },
            StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
            StackValue::String { value: _ } => panic!("Cannot compare MAGICINT with String"),
            StackValue::Float { value } => StackValue::Bool {
                value: truncate_biguint_to_f64(&val_a.to_biguint().unwrap()) < value,
            },
            StackValue::Object { value: _} => panic!("Cannot do math with objects.")
        },
        StackValue::String { value: _ } => panic!("Cannot do comparisons  with String's"),
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
        StackValue::Float { value: val_a } => match b {
            StackValue::Bool { value: _ } => panic!("Cannot do comparison with bool"),
            StackValue::BIGINT { value } => StackValue::Bool {
                value: val_a < truncate_biguint_to_f64(&value.to_biguint().unwrap()),
            },
            StackValue::Float { value } => StackValue::Bool {
                value: val_a < value,
            },
            StackValue::String { value: _ } => panic!("Cannot do comparisons with StringS"),
            StackValue::ARRAY { value: _ } => panic!("Cannot do comparisons with ARRAYS"),
            StackValue::Object { value: _} => panic!("Cannot do math with objects.")
        },
    }
}
pub fn larger_than(a: StackValue, b: StackValue) -> StackValue {
    match a {
        StackValue::Bool { value: val_a } => match b {
            StackValue::Bool { value } => StackValue::Bool {
                value: val_a > value,
            },
            StackValue::BIGINT { value: _ } => panic!("Cannot compare MAGICINT with Bool"),
            StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
            StackValue::String { value: _ } => panic!("Cannot compare MAGICINT with String"),
            StackValue::Float { value: _ } => panic!("Cannot compare Bool and Float"),
            StackValue::Object { value: _} => panic!("Cannot do math with objects.")
        },
        StackValue::BIGINT { value: val_a } => match b {
            StackValue::Bool { value: _ } => panic!("Cannot compare MAGICINT with Bool"),
            StackValue::BIGINT { value } => StackValue::Bool {
                value: val_a > value,
            },
            StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
            StackValue::String { value: _ } => panic!("Cannot compare MAGICINT with String"),
            StackValue::Float { value } => StackValue::Bool {
                value: truncate_biguint_to_f64(&val_a.to_biguint().unwrap()) > value,
            },
            StackValue::Object { value: _} => panic!("Cannot do math with objects.")
        },
        StackValue::String { value: _ } => panic!("Cannot do comparisons  with String's"),
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
        StackValue::Float { value: val_a } => match b {
            StackValue::Bool { value: _ } => panic!("Cannot do comparison with bool"),
            StackValue::BIGINT { value } => StackValue::Bool {
                value: val_a > truncate_biguint_to_f64(&value.to_biguint().unwrap()),
            },
            StackValue::Float { value } => StackValue::Bool {
                value: val_a > value,
            },
            StackValue::String { value: _ } => panic!("Cannot do comparisons with StringS"),
            StackValue::ARRAY { value: _ } => panic!("Cannot do comparisons with ARRAYS"),
            StackValue::Object { value: _} => panic!("Cannot do math with objects.")
        },
        StackValue::Object { value: _} => panic!("Cannot do math with objects.")
    }
}
pub fn less_or_eq(a: StackValue, b: StackValue) -> StackValue {
    match a {
        StackValue::Bool { value: val_a } => match b {
            StackValue::Bool { value } => StackValue::Bool {
                value: val_a <= value,
            },
            StackValue::BIGINT { value: _ } => panic!("Cannot compare MAGICINT with Bool"),
            StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
            StackValue::String { value: _ } => panic!("Cannot compare MAGICINT with String"),
            StackValue::Float { value: _ } => panic!("Cannot compare Bool and Float"),
            StackValue::Object { value: _ } => panic!("Cannot do math with objects.")
        },
        StackValue::BIGINT { value: val_a } => match b {
            StackValue::Bool { value: _ } => panic!("Cannot compare MAGICINT with Bool"),
            StackValue::BIGINT { value } => StackValue::Bool {
                value: val_a <= value,
            },
            StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
            StackValue::String { value: _ } => panic!("Cannot compare MAGICINT with String"),
            StackValue::Float { value } => StackValue::Bool {
                value: truncate_biguint_to_f64(&val_a.to_biguint().unwrap()) <= value,
            },
            StackValue::Object { value: _} => panic!("Cannot do math with objects.")
        },
        StackValue::String { value: _ } => panic!("Cannot do comparisons  with String's"),
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
        StackValue::Float { value: val_a } => match b {
            StackValue::Bool { value: _ } => panic!("Cannot do comparison with bool"),
            StackValue::BIGINT { value } => StackValue::Bool {
                value: val_a <= truncate_biguint_to_f64(&value.to_biguint().unwrap()),
            },
            StackValue::Float { value } => StackValue::Bool {
                value: val_a <= value,
            },
            StackValue::String { value: _ } => panic!("Cannot do comparisons with StringS"),
            StackValue::ARRAY { value: _ } => panic!("Cannot do comparisons with ARRAYS"),
            StackValue::Object { value: _} => panic!("Cannot do math with objects.")
        },
        StackValue::Object { value: _} => panic!("Cannot do math with objects.")
    }
}
pub fn larger_or_eq(a: StackValue, b: StackValue) -> StackValue {
    match a {
        StackValue::Bool { value: val_a } => match b {
            StackValue::Bool { value } => StackValue::Bool {
                value: val_a >= value,
            },
            StackValue::BIGINT { value: _ } => panic!("Cannot compare MAGICINT with Bool"),
            StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
            StackValue::String { value: _ } => panic!("Cannot compare MAGICINT with String"),
            StackValue::Float { value: _ } => panic!("Cannot compare Bool and Float"),
            StackValue::Object { value: _} => panic!("Cannot do math with objects.")
        },
        StackValue::Object { value: _} => panic!("Cannot do math with objects."),
        StackValue::BIGINT { value: val_a } => match b {
            StackValue::Bool { value: _ } => panic!("Cannot compare MAGICINT with Bool"),
            StackValue::BIGINT { value } => StackValue::Bool {
                value: val_a >= value,
            },
            StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
            StackValue::String { value: _ } => panic!("Cannot compare MAGICINT with String"),
            StackValue::Float { value } => StackValue::Bool {
                value: truncate_biguint_to_f64(&val_a.to_biguint().unwrap()) >= value,
            },
            StackValue::Object { value: _} => panic!("Cannot do math with objects.")
        },
        StackValue::String { value: _ } => panic!("Cannot do comparisons  with String's"),
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
        StackValue::Float { value: val_a } => match b {
            StackValue::Bool { value: _ } => panic!("Cannot do comparison with bool"),
            StackValue::BIGINT { value } => StackValue::Bool {
                value: val_a >= truncate_biguint_to_f64(&value.to_biguint().unwrap()),
            },
            StackValue::Float { value } => StackValue::Bool {
                value: val_a >= value,
            },
            StackValue::String { value: _ } => panic!("Cannot do comparisons with StringS"),
            StackValue::ARRAY { value: _ } => panic!("Cannot do comparisons with ARRAYS"),
            StackValue::Object { value: _} => panic!("Cannot do math with objects.")
        },
    }
}
pub fn not_eq(a: StackValue, b: StackValue) -> StackValue {
    match a {
        StackValue::Bool { value: val_a } => match b {
            StackValue::Bool { value } => StackValue::Bool {
                value: val_a != value,
            },
            StackValue::BIGINT { value: _ } => panic!("Cannot compare MAGICINT with Bool"),
            StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
            StackValue::String { value: _ } => panic!("Cannot compare MAGICINT with String"),
            StackValue::Float { value: _ } => panic!("Cannot compare Float with Bool"),
            StackValue::Object { value: _} => panic!("Cannot do math with objects.")
        },
        StackValue::BIGINT { value: val_a } => match b {
            
            StackValue::Bool { value: _ } => panic!("Cannot compare MAGICINT with Bool"),
            StackValue::BIGINT { value } => StackValue::Bool {
                value: val_a != value,
            },
            StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
            StackValue::String { value: _ } => panic!("Cannot compare MAGICINT with String"),
            StackValue::Float { value } => {
                if value.trunc() == value {
                    StackValue::Bool {
                        value: val_a != BigInt::from(value as i64),
                    }
                } else {
                    StackValue::Bool { value: true }
                }
            },
            StackValue::Object { value: _} => panic!("Cannot do math with objects.")
        },
        StackValue::String { value: val_a } => match b {
            StackValue::Bool { value: _ } => panic!("Cannot compare String with Bool"),
            StackValue::BIGINT { value: _ } => panic!("Cannot compare String with BIGINT"),
            StackValue::String { value } => StackValue::Bool {
                value: val_a != value,
            },
            StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
            StackValue::Float { value: _ } => panic!("Cannot compare String with Float"),
            StackValue::Object { value: _} => panic!("Cannot do math with objects.")
        },
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
        StackValue::Float { value: value_a } => match b {
            StackValue::Bool { value: _ } => panic!("Cannot compare Float with Bool"),
            StackValue::BIGINT { value } => {
                if value_a.trunc() == value_a {
                    StackValue::Bool {
                        value: value != BigInt::from(value_a as i64),
                    }
                } else {
                    StackValue::Bool { value: false }
                }
            },
            StackValue::Float { value: _ } => StackValue::Bool {
                value: value_a != value_a,
            },
            StackValue::String { value: _ } => panic!("Cannot compare Float with String"),
            StackValue::ARRAY { value: _ } => panic!("Cannot compare Float with ARRAY"),
            StackValue::Object { value: _} => panic!("Cannot do math with objects.")
        },
        StackValue::Object { value: _} => panic!("Cannot do math with objects.")
    }
}
pub fn eq(a: StackValue, b: StackValue) -> StackValue {
    match not_eq(a, b) {
        StackValue::Bool { value } => StackValue::Bool { value: !value },
        _ => panic!()
    }
}

pub fn and(a: StackValue, b: StackValue) -> StackValue {
    if (a == StackValue::Bool { value: true } && b == StackValue::Bool { value: true }) {
        StackValue::Bool { value: true }
    } else {
        StackValue::Bool { value: false }
    }
}

pub fn or(a: StackValue, b: StackValue) -> StackValue {
    if (a == StackValue::Bool { value: true } || b == StackValue::Bool { value: true }) {
        StackValue::Bool { value: true }
    } else {
        StackValue::Bool { value: false }
    }
}

pub fn xor(a: StackValue, b: StackValue) -> StackValue {
    if (a == StackValue::Bool { value: true }) {
        if b == (StackValue::Bool { value: false }) {
            StackValue::Bool { value: true }
        } else {
            StackValue::Bool { value: false }
        }
    } else {
        if b == (StackValue::Bool { value: false }) {
            StackValue::Bool { value: false }
        } else {
            StackValue::Bool { value: true }
        }
    }
}
