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
        StackValue::BOOL { value: _ } => panic!("Cannot add BOOL to INT"),
        StackValue::BIGINT { value } => StackValue::BIGINT { value: a + value },
        StackValue::STRING { value } => StackValue::STRING {
            value: a.to_string() + &value,
        },
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
        StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do math with objects"),
        StackValue::FLOAT { value } => {
            if value.trunc() == value {
                StackValue::BIGINT {
                    value: a + value as i64,
                }
            } else {
                StackValue::FLOAT {
                    value: truncate_biguint_to_f64(&a.to_biguint().unwrap()) + value,
                }
            }
        }
    }
}
fn add_to_string(a: String, b: StackValue) -> StackValue {
    match b {
        StackValue::BOOL { value: _ } => panic!("Cannot add BOOL to STRING"),
        StackValue::BIGINT { value } => StackValue::STRING {
            value: a + &value.to_string(),
        },
        StackValue::STRING { value } => StackValue::STRING { value: a + &value },
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
        StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do math with objects"),
        StackValue::FLOAT { value } => StackValue::STRING {
            value: a + &value.to_string().replace(".", ","),
        },
    }
}
fn add_to_float(a: f64, b: StackValue) -> StackValue {
    match b {
        StackValue::BOOL { value: _ } => panic!("Cannot add BOOL to FLOAT"),
        StackValue::BIGINT { value } => StackValue::FLOAT {
            value: a + truncate_biguint_to_f64(&value.to_biguint().unwrap()),
        },
        StackValue::STRING { value } => StackValue::STRING {
            value: a.to_string().replace(".", ",") + &value,
        },
        StackValue::ARRAY { value: _ } => panic!("Cannot do math with arrays"),
        StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do math with objects"),
        StackValue::FLOAT { value } => StackValue::FLOAT { value: a + value },
    }
}
pub fn add(a: StackValue, b: StackValue) -> StackValue {
    match a {
        StackValue::BOOL { value: _ } => panic!("Cannot do addition with bool's"),
        StackValue::BIGINT { value } => add_to_bigint(value, b),
        StackValue::STRING { value } => add_to_string(value, b),
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
        StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do math with objects"),
        StackValue::FLOAT { value } => add_to_float(value, b),
    }
}
fn subtract_from_bigint(a: BigInt, b: StackValue) -> StackValue {
    match b {
        StackValue::BOOL { value: _ } => panic!("Cannot subtract BOOL to INT"),
        StackValue::BIGINT { value } => StackValue::BIGINT { value: a - value },
        StackValue::STRING { value: _ } => panic!("Cannot subtract string from MAGICINT"),
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),

        StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do math with objects"),
        StackValue::FLOAT { value } => StackValue::FLOAT {
            value: truncate_biguint_to_f64(&a.to_biguint().unwrap()) - value,
        },
    }
}
fn subtract_from_float(a: f64, b: StackValue) -> StackValue {
    match b {
        StackValue::BOOL { value: _ } => panic!("Cannot subtract BOOL from FLOAT"),
        StackValue::BIGINT { value } => StackValue::FLOAT {
            value: a - truncate_biguint_to_f64(&value.to_biguint().unwrap()),
        },
        StackValue::STRING { value: _ } => panic!("Cannot subtract strings"),
        StackValue::ARRAY { value: _ } => panic!("Cannot do math with arrays"),
        StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do math with objects"),
        StackValue::FLOAT { value } => StackValue::FLOAT { value: a - value },
    }
}
pub fn subtract(a: StackValue, b: StackValue) -> StackValue {
    match a {
        StackValue::BOOL { value: _ } => panic!("Cannot do subtraction with BOOL's"),
        StackValue::BIGINT { value } => subtract_from_bigint(value, b),
        StackValue::STRING { value: _ } => panic!("Cannot do subtraction with STRING's"),
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
        StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do math with objects"),
        StackValue::FLOAT { value } => subtract_from_float(value, a),
    }
}
fn multiply_with_bigint(a: BigInt, b: StackValue) -> StackValue {
    match b {
        StackValue::BOOL { value: _ } => panic!("Cannot multiply BOOL with MAGICINT"),
        StackValue::BIGINT { value } => StackValue::BIGINT { value: a * value },
        StackValue::STRING { value: _ } => panic!("Cannot multiply string with MAGICINT"),
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
        StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do math with objects"),
        StackValue::FLOAT { value } => StackValue::FLOAT {
            value: value * truncate_biguint_to_f64(&a.to_biguint().unwrap()),
        },
    }
}
fn multiply_with_float(a: f64, b: StackValue) -> StackValue {
    match b {
        StackValue::BOOL { value: _ } => panic!("Cannot multiply BOOL with FLOAT"),
        StackValue::BIGINT { value } => StackValue::FLOAT {
            value: a * truncate_biguint_to_f64(&value.to_biguint().unwrap()),
        },
        StackValue::STRING { value: _ } => panic!("Cannot multiply string with MAGICINT"),
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
        StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do math with objects"),
        StackValue::FLOAT { value } => StackValue::FLOAT { value: value * a },
    }
}
pub fn multiply(a: StackValue, b: StackValue) -> StackValue {
    match a {
        StackValue::BOOL { value: _ } => panic!("Cannot do multiplication with BOOL's"),
        StackValue::BIGINT { value } => multiply_with_bigint(value, b),
        StackValue::STRING { value: _ } => panic!("Cannot do multiplication with STRING's"),
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
        StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do math with objects"),
        StackValue::FLOAT { value } => multiply_with_float(value, b),
    }
}
fn divide_with_bigint(a: BigInt, b: StackValue) -> StackValue {
    match b {
        StackValue::BOOL { value: _ } => panic!("Cannot divide BOOL with MAGICINT"),
        StackValue::BIGINT { value } => StackValue::BIGINT { value: a / value },
        StackValue::STRING { value: _ } => panic!("Cannot divide STRING with MAGICINT"),
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
        StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do math with objects"),
        StackValue::FLOAT { value } => StackValue::FLOAT {
            value: value / truncate_biguint_to_f64(&a.to_biguint().unwrap()),
        },
    }
}
fn divide_with_float(a: f64, b: StackValue) -> StackValue {
    match b {
        StackValue::BOOL { value: _ } => panic!("Cannot multiply BOOL with FLOAT"),
        StackValue::BIGINT { value } => StackValue::FLOAT {
            value: truncate_biguint_to_f64(&value.to_biguint().unwrap()) / a,
        },
        StackValue::STRING { value: _ } => panic!("Cannot multiply string with MAGICINT"),
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
        StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do math with objects"),
        StackValue::FLOAT { value } => StackValue::FLOAT { value: a / value },
    }
}
pub fn divide(a: StackValue, b: StackValue) -> StackValue {
    match a {
        StackValue::BOOL { value: _ } => panic!("Cannot do division with BOOL's"),
        StackValue::BIGINT { value } => divide_with_bigint(value, b),
        StackValue::STRING { value: _ } => panic!("Cannot do division with STRING's"),
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
        StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do math with objects"),
        StackValue::FLOAT { value } => divide_with_float(value, b),
    }
}
fn get_remainder_with_bigint(a: BigInt, b: StackValue) -> StackValue {
    match b {
        StackValue::BOOL { value: _ } => panic!("Cannot divide BOOL with MAGICINT"),
        StackValue::BIGINT { value } => StackValue::BIGINT { value: a % value },
        StackValue::STRING { value: _ } => panic!("Cannot divide STRING with MAGICINT"),
        StackValue::ARRAY { value: _ } => panic!("Cannot do math with arrays"),
        StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do math with objects"),
        StackValue::FLOAT { value } => StackValue::FLOAT {
            value: truncate_biguint_to_f64(&a.to_biguint().unwrap()) % value,
        },
    }
}
fn get_remainder_with_float(a: f64, b: StackValue) -> StackValue {
    match b {
        StackValue::BOOL { value: _ } => panic!("Cannot divide BOOL with MAGICINT"),
        StackValue::BIGINT { value } => StackValue::FLOAT {
            value: a % truncate_biguint_to_f64(&value.to_biguint().unwrap()),
        },
        StackValue::STRING { value: _ } => panic!("Cannot divide STRING with MAGICINT"),
        StackValue::ARRAY { value: _ } => panic!("Cannot do math with arrays"),
        StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do math with objects"),
        StackValue::FLOAT { value } => StackValue::FLOAT {
            value: truncate_biguint_to_f64(&a.to_biguint().unwrap()) % value,
        },
    }
}
pub fn remainder(a: StackValue, b: StackValue) -> StackValue {
    match a {
        StackValue::BOOL { value: _ } => panic!("Cannot do division with BOOL's"),
        StackValue::BIGINT { value } => get_remainder_with_bigint(value, b),
        StackValue::STRING { value: _ } => panic!("Cannot do division with STRING's"),
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
        StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do math with objects"),
        StackValue::FLOAT { value } => get_remainder_with_float(value, b),
    }
}
pub fn less_than(a: StackValue, b: StackValue) -> StackValue {
    match a {
        StackValue::BOOL { value: val_a } => match b {
            StackValue::BOOL { value } => StackValue::BOOL {
                value: val_a < value,
            },
            StackValue::BIGINT { value: _ } => panic!("Cannot compare MAGICINT with BOOL"),
            StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
            StackValue::STRING { value: _ } => panic!("Cannot compare MAGICINT with STRING"),
            StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do comparison with objects"),
            StackValue::FLOAT { value: _ } => panic!("Cannot compare BOOL and FLOAT"),
        },
        StackValue::BIGINT { value: val_a } => match b {
            StackValue::BOOL { value: _ } => panic!("Cannot compare MAGICINT with BOOL"),
            StackValue::BIGINT { value } => StackValue::BOOL {
                value: val_a < value,
            },
            StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
            StackValue::STRING { value: _ } => panic!("Cannot compare MAGICINT with STRING"),
            StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do comparison with objects"),
            StackValue::FLOAT { value } => StackValue::BOOL {
                value: truncate_biguint_to_f64(&val_a.to_biguint().unwrap()) < value,
            },
        },
        StackValue::STRING { value: _ } => panic!("Cannot do comparisons  with STRING's"),
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
        StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do comparison with objects"),
        StackValue::FLOAT { value: val_a } => match b {
            StackValue::BOOL { value: _ } => panic!("Cannot do comparison with bool"),
            StackValue::BIGINT { value } => StackValue::BOOL {
                value: val_a < truncate_biguint_to_f64(&value.to_biguint().unwrap()),
            },
            StackValue::FLOAT { value } => StackValue::BOOL {
                value: val_a < value,
            },
            StackValue::STRING { value: _ } => panic!("Cannot do comparisons with STRINGS"),
            StackValue::ARRAY { value: _ } => panic!("Cannot do comparisons with ARRAYS"),
            StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do comparisons with OBJECTS"),
        },
    }
}
pub fn larger_than(a: StackValue, b: StackValue) -> StackValue {
    match a {
        StackValue::BOOL { value: val_a } => match b {
            StackValue::BOOL { value } => StackValue::BOOL {
                value: val_a > value,
            },
            StackValue::BIGINT { value: _ } => panic!("Cannot compare MAGICINT with BOOL"),
            StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
            StackValue::STRING { value: _ } => panic!("Cannot compare MAGICINT with STRING"),
            StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do comparison with objects"),
            StackValue::FLOAT { value: _ } => panic!("Cannot compare BOOL and FLOAT"),
        },
        StackValue::BIGINT { value: val_a } => match b {
            StackValue::BOOL { value: _ } => panic!("Cannot compare MAGICINT with BOOL"),
            StackValue::BIGINT { value } => StackValue::BOOL {
                value: val_a > value,
            },
            StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
            StackValue::STRING { value: _ } => panic!("Cannot compare MAGICINT with STRING"),
            StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do comparison with objects"),
            StackValue::FLOAT { value } => StackValue::BOOL {
                value: truncate_biguint_to_f64(&val_a.to_biguint().unwrap()) > value,
            },
        },
        StackValue::STRING { value: _ } => panic!("Cannot do comparisons  with STRING's"),
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
        StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do comparison with objects"),
        StackValue::FLOAT { value: val_a } => match b {
            StackValue::BOOL { value: _ } => panic!("Cannot do comparison with bool"),
            StackValue::BIGINT { value } => StackValue::BOOL {
                value: val_a > truncate_biguint_to_f64(&value.to_biguint().unwrap()),
            },
            StackValue::FLOAT { value } => StackValue::BOOL {
                value: val_a > value,
            },
            StackValue::STRING { value: _ } => panic!("Cannot do comparisons with STRINGS"),
            StackValue::ARRAY { value: _ } => panic!("Cannot do comparisons with ARRAYS"),
            StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do comparisons with OBJECTS"),
        },
    }
}
pub fn less_or_eq(a: StackValue, b: StackValue) -> StackValue {
    match a {
        StackValue::BOOL { value: val_a } => match b {
            StackValue::BOOL { value } => StackValue::BOOL {
                value: val_a <= value,
            },
            StackValue::BIGINT { value: _ } => panic!("Cannot compare MAGICINT with BOOL"),
            StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
            StackValue::STRING { value: _ } => panic!("Cannot compare MAGICINT with STRING"),
            StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do comparison with objects"),
            StackValue::FLOAT { value: _ } => panic!("Cannot compare BOOL and FLOAT"),
        },
        StackValue::BIGINT { value: val_a } => match b {
            StackValue::BOOL { value: _ } => panic!("Cannot compare MAGICINT with BOOL"),
            StackValue::BIGINT { value } => StackValue::BOOL {
                value: val_a <= value,
            },
            StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
            StackValue::STRING { value: _ } => panic!("Cannot compare MAGICINT with STRING"),
            StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do comparison with objects"),
            StackValue::FLOAT { value } => StackValue::BOOL {
                value: truncate_biguint_to_f64(&val_a.to_biguint().unwrap()) <= value,
            },
        },
        StackValue::STRING { value: _ } => panic!("Cannot do comparisons  with STRING's"),
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
        StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do comparison with objects"),
        StackValue::FLOAT { value: val_a } => match b {
            StackValue::BOOL { value: _ } => panic!("Cannot do comparison with bool"),
            StackValue::BIGINT { value } => StackValue::BOOL {
                value: val_a <= truncate_biguint_to_f64(&value.to_biguint().unwrap()),
            },
            StackValue::FLOAT { value } => StackValue::BOOL {
                value: val_a <= value,
            },
            StackValue::STRING { value: _ } => panic!("Cannot do comparisons with STRINGS"),
            StackValue::ARRAY { value: _ } => panic!("Cannot do comparisons with ARRAYS"),
            StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do comparisons with OBJECTS"),
        },
    }
}
pub fn larger_or_eq(a: StackValue, b: StackValue) -> StackValue {
    match a {
        StackValue::BOOL { value: val_a } => match b {
            StackValue::BOOL { value } => StackValue::BOOL {
                value: val_a >= value,
            },
            StackValue::BIGINT { value: _ } => panic!("Cannot compare MAGICINT with BOOL"),
            StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
            StackValue::STRING { value: _ } => panic!("Cannot compare MAGICINT with STRING"),
            StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do comparison with objects"),
            StackValue::FLOAT { value: _ } => panic!("Cannot compare BOOL and FLOAT"),
        },
        StackValue::BIGINT { value: val_a } => match b {
            StackValue::BOOL { value: _ } => panic!("Cannot compare MAGICINT with BOOL"),
            StackValue::BIGINT { value } => StackValue::BOOL {
                value: val_a >= value,
            },
            StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
            StackValue::STRING { value: _ } => panic!("Cannot compare MAGICINT with STRING"),
            StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do comparison with objects"),
            StackValue::FLOAT { value } => StackValue::BOOL {
                value: truncate_biguint_to_f64(&val_a.to_biguint().unwrap()) >= value,
            },
        },
        StackValue::STRING { value: _ } => panic!("Cannot do comparisons  with STRING's"),
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
        StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do comparison with objects"),
        StackValue::FLOAT { value: val_a } => match b {
            StackValue::BOOL { value: _ } => panic!("Cannot do comparison with bool"),
            StackValue::BIGINT { value } => StackValue::BOOL {
                value: val_a >= truncate_biguint_to_f64(&value.to_biguint().unwrap()),
            },
            StackValue::FLOAT { value } => StackValue::BOOL {
                value: val_a >= value,
            },
            StackValue::STRING { value: _ } => panic!("Cannot do comparisons with STRINGS"),
            StackValue::ARRAY { value: _ } => panic!("Cannot do comparisons with ARRAYS"),
            StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do comparisons with OBJECTS"),
        },
    }
}
pub fn not_eq(a: StackValue, b: StackValue) -> StackValue {
    match a {
        StackValue::BOOL { value: val_a } => match b {
            StackValue::BOOL { value } => StackValue::BOOL {
                value: val_a != value,
            },
            StackValue::BIGINT { value: _ } => panic!("Cannot compare MAGICINT with BOOL"),
            StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
            StackValue::STRING { value: _ } => panic!("Cannot compare MAGICINT with STRING"),
            StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do comparison with objects"),
            StackValue::FLOAT { value: _ } => panic!("Cannot compare FLOAT with BOOL"),
        },
        StackValue::BIGINT { value: val_a } => match b {
            
            StackValue::BOOL { value: _ } => panic!("Cannot compare MAGICINT with BOOL"),
            StackValue::BIGINT { value } => StackValue::BOOL {
                value: val_a != value,
            },
            StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
            StackValue::STRING { value: _ } => panic!("Cannot compare MAGICINT with STRING"),
            StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do comparison with objects"),
            StackValue::FLOAT { value } => {
                if value.trunc() == value {
                    StackValue::BOOL {
                        value: val_a != BigInt::from(value as i64),
                    }
                } else {
                    StackValue::BOOL { value: true }
                }
            },
        },
        StackValue::STRING { value: val_a } => match b {
            StackValue::BOOL { value: _ } => panic!("Cannot compare STRING with BOOL"),
            StackValue::BIGINT { value: _ } => panic!("Cannot compare STRING with BIGINT"),
            StackValue::STRING { value } => StackValue::BOOL {
                value: val_a != value,
            },
            StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
            StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do comparison with objects"),
            StackValue::FLOAT { value: _ } => panic!("Cannot compare STRING with FLOAT"),
        },
        StackValue::ARRAY { value: _ } => panic!("Cannot do addition with arrays"),
        StackValue::OBJECT { name: _, value: _ } => panic!("Cannot do comparison with objects"),
        StackValue::FLOAT { value: value_a } => match b {
            StackValue::BOOL { value: _ } => panic!("Cannot compare FLOAT with BOOL"),
            StackValue::BIGINT { value } => {
                if value_a.trunc() == value_a {
                    StackValue::BOOL {
                        value: value != BigInt::from(value_a as i64),
                    }
                } else {
                    StackValue::BOOL { value: false }
                }
            },
            StackValue::FLOAT { value: _ } => StackValue::BOOL {
                value: value_a != value_a,
            },
            StackValue::STRING { value: _ } => panic!("Cannot compare FLOAT with STRING"),
            StackValue::ARRAY { value: _ } => panic!("Cannot compare FLOAT with ARRAY"),
            StackValue::OBJECT { name: _, value: _ } => panic!("Cannot compare FLOAT with OBJECT"),
        },
    }
}
pub fn eq(a: StackValue, b: StackValue) -> StackValue {
    match not_eq(a, b) {
        StackValue::BOOL { value } => StackValue::BOOL { value: !value },
        _ => panic!()
    }
}

pub fn and(a: StackValue, b: StackValue) -> StackValue {
    if (a == StackValue::BOOL { value: true } && b == StackValue::BOOL { value: true }) {
        StackValue::BOOL { value: true }
    } else {
        StackValue::BOOL { value: false }
    }
}

pub fn or(a: StackValue, b: StackValue) -> StackValue {
    if (a == StackValue::BOOL { value: true } || b == StackValue::BOOL { value: true }) {
        StackValue::BOOL { value: true }
    } else {
        StackValue::BOOL { value: false }
    }
}

pub fn xor(a: StackValue, b: StackValue) -> StackValue {
    if (a == StackValue::BOOL { value: true }) {
        if b == (StackValue::BOOL { value: false }) {
            StackValue::BOOL { value: true }
        } else {
            StackValue::BOOL { value: false }
        }
    } else {
        if b == (StackValue::BOOL { value: false }) {
            StackValue::BOOL { value: false }
        } else {
            StackValue::BOOL { value: true }
        }
    }
}
