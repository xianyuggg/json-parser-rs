use common::SYNTAX_ERROR;
use common::USIZEWrapper;
use std::io::{Error, ErrorKind};
use json_value::JsonValue;



// whitespace has been trailed
pub fn parse_json_entry(bytes: &[u8], mut idx: USIZEWrapper) -> Result<(), Error>{

    let obj = match bytes[idx.into() : usize] as char {
        '{' => {
            println!("Start parsing a object!");
            let mut top_object = JsonValue::Object(Default::default());
            idx = parse_inside_object(&mut top_object, bytes, idx.go_ahead(bytes))?.trim_whitespace(bytes);

            match bytes[idx.into() : usize] as char {
                '}' => {
                    println!("Parsing complete!");
                    top_object
                }
                _ => {
                    return SYNTAX_ERROR!("SYNTAX ERR!");
                }
            }
        }
        _ => {
            return SYNTAX_ERROR!("SYNTAX ERR!");
        }
    };
    println!("\n=================================\nParse result:\n{}", obj);
    return Ok(());
}


fn parse_inside_object(parent: & mut JsonValue, bytes: &[u8], mut idx: USIZEWrapper) -> Result<USIZEWrapper, Error>  {
    let mut idx = idx.trim_whitespace(bytes);
    enum ParseObjectStatus{
        ParsingString,
        ParsingColon(String),
        ParsingValue(String),
        ParsingComma
    }
    let mut status = ParseObjectStatus::ParsingString;

    while !idx.is_end(bytes)  {
        match status {
            ParseObjectStatus::ParsingString => {
                match bytes[idx.into(): usize] as char {
                    '"' => {
                        println!("Start parsing a string");
                        let res = parse_string(bytes, idx.go_ahead(bytes));
                        let key = res.0;
                        idx = res.1?.trim_whitespace(bytes);
                        status = ParseObjectStatus::ParsingColon(key);
                    }
                    _ => {
                        return SYNTAX_ERROR!("START-PARSE-STRING-ERROR!");
                    }
                }
            },
            ParseObjectStatus::ParsingColon(key) => {
                match bytes[idx.into(): usize] as char {
                    ':' => {
                        println!("Start parsing a colon");
                        idx = idx.go_ahead(bytes).trim_whitespace(bytes);
                        status = ParseObjectStatus::ParsingValue(key);
                    }
                    _ => {
                        return SYNTAX_ERROR!("COLON-PARSE-ERROR!");
                    }
                }
            },
            ParseObjectStatus::ParsingValue(key) => {
                let res = parse_value(bytes, idx);
                if let JsonValue::Object(map) = parent {
                    map.insert(key, res.0);
                }
                idx = res.1?.trim_whitespace(bytes);
                // idx = idx.go_ahead(bytes);  ------ "", no need to go_ahead
                status = ParseObjectStatus::ParsingComma;
            },
            ParseObjectStatus::ParsingComma => {
                println!("{}", bytes[idx.into() : usize] as char);
                match bytes[idx.into(): usize] as char {
                    ',' => {
                        idx = idx.go_ahead(bytes).trim_whitespace(bytes);
                        status = ParseObjectStatus::ParsingString;
                    }
                    '}' => {
                        println!("Detect }}");
                        return Ok(idx.go_ahead(bytes));
                    } // It will be only useful when encounter } and } is not the end of the file
                    _ => {
                        return SYNTAX_ERROR!("COMMA-PARSE-ERROR!");
                    }
                }
            },
        }
    }
    Ok(idx)
}

fn parse_value(bytes: &[u8], mut idx: USIZEWrapper) -> (JsonValue, Result<USIZEWrapper, Error>) {
    // white space has been trailed in Parsing colon
    return match bytes[idx.into(): usize] as char {
        '"' => {
            let res = parse_string(bytes, idx.go_ahead(bytes));
            (JsonValue::String(res.0), res.1)
        }
        '[' => {
            let mut vec = vec![];
            let res = parse_array(bytes, idx.go_ahead(bytes), &mut vec);
            (JsonValue::Array(vec), res)
        }
        '{' => {
            println!("Start parsing object of value");
            let mut object = JsonValue::Object(Default::default());
            let res = parse_inside_object(&mut object, bytes, idx.go_ahead(bytes));
            if let Ok(idx) = res {
                (object, Ok(idx))
            } else {
                (JsonValue::Error, SYNTAX_ERROR!("VALUE-OBJECT-PARSE-ERROR!"))
            }
        }
        't' => {
            (JsonValue::True, Ok(idx.go_ahead_by_times(bytes, 4)))
        }
        'f' => {
            (JsonValue::False, Ok(idx.go_ahead_by_times(bytes, 5)))
        }
        'n' => {
            (JsonValue::Null, Ok(idx.go_ahead_by_times(bytes, 4)))
        }
        _ => {
            let res = parse_number(bytes, idx);
            (res.0, Ok(res.1.unwrap().trim_whitespace(bytes)))
        }
    }
}

fn parse_number(bytes: &[u8], mut idx: USIZEWrapper) -> (JsonValue,  Result<USIZEWrapper, Error>) {
    println!("Start parsing number!");
    // TODO : A Naive number parser
    let mut vec = vec![];
    while !idx.is_end(bytes) & ! bytes[idx.into() : usize].is_ascii_whitespace() {

        match bytes[idx.into() : usize] as char{
            _ => {
                vec.push(bytes[idx.into() : usize] as char);
                idx = idx.go_ahead(bytes);
            }
        }
    }
    (JsonValue::Number(vec.iter().collect()), Ok(idx))
}

fn parse_array(bytes: &[u8], mut idx: USIZEWrapper, vec: &mut Vec<JsonValue>) -> Result<USIZEWrapper, Error> {
    println!("Start parsing array!");
    while !idx.is_end(bytes) {
        idx = idx.trim_whitespace(bytes);
        match bytes[idx.into() : usize] as char{
            '"' => {
                let res = parse_string(bytes, idx.go_ahead(bytes));
                vec.push(JsonValue::String(res.0));
                idx = res.1?;
            }
            '{' => {
                println!("Start parsing a object inside a array");
                let mut obj = JsonValue::Object(Default::default());
                idx = parse_inside_object(&mut obj ,bytes, idx.go_ahead(bytes))?;
                vec.push(obj);
            }
            ',' => {
                idx = idx.go_ahead(bytes);
            }
            ']' => {
                println!("Vec generate complete!");
                return Ok(idx.go_ahead(bytes));
            }
            _ => {
                return SYNTAX_ERROR!("UNREACHABLE-IN-PARSE-ARRAY")
            }
        }
    }
    SYNTAX_ERROR!("ARRAY-PARSE-ERROR!")
}

fn parse_string(bytes: &[u8], mut idx: USIZEWrapper) -> (String, Result<USIZEWrapper, Error>) {

    let mut vec = vec![];
    while !idx.is_end(bytes){
        match bytes[idx.into() : usize] as char{
            '"' => {
                println!("Parsing string complete!");
                return (vec.into_iter().collect(), Ok(idx.go_ahead(bytes)));
            }
            _ => {
                vec.push(bytes[idx.into() : usize] as char);
                idx.go_ahead(bytes);
            }
        }
    }
    ("".to_string(), SYNTAX_ERROR!("STRING-PARSE-ERROR!"))
}




