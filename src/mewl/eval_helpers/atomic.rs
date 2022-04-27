use crate::mewl::errors::*;
use crate::mewl::eval_helpers::mewcheck::*;
use crate::mewl::types::*;
use std::process::exit;

pub fn extract_atom(atom: &Atom, source: &str) -> Option<f64> {
    match atom {
        Atom::Number(atm) => Some(*atm),
        Atom::Sym(atm) => {
            if is_this_identifier(atm) {
                //self.show_nice_error(atm, "Undefined variable!".to_string());
                undefined_var(atm, source, false);
                //None
                exit(1);
            } else if is_this_assignment(atm) {
                unexpected_assignment(atm, source, false);
                //None
                //self.show_nice_error(atm, "Unexpected assignment!".to_string());
                exit(1);
            } else {
                unknown_atom(atm, source, false);
                //None
                //self.show_nice_error(atm, "Unexpected symbol!".to_string());
                exit(1);
            }
            //return None;
        }
    }
}

pub fn convert_from_mewnum(lexeme: &str) -> f64 {
    //let lexeme = &token.lexeme;

    if lexeme.contains('.') {
        let raw_mews: Vec<&str> = lexeme.trim().split('.').collect();
        if raw_mews.len() > 2 {
            println!("MULTIPLE `.` => {:?}", lexeme);
            exit(1);
        }
        let first_part = raw_mews[0];
        let sec_part = raw_mews[1];
        //println!("{:?}")
        let output = format!(
            "{}.{}",
            (first_part.len() as f64 / 3.0),
            (sec_part.len() as f64 / 3.0)
        );

        let x = match output.parse::<f64>() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Failed to parse this expresssion -> {} as float", lexeme);
                exit(1);
            }
        };
        return x;
    }
    lexeme.len() as f64 / 3.0
}
