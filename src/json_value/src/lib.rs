use std::fmt::{Display, Formatter, Error,  Debug};
use hashbrown::HashMap;

#[derive(Debug, Clone)]
pub enum JsonValue{
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
    Number(Number),
    True,
    False,
    Null,
    Error
}

#[derive(Debug, Clone)]
pub enum Number{
    Int(isize),
    Float(f64),
    Expo(f64)
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Number::Int(i) => {
                write!(f, "{}", i)
            },
            Number::Float(t) => {
                println!("{} (Float)", t);
                Ok(())
            },
            Number::Expo(t) => {
                println!("{} (Expo)", t);
                Ok(())
            }
        }

    }
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