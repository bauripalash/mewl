use crate::mewl::types::*;

fn is_this_mewnum_string(lexeme: &str) -> bool {
    let temp_token = MewToken {
        lexeme: lexeme.to_string(),
        position: (0, (0, 0)),
    };
    is_this_mewnum(&temp_token)
}

pub fn is_this_mewnum(token: &MewToken) -> bool {
    let mut token_lexeme = token.lexeme.chars();
    if token.lexeme.contains('.') {
        let mut raw_mews: Vec<&str> = token.lexeme.split('.').collect();
        if raw_mews.len() < 2 {
            return false;
        }
        if raw_mews[0].is_empty() {
            raw_mews.drain(..1);
        }

        for rm in &raw_mews {
            if !is_this_mewnum_string(rm) {
                return false;
            }
        }
        return true;
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
