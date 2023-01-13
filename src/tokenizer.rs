use crate::error::TokenIgnoredError;
use crate::error::TokenNotFoundError;
use crate::error::TokenUnexpectedError;
use std::vec::Vec;
use std::slice::from_ref;

pub fn tokenize_json(data: &Vec<char>) -> Result<Vec<&[char]>, TokenUnexpectedError>{
    let mut vec = Vec::new();
    let mut iter = data.iter().enumerate();
    while let Some((i, c)) = iter.next() {
        match token_ignore(&c) {
            Ok(_) => (),
            Err(_) => {
                continue;
            }
        }
        match tokenize_bool(&data[i..]) {
            Ok(slice) => {
                vec.push(slice);
                iter.nth(slice.len()+i);
                continue;
            }
            Err(_) => ()
        }
        match tokenize_string(&data[i..]) {
            Ok(slice) => {
                vec.push(slice);
                iter.nth(slice.len()+i);
                continue;
            }
            Err(_) => ()
        }
        match tokenize_syntax(c) {
            Ok(_) => {
                vec.push(from_ref(c));
                continue;
            }
            Err(_) => ()
        }
        return Err(TokenUnexpectedError{unexpected_token: *c});
    }

    Ok(vec)
}

fn tokenize_bool(data: &[char]) -> Result<&[char], TokenNotFoundError>{
    const TRUE_TOKEN: [char; 4] = ['t', 'r', 'u', 'e'];
    const FALSE_TOKEN: [char; 5] = ['f', 'a', 'l', 's', 'e'];

    for (i, c) in TRUE_TOKEN.iter().enumerate()  {
        if data[i] != *c {
            break;
        } 
        else if i == FALSE_TOKEN.len() {
            return Ok(&data[0..=TRUE_TOKEN.len()]);
        }
    }

    for (i, c) in FALSE_TOKEN.iter().enumerate()  {
        if data[i] != *c {
            return Err(TokenNotFoundError{});
        } 
    }

    Ok(&data[0..=FALSE_TOKEN.len()])
}


fn tokenize_string(data: &[char]) -> Result<&[char], TokenNotFoundError>{
    let mut iter = data.iter();
    if *iter.next().unwrap() != '\"' {
        return Err(TokenNotFoundError{});
    }
    for (i, c) in iter.enumerate() {
        if *c == '\"' {
            return Ok(&data[0..=i+1]);
        }
    }
    
    Ok(data)
}

fn tokenize_syntax(c: &char) ->Result<&char, TokenNotFoundError> {
    const SYNTAX_TOKENS: &str = "{[\",:]}";
    if SYNTAX_TOKENS.contains(*c) {
        return Ok(c);
    }

    Err(TokenNotFoundError{})
}

fn token_ignore(c: &char) -> Result<&char, TokenIgnoredError>{
    const IGNORED_TOKENS: &str = " \n\r";
    if IGNORED_TOKENS.contains(*c) {
        return Err(TokenIgnoredError{});
    }
    Ok(c)
}
