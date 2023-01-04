pub struct Error {
    details: String
}

pub enum JSONObject<'a> {
    Bool(bool),
    Number(i32),
    String(&'a str),
    Array(&'a JSONObject<'a>),
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

pub fn parse_json<'a, R>(data: &str) -> Result<JSONObject<'a>, Error>{
    Ok(JSONObject::Number(1))
}
