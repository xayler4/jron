use crate::error::TokenIgnoredError;
use crate::error::TokenNotFoundError;
use crate::error::TokenUnexpectedError;
use std::vec::Vec;

#[derive(Debug)]
pub enum Token<'a>{
    CurlyOpen,
    CurlyClose,
    SquareOpen,
    SquareClose,
    DoubleQuotes,
    Colon,
    Comma,
    Numeric(f64),
    String(&'a[char]),
    Boolean(bool),
    Null
}

pub fn tokenize_json(data: &Vec<char>) -> Result<Vec<Token>, TokenUnexpectedError>{
    let mut vec = Vec::new();
    let mut iter = data.iter().enumerate();
    while let Some((i, c)) = iter.next() {
        if let Err(_) = tokenize_ignore(&c) {
            
        }
        else if let Ok(token) = tokenize_numeric(&data[i..]) {
            vec.push(token.0);
            iter.advance_by(token.1).unwrap();
        }
        else if let Ok(token) = tokenize_string(&data[i..]) {
            {
                let Token::String(t) = token else { todo!() };

                iter.advance_by(t.len() + 2).unwrap();
            }
            vec.push(token);
        }
        else if let Ok(token) = tokenize_boolean(&data[i..]) {
            {
                let Token::Boolean(t) = token else { todo!() };
                if t {
                    iter.advance_by("true".len()).unwrap();
                }
                else {
                    iter.advance_by("false".len()).unwrap();
                }
            }
            vec.push(token);
        }
        else if let Ok(token) = tokenize_null(&data[i..]) {
            vec.push(token);
            iter.advance_by("null".len()).unwrap();
        }
        else if let Ok(token) = tokenize_syntax(&c) {
            vec.push(token);
        }
        else {
            return Err(TokenUnexpectedError{unexpected_token: *c});
        }
    }

    Ok(vec)
}

fn tokenize_numeric(data: &[char]) -> Result<(Token, usize), TokenNotFoundError>{
    let mut number_len = 0;
    let mut integer_len = 0;

    for c in data.iter(){
        if c.is_ascii_digit() {
            number_len+=1;
        }
        else if *c == '.' {
            integer_len = number_len;
        }
        else if number_len == 0 {
            return Err(TokenNotFoundError{});
        }
        else {
            let mut value: f64 = 0.0;
            for (i, ch) in data[0..=integer_len].iter().enumerate() {
                value += ((*ch as u32 - 0x30) as usize * (integer_len - i)) as f64;
            }
            if integer_len + 1 < number_len {
                for (i, ch) in data[integer_len+1..=number_len].iter().enumerate() {
                    value += ((*ch as u32 - 0x30) as usize / 10^(i)) as f64;
                }
            }
            return Ok((Token::Numeric(value), number_len));
        }
    }
    return Err(TokenNotFoundError{});
}

fn tokenize_boolean(data: &[char]) -> Result<Token, TokenNotFoundError>{
    const TRUE_TOKEN: [char; 4] = ['t', 'r', 'u', 'e'];
    const FALSE_TOKEN: [char; 5] = ['f', 'a', 'l', 's', 'e'];

    for (i, c) in TRUE_TOKEN.iter().enumerate()  {
        if data[i] != *c {
            break;
        } 
        else if i == TRUE_TOKEN.len() - 1 {
            return Ok(Token::Boolean(true));
        }
    }

    for (i, c) in FALSE_TOKEN.iter().enumerate()  {
        if data[i] != *c {
            return Err(TokenNotFoundError{});
        } 
    }

    Ok(Token::Boolean(false))
}

fn tokenize_null(data: &[char]) -> Result<Token, TokenNotFoundError>{
    const NULL_TOKEN: [char; 4] = ['n', 'u', 'l', 'l'];

    for (i, c) in NULL_TOKEN.iter().enumerate()  {
        if data[i] != *c {
            return Err(TokenNotFoundError{});
        } 
    }

    Ok(Token::Null)
}

fn tokenize_string(data: &[char]) -> Result<Token, TokenNotFoundError>{
    let mut iter = data.iter();
    if *iter.next().unwrap() != '\"' {
        return Err(TokenNotFoundError{});
    }
    for (i, c) in iter.enumerate() {
        if *c == '\"' {
            return Ok(Token::String(&data[1..=i]));
        }
    }
    
    Ok(Token::String(data))
}

fn tokenize_syntax(c: &char) ->Result<Token, TokenNotFoundError> {
    match *c {
        '{' => {
            return Ok(Token::CurlyOpen);
        }
        '}' => {
            return Ok(Token::CurlyClose);
        }
        '[' => {
            return Ok(Token::SquareOpen);
        }
        ']' => {
            return Ok(Token::SquareClose);
        }
        ':' => {
            return Ok(Token::Colon);
        }
        ',' => {
            return Ok(Token::Comma);
        }
        _ => {
            return Err(TokenNotFoundError{})
        }

    }

}

fn tokenize_ignore(c: &char) -> Result<&char, TokenIgnoredError>{
    const IGNORED_TOKENS: &str = " \n\r\t";
    if IGNORED_TOKENS.contains(*c) {
        return Err(TokenIgnoredError{});
    }
    Ok(c)
}
