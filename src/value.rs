// Represents an event or state value

use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    StringValue(String),
    IntValue(i64),
    BooleanValue(bool),
    ArrayValue(Vec<Value>),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::StringValue(s1), Value::StringValue(s2)) => s1.cmp(s2),
            (Value::IntValue(i1), Value::IntValue(i2)) => i1.cmp(i2),
            (Value::BooleanValue(b1), Value::BooleanValue(b2)) => b1.cmp(b2),
            (Value::ArrayValue(arr1), Value::ArrayValue(arr2)) => {
                // Compare arrays element by element
                for (v1, v2) in arr1.iter().zip(arr2.iter()) {
                    match v1.cmp(v2) {
                        Ordering::Equal => continue,
                        ord => return ord,
                    }
                }
                // If arrays are equal up to the common length, compare their lengths
                arr1.len().cmp(&arr2.len())
            }
            // Define a total ordering between different types of values
            (Value::StringValue(_), _) => Ordering::Greater,
            (Value::IntValue(_), Value::StringValue(_)) => Ordering::Less,
            (Value::IntValue(_), _) => Ordering::Greater,
            (Value::BooleanValue(_), Value::StringValue(_))
            | (Value::BooleanValue(_), Value::IntValue(_)) => Ordering::Less,
            (Value::BooleanValue(_), _) => Ordering::Greater,
            (Value::ArrayValue(_), _) => Ordering::Less,
        }
    }
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::StringValue(value) => value.clone(),
            _ => panic!("Value is not a string"),
        }
    }
}
