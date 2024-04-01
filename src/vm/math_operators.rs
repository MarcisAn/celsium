use num::BigInt;

use super::StackValue;

fn add_to_bigint(a: BigInt, b: StackValue) -> StackValue {
    match b {
        StackValue::BOOL { value: _ } => panic!("Cannot add BOOL to INT"),
        StackValue::BIGINT { value } => StackValue::BIGINT { value: a + value },
        StackValue::STRING { value } => StackValue::STRING {
            value: a.to_string() + &value,
        },
        StackValue::ARRAY { value } => panic!("Cannot do addition with arrays"),
    }
}
fn add_to_string(a: String, b: StackValue) -> StackValue {
    match b {
        StackValue::BOOL { value: _ } => panic!("Cannot add BOOL to STRING"),
        StackValue::BIGINT { value } => StackValue::STRING {
            value: a + &value.to_string(),
        },
        StackValue::STRING { value } => StackValue::STRING { value: a + &value },
        StackValue::ARRAY { value } => panic!("Cannot do addition with arrays"),
    }
}

pub fn add(a: StackValue, b: StackValue) -> StackValue {
    match a {
        StackValue::BOOL { value: _ } => panic!("Cannot do addition with bool's"),
        StackValue::BIGINT { value } => add_to_bigint(value, b),
        StackValue::STRING { value } => add_to_string(value, b),
        StackValue::ARRAY { value } => panic!("Cannot do addition with arrays"),
    }
}
fn subtract_from_bigint(a: BigInt, b: StackValue) -> StackValue {
    match b {
        StackValue::BOOL { value: _ } => panic!("Cannot subtract BOOL to INT"),
        StackValue::BIGINT { value } => StackValue::BIGINT { value: a - value },
        StackValue::STRING { value: _ } => panic!("Cannot subtract string from MAGICINT"),
        StackValue::ARRAY { value } => panic!("Cannot do addition with arrays"),
    }
}
pub fn subtract(a: StackValue, b: StackValue) -> StackValue {
    match a {
        StackValue::BOOL { value: _ } => panic!("Cannot do subtraction with BOOL's"),
        StackValue::BIGINT { value } => subtract_from_bigint(value, b),
        StackValue::STRING { value: _ } => panic!("Cannot do subtraction with STRING's"),
        StackValue::ARRAY { value } => panic!("Cannot do addition with arrays"),
    }
}
fn multiply_with_bigint(a: BigInt, b: StackValue) -> StackValue {
    match b {
        StackValue::BOOL { value: _ } => panic!("Cannot multiply BOOL with MAGICINT"),
        StackValue::BIGINT { value } => StackValue::BIGINT { value: a * value },
        StackValue::STRING { value: _ } => panic!("Cannot multiply string with MAGICINT"),
        StackValue::ARRAY { value } => panic!("Cannot do addition with arrays"),
    }
}
pub fn multiply(a: StackValue, b: StackValue) -> StackValue {
    match a {
        StackValue::BOOL { value: _ } => panic!("Cannot do multiplication with BOOL's"),
        StackValue::BIGINT { value } => multiply_with_bigint(value, b),
        StackValue::STRING { value: _ } => panic!("Cannot do multiplication with STRING's"),
        StackValue::ARRAY { value } => panic!("Cannot do addition with arrays"),
    }
}
fn divide_with_bigint(a: BigInt, b: StackValue) -> StackValue {
    match b {
        StackValue::BOOL { value: _ } => panic!("Cannot divide BOOL with MAGICINT"),
        StackValue::BIGINT { value } => StackValue::BIGINT { value: a / value },
        StackValue::STRING { value: _ } => panic!("Cannot divide STRING with MAGICINT"),
        StackValue::ARRAY { value } => panic!("Cannot do addition with arrays"),
    }
}
pub fn divide(a: StackValue, b: StackValue) -> StackValue {
    match a {
        StackValue::BOOL { value: _ } => panic!("Cannot do division with BOOL's"),
        StackValue::BIGINT { value } => divide_with_bigint(value, b),
        StackValue::STRING { value: _ } => panic!("Cannot do division with STRING's"),
        StackValue::ARRAY { value } => panic!("Cannot do addition with arrays"),
    }
}
fn get_remainder_with_bigint(a: BigInt, b: StackValue) -> StackValue {
    match b {
        StackValue::BOOL { value: _ } => panic!("Cannot divide BOOL with MAGICINT"),
        StackValue::BIGINT { value } => StackValue::BIGINT { value: a % value },
        StackValue::STRING { value: _ } => panic!("Cannot divide STRING with MAGICINT"),
        StackValue::ARRAY { value } => panic!("Cannot do addition with arrays"),
    }
}
pub fn remainder(a: StackValue, b: StackValue) -> StackValue {
    match a {
        StackValue::BOOL { value: _ } => panic!("Cannot do division with BOOL's"),
        StackValue::BIGINT { value } => get_remainder_with_bigint(value, b),
        StackValue::STRING { value: _ } => panic!("Cannot do division with STRING's"),
        StackValue::ARRAY { value } => panic!("Cannot do addition with arrays"),
    }
}
pub fn less_than(a: StackValue, b: StackValue) -> StackValue {
    match a {
        StackValue::BOOL { value: val_a } => match b {
            StackValue::BOOL { value } => StackValue::BOOL {
                value: val_a < value,
            },
            StackValue::BIGINT { value } => panic!("Cannot compare MAGICINT with BOOL"),
            StackValue::ARRAY { value } => panic!("Cannot do addition with arrays"),
            StackValue::STRING { value } => panic!("Cannot compare MAGICINT with STRING"),
        },
        StackValue::BIGINT { value: val_a } => match b {
            StackValue::BOOL { value } => panic!("Cannot compare MAGICINT with BOOL"),
            StackValue::BIGINT { value } => StackValue::BOOL {
                value: val_a < value,
            },
            StackValue::ARRAY { value } => panic!("Cannot do addition with arrays"),
            StackValue::STRING { value } => panic!("Cannot compare MAGICINT with STRING"),
        },
        StackValue::STRING { value: _ } => panic!("Cannot do comparisons  with STRING's"),
        StackValue::ARRAY { value } => panic!("Cannot do addition with arrays"),
    }
}
pub fn larger_than(a: StackValue, b: StackValue) -> StackValue {
    match a {
        StackValue::BOOL { value: val_a } => match b {
            StackValue::BOOL { value } => StackValue::BOOL {
                value: val_a > value,
            },
            StackValue::BIGINT { value } => panic!("Cannot compare MAGICINT with BOOL"),
            StackValue::ARRAY { value } => panic!("Cannot do addition with arrays"),
            StackValue::STRING { value } => panic!("Cannot compare MAGICINT with STRING"),
        },
        StackValue::BIGINT { value: val_a } => match b {
            StackValue::BOOL { value } => panic!("Cannot compare MAGICINT with BOOL"),
            StackValue::BIGINT { value } => StackValue::BOOL {
                value: val_a > value,
            },
            StackValue::STRING { value } => panic!("Cannot compare MAGICINT with STRING"),
            StackValue::ARRAY { value } => panic!("Cannot do addition with arrays"),
        },
        StackValue::STRING { value: _ } => panic!("Cannot do comparisons  with STRING's"),
        StackValue::ARRAY { value } => panic!("Cannot do addition with arrays"),
    }
}
pub fn less_or_eq(a: StackValue, b: StackValue) -> StackValue {
    match a {
        StackValue::BOOL { value: val_a } => match b {
            StackValue::BOOL { value } => StackValue::BOOL {
                value: val_a <= value,
            },
            StackValue::BIGINT { value } => panic!("Cannot compare MAGICINT with BOOL"),
            StackValue::ARRAY { value } => panic!("Cannot do addition with arrays"),
            StackValue::STRING { value } => panic!("Cannot compare MAGICINT with STRING"),
        },
        StackValue::BIGINT { value: val_a } => match b {
            StackValue::BOOL { value } => panic!("Cannot compare MAGICINT with BOOL"),
            StackValue::BIGINT { value } => StackValue::BOOL {
                value: val_a <= value,
            },
            StackValue::ARRAY { value } => panic!("Cannot do addition with arrays"),
            StackValue::STRING { value } => panic!("Cannot compare MAGICINT with STRING"),
        },
        StackValue::STRING { value: _ } => panic!("Cannot do comparisons  with STRING's"),
        StackValue::ARRAY { value } => panic!("Cannot do addition with arrays"),
    }
}
pub fn larger_or_eq(a: StackValue, b: StackValue) -> StackValue {
    match a {
        StackValue::BOOL { value: val_a } => match b {
            StackValue::BOOL { value } => StackValue::BOOL {
                value: val_a >= value,
            },
            StackValue::BIGINT { value } => panic!("Cannot compare MAGICINT with BOOL"),
            StackValue::ARRAY { value } => panic!("Cannot do addition with arrays"),
            StackValue::STRING { value } => panic!("Cannot compare MAGICINT with STRING"),
        },
        StackValue::BIGINT { value: val_a } => match b {
            StackValue::BOOL { value } => panic!("Cannot compare MAGICINT with BOOL"),
            StackValue::BIGINT { value } => StackValue::BOOL {
                value: val_a >= value,
            },
            StackValue::ARRAY { value } => panic!("Cannot do addition with arrays"),
            StackValue::STRING { value } => panic!("Cannot compare MAGICINT with STRING"),
        },
        StackValue::STRING { value: _ } => panic!("Cannot do comparisons  with STRING's"),
        StackValue::ARRAY { value } => panic!("Cannot do addition with arrays"),
    }
}
pub fn not_eq(a: StackValue, b: StackValue) -> StackValue {
    match a {
        StackValue::BOOL { value: val_a } => match b {
            StackValue::BOOL { value } => StackValue::BOOL {
                value: val_a != value,
            },
            StackValue::BIGINT { value } => panic!("Cannot compare MAGICINT with BOOL"),
            StackValue::ARRAY { value } => panic!("Cannot do addition with arrays"),
            StackValue::STRING { value } => panic!("Cannot compare MAGICINT with STRING"),
        },
        StackValue::BIGINT { value: val_a } => match b {
            StackValue::BOOL { value } => panic!("Cannot compare MAGICINT with BOOL"),
            StackValue::BIGINT { value } => StackValue::BOOL {
                value: val_a != value,
            },
            StackValue::ARRAY { value } => panic!("Cannot do addition with arrays"),
            StackValue::STRING { value } => panic!("Cannot compare MAGICINT with STRING"),
        },
        StackValue::STRING { value: val_a } => match b {
            StackValue::BOOL { value } => panic!("Cannot compare STRING with BOOL"),
            StackValue::BIGINT { value } => panic!("Cannot compare STRING with BIGINT"),
            StackValue::STRING { value } => StackValue::BOOL {
                value: val_a != value,
            },
            StackValue::ARRAY { value } => panic!("Cannot do addition with arrays"),
        },
        StackValue::ARRAY { value } => panic!("Cannot do addition with arrays"),
    }
}
pub fn eq(a: StackValue, b: StackValue) -> StackValue {
    match a {
        StackValue::BOOL { value: val_a } => match b {
            StackValue::BOOL { value } => StackValue::BOOL {
                value: val_a == value,
            },
            StackValue::BIGINT { value } => panic!("Cannot compare MAGICINT with BOOL"),
            StackValue::ARRAY { value } => panic!("Cannot do addition with arrays"),
            StackValue::STRING { value } => panic!("Cannot compare MAGICINT with STRING"),
        },
        StackValue::BIGINT { value: val_a } => match b {
            StackValue::BOOL { value } => panic!("Cannot compare MAGICINT with BOOL"),
            StackValue::BIGINT { value } => StackValue::BOOL {
                value: val_a == value,
            },
            StackValue::ARRAY { value } => panic!("Cannot do addition with arrays"),
            StackValue::STRING { value } => panic!("Cannot compare MAGICINT with STRING"),
        },
        StackValue::STRING { value: val_a } => match b {
            StackValue::BOOL { value } => panic!("Cannot compare STRING with BOOL"),
            StackValue::BIGINT { value } => panic!("Cannot compare STRING with BIGINT"),
            StackValue::STRING { value } => StackValue::BOOL {
                value: val_a == value,
            },
            StackValue::ARRAY { value } => panic!("Cannot do addition with arrays"),
        },
        StackValue::ARRAY { value } => panic!("Cannot do addition with arrays"),
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
