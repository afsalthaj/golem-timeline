// Represents an event or state value
pub enum Value {
    StringValue(String),
    IntValue(i64),
    FloatValue(f64),
    BooleanValue(bool)
}

impl Value {
    pub fn to_string(self) -> String {
        match self {
            Value::StringValue(value) => value,
            _ => panic!("Value is not a string")
        }
    }
}