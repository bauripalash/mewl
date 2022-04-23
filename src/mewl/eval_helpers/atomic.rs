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
