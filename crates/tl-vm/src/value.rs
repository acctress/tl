use std::ops::{Add, Sub, Mul, Div};
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Debug)]
pub enum Value {
    Num(f64),
    Str(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Num(n) => write!(f, "{}", n),
            Value::Str(s) => write!(f, "{}", s),
        }
    }
}

impl Add for &Value {
    type Output = Value;
    fn add(self, rhs: &Value) -> Value {
        match (self, rhs) {
            (Value::Num(a), Value::Num(b)) => Value::Num(a + b),
            _ => panic!("type error"),
        }
    }
}

impl Sub for &Value {
    type Output = Value;
    fn sub(self, rhs: &Value) -> Value {
        match (self, rhs) {
            (Value::Num(a), Value::Num(b)) => Value::Num(a - b),
            _ => panic!("type error"),
        }
    }
}

impl Mul for &Value {
    type Output = Value;
    fn mul(self, rhs: &Value) -> Value {
        match (self, rhs) {
            (Value::Num(a), Value::Num(b)) => Value::Num(a * b),
            _ => panic!("type error"),
        }
    }
}

impl Div for &Value {
    type Output = Value;
    fn div(self, rhs: &Value) -> Value {
        match (self, rhs) {
            (Value::Num(a), Value::Num(b)) => Value::Num(a / b),
            _ => panic!("type error"),
        }
    }
}