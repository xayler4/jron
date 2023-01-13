use crate::error::ParseError;
use crate::tokenizer::tokenize_json;

pub enum JSONObject<'a> {
    Bool(bool),
    Number(i32),
    String(&'a str),
    Array(std::vec::Vec<JSONObject<'a>>),
    Object(std::collections::HashMap<&'a str, JSONObject<'a>>),
    Null
}

impl<'a> std::fmt::Display for JSONObject<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JSONObject::Bool(v) => {
                return write!(f, "{}", v as &bool);
            }
            JSONObject::Number(v) => {
                return write!(f, "{}", v as &i32);
            }
            JSONObject::String(v) => {
                return write!(f, "{}", v as &str);
            }
            JSONObject::Null => {
                return write!(f, "Null");
            }
            JSONObject::Object(_) => panic!("Can't print JSONObject::Object"),
            JSONObject::Array(_) => panic!("Can't print JSONObject::Array")
        }
    }
}

pub fn parse_json<'a>(data: &str) -> Result<JSONObject<'a>, ParseError>{
    let mut vec: Vec<_> = data.chars().collect(); 

    match tokenize_json(&vec) {
        Ok(vec) => {
            println!("{:?}", vec);
        }
        Err(e) => {
            panic!("{}", e.unexpected_token);
        }
    }

    Ok(JSONObject::Number(1))
}
