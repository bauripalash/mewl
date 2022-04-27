use crate::mewl::types::*;

pub fn is_this_mewnum(token: &MewToken) -> bool {
    let mut token_lexeme = token.lexeme.chars();
    if token.lexeme.contains('.') {
        let raw_mews: Vec<&str> = token.lexeme.split('.').collect();
        if raw_mews.len() > 2 {
            println!("Number contains multiple `.`s => {}", token.lexeme);
            return false;
        }

        let first_part = raw_mews[0];
        let sec_part = raw_mews[1];

        return is_this_mewnum(&MewToken {
            lexeme: first_part.to_string(),
            position: (0, (0, 0)),
        }) && is_this_mewnum(&MewToken {
            lexeme: sec_part.to_string(),
            position: (0, (0, 0)),
        });
    } else {
        if token_lexeme.as_str().len() < 3 {
            return false;
        }

        while !token_lexeme.as_str().is_empty() {
            if token_lexeme.next() != Some('m') {
                return false;
            }
            if token_lexeme.next() != Some('e') {
                return false;
            }

            if token_lexeme.next() != Some('w') {
                return false;
            }
        }
    }

    true
}

#[allow(dead_code)]
pub fn is_this_assignment(token: &MewToken) -> bool {
    let mut token_lexeme = token.lexeme.chars();
    if token_lexeme.as_str().len() < 4 {
        return false;
    }

    if token_lexeme.next() != Some('=') {
        return false;
    }

    while !token_lexeme.as_str().is_empty() {
        if token_lexeme.next() != Some('m') {
            return false;
        }
        if token_lexeme.next() != Some('e') {
            return false;
        }

        if token_lexeme.next() != Some('w') {
            return false;
        }
    }

    true
}

pub fn is_this_identifier(token: &MewToken) -> bool {
    //println!("IS_ID=> {:?}" , token);
    let mut token_lexeme = token.lexeme.chars();
    //let mut result = false;
    if token_lexeme.as_str().len() < 4 {
        return false;
    }
    if token_lexeme.next() != Some('~') {
        return false;
    }

    while !token_lexeme.as_str().is_empty() {
        if token_lexeme.next() != Some('m') {
            return false;
        }
        if token_lexeme.next() != Some('e') {
            return false;
        }

        if token_lexeme.next() != Some('w') {
            return false;
        }
    }

    true
}
