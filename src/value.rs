// Represents an event or state value

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    StringValue(String),
    IntValue(i64),
    FloatValue(f64),
    BooleanValue(bool),
    ArrayValue(Vec<Value>),
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::StringValue(value) => value.clone(),
            _ => panic!("Value is not a string")
        }
    }
}