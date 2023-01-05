pub struct Error<'a> {
    details: &'a str 
}

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

pub fn parse_json<'a>(data: &str) -> Result<JSONObject<'a>, Error>{
    match lex_json(data) {
        Ok(vec) => {
            println!("{:?}", vec);
        }
        Err(e) => {
            println!("{}", e.details);
        }
    }
    Ok(JSONObject::Number(1))
}

fn lex_json(data: &str) -> Result<std::vec::Vec<String>, Error> {
    const JSON_SYNTAX: &str = "[{,:}]";

    let mut vec: Vec<String> = std::vec::Vec::new();
    let mut is_string = false;
    let mut is_numeric = false;

    for c in data.chars() {
        if c == ' ' || c == '\n' {
            is_numeric = false;
        } else if c == '\"' {
            if is_string {
                is_string = false;
            } else {
                is_string = true;
                vec.push(String::new());
            }
        } else if is_string {
            vec.last_mut().unwrap().push(c);
        } else if JSON_SYNTAX.contains(c) {
            vec.push(String::from(c));
        } else if is_numeric{

        }
        else if c.is_numeric() {
            vec.push(String::from(c));
            is_numeric = true;
        } else {
            return Err(Error {details: "Unexpected character found"});
        }
    }

    Ok(vec)
}
