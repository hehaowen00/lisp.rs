use crate::tokens::*;

pub fn parse(expr: &Vec<char>) -> Result<LispToken, LispError> {
    let mut idx = 0;
    parse_rd(&expr, &mut idx)
}

fn parse_rd(expr: &Vec<char>, idx: &mut usize) -> Result<LispToken, LispError> {
    loop {
        if *idx >= expr.len() {
            return Err(LispError::EndOfSequence);
        }

        let ch = expr[*idx];
        let ahead = if *idx + 1 >= expr.len() {
            ' '
        } else {
            expr[*idx + 1]
        };

        if ch.is_alphabetic() || ch == '\'' || ch == '#' {
            return symbol(&expr, idx);
        } else if ch.is_numeric() || (ch == '-' && ahead.is_numeric()) {
            return number(&expr, idx);
        } else if ch == '"' {
            return string(&expr, idx);
        } else if is_special(ch) {
            return special(&expr, idx);
        } else if ch == '(' {
            return list(&expr, idx);
        } else if !is_bracket(ch) {
            *idx = *idx + 1;
        }
    }
}

fn number(expr: &Vec<char>, idx: &mut usize) ->  Result<LispToken, LispError> {
    let mut s = expr[*idx].to_string();
    let mut decimal_set = false;
    *idx = *idx + 1;

    loop {
        if *idx >= expr.len() {
            return Err(LispError::EndOfSequence);
        }
    
        let ch = expr[*idx];

        if ch.is_numeric() {
            s.push(ch);
        } else if ch == '.' && !decimal_set {
            s.push(ch);
            decimal_set = true;
        } else if is_delimiter(ch) {
            *idx = *idx - 1;
            break;
        } else if !is_bracket(ch) {
            return Err(LispError::UnexpectedChar(ch, *idx));
        }

        *idx = *idx + 1;
    }

    Ok(LispToken::Num(s))
}

fn symbol(expr: &Vec<char>, idx: &mut usize) ->  Result<LispToken, LispError> {
    let mut s = String::new();

    loop {
        if *idx >= expr.len() {
            return Err(LispError::EndOfSequence);
        }

        let ch = expr[*idx];

        if ch.is_alphanumeric() || ch == '-' || ch == '#' {
            s.push(ch);
        } else if is_delimiter(ch) {
            *idx = *idx - 1;
            break;
        } else {
            return Err(LispError::UnexpectedChar(ch, *idx));
        }

        *idx = *idx + 1;
    }

    Ok(LispToken::Sym(s))
}

fn special(expr: &Vec<char>, idx: &mut usize) -> Result<LispToken, LispError> {
    let mut s = String::new();
    let mut not_set = true;

    loop {
        if *idx >= expr.len() {
            return Err(LispError::EndOfSequence);
        }

        let ch = expr[*idx];

        if not_set && is_special(ch) {
            s.push(ch);
            not_set = false;
        } else if is_delimiter(ch) {
            *idx = *idx - 1;
            return Ok(LispToken::Sym(s));
        } else {
            return Err(LispError::UnexpectedChar(ch, *idx));
        }

        *idx = *idx + 1;
    }
}

fn string(expr: &Vec<char>, idx: &mut usize) -> Result<LispToken, LispError> {
    let mut s = expr[*idx].to_string();
    
    loop {
        *idx = *idx + 1;
        if *idx >= expr.len() {
            return Err(LispError::EndOfSequence);
        }
        
        let ch = expr[*idx];
        s.push(ch);

        if ch == '"' {
            return Ok(LispToken::Str(s));
        }
    }
}

fn list(expr: &Vec<char>, idx: &mut usize) ->  Result<LispToken, LispError> {
    let mut lst = Vec::new();
    *idx = *idx + 1;

    loop {
        if *idx >= expr.len() {
            return Err(LispError::Other("expected closing ')".to_string()));
        }

        if expr[*idx] == ')' {
            break;
        }

        match parse_rd(&expr, idx) {
            Ok(token) => lst.push(token),
            Err(err) => {
                return Err(err)
            }
        }

        *idx = *idx + 1;
    }

    Ok(LispToken::List(lst))
}

fn is_bracket(ch: char) -> bool {
    ch == '(' || ch == ')'
}

fn is_special(ch: char) -> bool {
    "+-*/<>".contains(ch)
}

fn is_delimiter(ch: char) -> bool {
    is_bracket(ch) || ch.is_whitespace()
}