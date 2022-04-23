use crate::mewl::types::*;


pub fn is_this_mewnum(token: &MewToken) -> bool {
    //println!("IS_MEWNUM=> {:?}" , token);
    let mut token_lexeme = token.lexeme.chars();
    //let mut result = false;
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

    true
}

#[allow(dead_code)]
pub fn is_this_assignment(token: &MewToken) -> bool {
    //println!("ID_ASSIGN=> {:?}" , token);
    let mut token_lexeme = token.lexeme.chars();
    //let mut result = false;
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
    //println!("YES_ASSIGN");

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