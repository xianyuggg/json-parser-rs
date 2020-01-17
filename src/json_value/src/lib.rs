use std::fmt::{Display, Formatter};
use hashbrown::HashMap;

#[derive(Debug, Clone)]
pub enum JsonValue{
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
    Number(String),
    True,
    False,
    Null,
    Error
}


impl Display for JsonValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            JsonValue::Object(map) => {
                write!(f, "{{\n")?;
                map.iter().for_each(|(key,val)| {
                    write!(f, "{} : {}\n", key, val);
                });
                write!(f, "}} ")
            },
            JsonValue::String(val) => {
                write!(f, "{}", val)
            }
            JsonValue::False => {
                write!(f, "false")
            }
            JsonValue::True => {
                write!(f, "true")
            }
            JsonValue::Null => {
                write!(f, "null")
            }
            JsonValue::Array(arr) => {
                write!(f, "[")?;
                arr.iter().for_each(|val| {
                    write!(f, " {} ", val);
                });
                write!(f, "]")
            }
            JsonValue::Number(string) => {
                write!(f, "{}", string)
            }
            _ => {
                unimplemented!()
            }
        }
    }
}