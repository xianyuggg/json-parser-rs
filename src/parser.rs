use common::SYNTAX_ERROR;
use common::abc;
use common::USIZEWrapper;
use std::io::{Error, ErrorKind};
use json_value::JsonValue;
use std::ops::Deref;
use std::borrow::BorrowMut;



// whitespace has been trailed
pub fn parse_json_entry(bytes: &[u8], idx: &mut USIZEWrapper) -> Result<JsonValue, Error>{

    let obj = match bytes[**idx] as char {
        '{' => {

            abc!("Start parsing a object!");
            let mut top_object = JsonValue::Object(Default::default());
            parse_inside_object(&mut top_object, bytes, idx.go_ahead(bytes))?;
            idx.trim_whitespace(bytes);
            match bytes[**idx] as char {
                '}' => {
                    abc!("Parsing complete!");
                    top_object
                }
                _ => {
                    return SYNTAX_ERROR!("SYNTAX ERR!");
                }
            }
        }
        '[' => {
            let mut vec = vec![];
            parse_array(bytes, idx.go_ahead(bytes), vec.borrow_mut());
            idx.trim_whitespace(bytes);
            let top_array = JsonValue::Array(vec);
            match bytes[**idx] as char{
                ']' => {
                    abc!("Parsing complete!");
                    top_array
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
    return Ok(obj);
}


fn parse_inside_object(parent: & mut JsonValue, bytes: &[u8], idx: &mut USIZEWrapper) -> Result<(), Error>  {
    idx.trim_whitespace(bytes);
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
                match bytes[**idx] as char {
                    '"' => {
                        abc!("Start parsing a string");
                        let res = parse_string(bytes, idx.go_ahead(bytes))?;
                        if let JsonValue::String(key) = res{
                            idx.trim_whitespace(bytes);
                            status = ParseObjectStatus::ParsingColon(key);
                        }
                    }
                    _ => {
                        return SYNTAX_ERROR!("START-PARSE-STRING-ERROR!");
                    }
                }
            },
            ParseObjectStatus::ParsingColon(key) => {
                match bytes[**idx] as char {
                    ':' => {
                        abc!("Start parsing a colon");
                        idx.go_ahead(bytes).trim_whitespace(bytes);
                        status = ParseObjectStatus::ParsingValue(key);
                    }
                    _ => {
                        return SYNTAX_ERROR!("COLON-PARSE-ERROR!");
                    }
                }
            },
            ParseObjectStatus::ParsingValue(key) => {
                let res = parse_value(bytes, idx)?;
                if let JsonValue::Object(map) = parent {
                    map.insert(key, res);
                }
                idx.trim_whitespace(bytes);
                // idx = idx.go_ahead(bytes);  ------ "", no need to go_ahead
                status = ParseObjectStatus::ParsingComma;
            },
            ParseObjectStatus::ParsingComma => {

                match bytes[**idx] as char {
                    ',' => {
                        idx.go_ahead(bytes).trim_whitespace(bytes);
                        status = ParseObjectStatus::ParsingString;
                    }
                    '}' => {
                        abc!("Detect }}");
                        idx.go_ahead(bytes);
                        return Ok(());
                    } // It will be only useful when encounter } and } is not the end of the file
                    _ => {
                        return SYNTAX_ERROR!("COMMA-PARSE-ERROR!");
                    }
                }
            },
        }
    }
    Ok(())
}

fn parse_value(bytes: &[u8], idx: &mut USIZEWrapper) -> Result<JsonValue, Error> {
    // white space has been trailed in Parsing colon
    return match bytes[**idx] as char {
        '"' => {
            let res = parse_string(bytes, idx.go_ahead(bytes))?;
            Ok(res)
        }
        '[' => {
            let mut vec = vec![];
            parse_array(bytes, idx.go_ahead(bytes), &mut vec)?;
            Ok(JsonValue::Array(vec))
        }
        '{' => {
            abc!("Start parsing object of value");
            let mut object = JsonValue::Object(Default::default());
            parse_inside_object(&mut object, bytes, idx.go_ahead(bytes))?;
            Ok(object)

        }
        't' => {
            idx.go_ahead_by_times(bytes, 4);
            Ok(JsonValue::True)
        }
        'f' => {
            idx.go_ahead_by_times(bytes, 5);
            Ok(JsonValue::False)
        }
        'n' => {
            idx.go_ahead_by_times(bytes, 4);
            Ok(JsonValue::Null)
        }
        _ => {
            let res = parse_number(bytes, idx)?;
            Ok(res)
        }
    }
}

fn parse_number(bytes: &[u8], idx: &mut USIZEWrapper) -> Result<JsonValue, Error> {
    abc!("Start parsing number!");
    // TODO : A Naive number parser
    let mut vec = vec![];
    while !idx.is_end(bytes) & ! bytes[**idx].is_ascii_whitespace() & ( bytes[**idx].is_ascii_digit() || bytes[**idx] as char == 'e'){
        match bytes[**idx] as char{
            _ => {
                vec.push(bytes[**idx] as char);
                idx.go_ahead(bytes);
            }
        }
    }
    Ok(JsonValue::Number(vec.iter().collect()))
}

fn parse_array(bytes: &[u8], idx: &mut USIZEWrapper, vec: &mut Vec<JsonValue>) -> Result<(), Error> {
    abc!("Start parsing array!");
    while !idx.is_end(bytes) {
        idx.trim_whitespace(bytes);
        match bytes[**idx] as char{
            '"' => {
                vec.push(parse_string(bytes, idx.go_ahead(bytes))?);
            }
            '{' => {
                abc!("Start parsing a object inside a array");
                let mut obj = JsonValue::Object(Default::default());
                parse_inside_object(&mut obj ,bytes, idx.go_ahead(bytes))?;
                vec.push(obj);
            }
            ',' => {
                idx.go_ahead(bytes);
            }
            ']' => {
                abc!("Vec generate complete!");
                idx.go_ahead(bytes);
                return Ok(());
            }
            _ => {
                return SYNTAX_ERROR!("UNREACHABLE-IN-PARSE-ARRAY")
            }
        }
    }
    SYNTAX_ERROR!("ARRAY-PARSE-ERROR!")
}

fn parse_string(bytes: &[u8], idx: &mut USIZEWrapper) -> Result<JsonValue, Error> {

    let mut vec = vec![];
    while !idx.is_end(bytes){
        match bytes[**idx] as char{
            '"' => {
                abc!("Parsing string complete!");
                idx.go_ahead(bytes);
                return Ok( JsonValue::String(vec.into_iter().collect()));
            }
            _ => {
                vec.push(bytes[**idx] as char);
                idx.go_ahead(bytes);
            }
        }
    }
    SYNTAX_ERROR!("STRING-PARSE-ERROR!")
}




