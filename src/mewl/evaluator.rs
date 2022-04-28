use crate::mewl::errors::*;
use crate::mewl::eval_helpers::atomic::{convert_from_mewnum, extract_atom};
use crate::mewl::eval_helpers::mewcheck::*;
use crate::mewl::eval_helpers::operations::binary::do_binary_operation;
use crate::mewl::types::*;
use std::collections::HashMap;
use std::io;
use std::process::exit;

pub struct MewlEvaluator {
    pub expression: Expr,
    pub source: String,
    pub symbol_table: HashMap<String, f64>,
}

impl MewlEvaluator {
    pub fn new(expression: Expr, source: String) -> Self {
        Self {
            expression,
            source,
            symbol_table: HashMap::new(),
        }
    }

    pub fn do_eval(&mut self) {
        self.evaluate(&mut self.expression.clone(), &mut self.symbol_table.clone());
    }
    #[allow(unused_assignments)]
    pub fn evaluate(
        &mut self,
        exp: &mut Expr,
        symbol_table: &mut HashMap<String, f64>,
    ) -> (Option<Atom>, Option<Vec<Atom>>) {
        //let p = exp.clone();
        match exp {
            Expr::Atom(atom) => match atom {
                Atom::Number(_) => (Some(atom.to_owned()), None),
                Atom::Sym(atom_symbol) => {
                    if OPERATORS.contains(&atom_symbol.lexeme.as_str()) {
                        (Some(atom.to_owned()), None)
                    } else if is_this_identifier(atom_symbol) {
                        //[^ref-1] //see below
                        //check if the symbol is identifer; basically if mew number starts with a `~` char

                        let var_value = symbol_table.get(&atom_symbol.lexeme); //get value from symbol table

                        if let Some(..) = var_value {
                            (Some(Atom::Number(*var_value.unwrap())), None)
                        //if the id has value assigned to it; create a new Atom with the value
                        } else {
                            undefined_var(atom_symbol, &self.source, true);
                            //self.show_nice_error(atom_symbol, "Undefined variable!".to_string()); //variable has no value; show error
                            //exit(1);
                            (None, None)
                        }
                    } else if is_this_assignment(atom_symbol) {
                        //check if the symbol is assignment; if mew number starts with `=`

                        (Some(atom.to_owned()), None) // return as is; so we can use it later for assignment
                    } else {
                        unknown_atom(atom_symbol, &self.source, false);
                        //self.show_nice_error(
                        //    atom_symbol,
                        //    "Sorry! I dont know what to do with this symbol!".to_string(),
                        //);
                        //exit(1);
                        (None, None)
                    }
                }
            },

            Expr::List(expr_list) => {
                if !expr_list.is_empty() {
                    let mut atom_list: Vec<Atom> = vec![];
                    if let Expr::Atom(Atom::Sym(s)) = &expr_list.clone()[0] {
                        if s.lexeme == *"|>" {
                            if expr_list.len() != 2 {
                                //println!("{:?}", expr_list);
                                //eprintln!("Please provide a identifier to store the input");
                                //exit(1);
                                //
                                no_assignment_symbol_after_stdin(s, &self.source, true);
                            }

                            let identifer = expr_list.drain(1..2).collect::<Vec<Expr>>();
                            let mut identifier_name: String = String::new();
                            let mut input_buffer: String = String::new();

                            match &identifer[0] {
                                Expr::Atom(atm) => match atm {
                                    Atom::Sym(id) => {
                                        if is_this_assignment(id) {
                                            identifier_name = id.lexeme.clone();
                                        } else if is_this_identifier(id) {
                                            let mut x = id.lexeme.chars();
                                            x.next();
                                            let temp_wrong_id = x.collect::<String>();
                                            eprintln!("Did you meant to write ={}", temp_wrong_id);
                                            exit(1);
                                        } else {
                                            //eprintln!()
                                            unknown_atom(id, &self.source, true);
                                        }
                                    }
                                    Atom::Number(_) => {
                                        number_after_stdin(s, &self.source, true);
                                    }
                                },
                                Expr::List(l) => {
                                    //println!("{:?}" , expr_list)
                                    if l.len() != 1 {
                                        //println!("Please provide a single assignment mew symbol for stdin input operation");

                                        //exit(1);
                                        //
                                        multiple_exp_after_stdin(s, &self.source, true);
                                    } else if let Expr::Atom(Atom::Sym(id)) = &l[0] {
                                        identifier_name = id.lexeme.clone();
                                    } else if let Expr::Atom(Atom::Number(_)) = &l[0] {
                                        //eprintln!("I was not expecting a number here!");
                                        //exit(1);

                                        number_after_stdin(s, &self.source, true);
                                    }
                                }
                            }

                            if !identifier_name.is_empty() {
                                match io::stdin().read_line(&mut input_buffer) {
                                    Ok(_) => {}
                                    Err(_) => {
                                        eprintln!("Failed to read the input!");
                                        exit(1);
                                    }
                                }
                            }

                            input_buffer = input_buffer.trim_end().to_owned();
                            let temp_token = MewToken {
                                lexeme: input_buffer.clone(),
                                position: (0, (0, 0)),
                            };
                            let temp_atom: Atom = if is_this_mewnum(&temp_token) {
                                Atom::Number(convert_from_mewnum(&temp_token.lexeme))
                            } else {
                                let t = match input_buffer.parse::<f64>() {
                                    Ok(v) => v,
                                    Err(_) => {
                                        eprintln!("Failed to parse the input; it is not a mewnum or traditional number!");
                                        exit(1);
                                    }
                                };

                                Atom::Number(t)
                            };
                            let temp_id_token = MewToken {
                                lexeme: identifier_name,
                                position: (0, (0, 0)),
                            };
                            self.do_assignment(&temp_id_token, &[temp_atom], symbol_table);
                        } else if s.lexeme == *"||>" {
                            if expr_list.len() != 2 {
                                //println!("{:?}", expr_list);
                                //eprintln!("Please provide a identifier to store the input");
                                //exit(1);
                                no_assignment_symbol_after_stdin(s, &self.source, true);
                            }

                            let identifer = expr_list.drain(1..2).collect::<Vec<Expr>>();
                            let mut identifier_name: String = String::new();
                            let mut input_buffer: String = String::new();

                            match &identifer[0] {
                                Expr::Atom(atm) => match atm {
                                    Atom::Sym(id) => {
                                        if is_this_assignment(id) {
                                            identifier_name = id.lexeme.clone();
                                        } else if is_this_identifier(id) {
                                            let mut x = id.lexeme.chars();
                                            x.next();
                                            let temp_wrong_id = x.collect::<String>();
                                            eprintln!("Did you meant to write ={}", temp_wrong_id);
                                            exit(1);
                                        } else {
                                            //eprintln!()
                                            unknown_atom(id, &self.source, true);
                                        }
                                    }
                                    Atom::Number(_) => number_after_stdin(s, &self.source, true),
                                },
                                Expr::List(l) => {
                                    //println!("{:?}" , expr_list)
                                    if l.len() != 1 {
                                        multiple_exp_after_stdin(s, &self.source, true);
                                        //println!("Please provide a single assignment mew symbol for stdin input operation");
                                        //exit(1);
                                    } else if let Expr::Atom(Atom::Sym(id)) = &l[0] {
                                        identifier_name = id.lexeme.clone();
                                    } else if let Expr::Atom(Atom::Number(_)) = &l[0] {
                                        //eprintln!("I was not expecting a number here!");
                                        //exit(1);
                                        //
                                        number_after_stdin(s, &self.source, true);
                                    }
                                }
                            }

                            if !identifier_name.is_empty() {
                                match io::stdin().read_line(&mut input_buffer) {
                                    Ok(_) => {}
                                    Err(_) => {
                                        eprintln!("Failed to read the input from stdin!");
                                        exit(1);
                                    }
                                }
                            }
                            let temp_atom = Atom::Number((input_buffer.trim_end().len()) as f64);
                            let temp_id_token = MewToken {
                                lexeme: identifier_name,
                                position: (0, (0, 0)),
                            };
                            self.do_assignment(&temp_id_token, &[temp_atom], symbol_table);
                        } else if s.lexeme == *"@" {
                            if expr_list.len() < 3 {
                                loop_arg_wrong(s, &self.source, true);

                                //exit(1);
                            }
                            let mut con_expr = expr_list.drain(..2).collect::<Vec<Expr>>();
                            let mut condition_temp =
                                self.evaluate(&mut con_expr[1], symbol_table).0;
                            let mut condition: f64 = if let Some(Atom::Number(n)) = condition_temp {
                                n
                            } else {
                                0.0
                            };

                            let body = expr_list.drain(..1).collect::<Vec<Expr>>();

                            if condition >= 1.0 {
                                loop {
                                    self.evaluate(&mut body.clone()[0], symbol_table);

                                    condition_temp =
                                        self.evaluate(&mut con_expr[1], symbol_table).0;
                                    condition = if let Some(Atom::Number(n)) = condition_temp {
                                        n
                                    } else {
                                        0.0
                                    };

                                    if condition == 0.0 {
                                        if !expr_list.is_empty() {
                                            let mut else_body =
                                                expr_list.drain(..1).collect::<Vec<Expr>>();
                                            return self.evaluate(&mut else_body[0], symbol_table);
                                        } else {
                                            return (None, None);
                                        }

                                        //break;
                                    }
                                }
                            }
                        } else if s.lexeme == *"?" {
                            if expr_list.len() < 3 {
                                if_arg_wrong(s, &self.source, true);
                                //exit(1);
                            }

                            let mut con_expr = expr_list.drain(..2).collect::<Vec<Expr>>();
                            //println!("=> condition => {:?}" , con_expr[1]);
                            let condition_temp = self.evaluate(&mut con_expr[1], symbol_table).0;
                            //println!("{:?}" , self.evaluate(&mut con_expr.clone()[1], symbol_table));
                            let condition: f64 = if let Some(Atom::Number(n)) = condition_temp {
                                n
                            } else {
                                0.0
                            };
                            //println!("=> {}" , condition);
                            let mut body = expr_list.drain(..1).collect::<Vec<Expr>>();
                            //println!("{:?}" , body);
                            if condition >= 1.0 {
                                return self.evaluate(&mut body[0], symbol_table);

                                //break;
                            } else if !expr_list.is_empty() {
                                let mut else_body = expr_list.drain(..1).collect::<Vec<Expr>>();
                                return self.evaluate(&mut else_body[0], symbol_table);
                            }
                            //return (None,None)
                        }
                    }
                    for item in expr_list.iter_mut() {
                        let evaluted_res: (Option<Atom>, Option<Vec<Atom>>) =
                            self.evaluate(item, symbol_table);

                        if evaluted_res.0.is_some() && evaluted_res.1.is_none() {
                            atom_list.push(evaluted_res.0.unwrap());
                        } else if evaluted_res.1.is_some() && evaluted_res.0.is_none() {
                            atom_list.append(&mut evaluted_res.1.unwrap());
                        }
                    }

                    if !atom_list.is_empty() {
                        let clone_of_atom_list = atom_list.clone();
                        let current_operator: Vec<Atom> = atom_list.drain(..1).collect();

                        match &current_operator[0] {
                            Atom::Number(_) => {
                                return (None, Some(clone_of_atom_list));
                            }
                            Atom::Sym(symbol) => {
                                if OPERATORS.contains(&symbol.lexeme.as_str()) {
                                    return (
                                        Some(do_binary_operation(symbol, atom_list, &self.source)),
                                        None,
                                    );
                                //we only need to check if it is a assignment expression or not;
                                //because the value has already been extracted above [^ref-1]
                                //or an error has been thrown
                                } else if is_this_assignment(symbol) {
                                    //check if assignment; mew number with `=`
                                    if !atom_list.is_empty() {
                                        self.do_assignment(symbol, &atom_list, symbol_table);
                                        return (Some(Atom::Number(0.0)), None); //return zero as like lisp; everything is an expression
                                    } else {
                                        no_expression_after_id(symbol, &self.source, true);
                                        /*
                                            self.show_nice_error(
                                        symbol,
                                        "No expression provided after identifier to assign to it."
                                            .to_string(),
                                        )
                                        ;*/
                                        //exit(1);
                                    }
                                } else {
                                    unknown_atom(symbol, &self.source, true);
                                    /*
                                    self.show_nice_error(
                                        symbol,
                                        "Unexpected Atom; I don't know, what to do with this!"
                                            .to_string(),
                                    );
                                    */
                                }
                            }
                        }

                        self.evaluate(&mut expr_list[0], symbol_table)
                    } else {
                        (None, None)
                    }
                } else {
                    (None, None)
                }
            }
        }
    }

    fn do_assignment(
        &self,
        identifer: &MewToken,
        atom: &[Atom],
        symbol_table: &mut HashMap<String, f64>,
    ) {
        // the argument we got will be something like `=mewmew` so, what we have to is convert it
        // to something like `~mewmew` , so it can be found on the symbol table later;
        let mut p_id: Vec<String> = identifer.lexeme.chars().map(|c| c.to_string()).collect();
        p_id[0] = "~".to_string();
        let id = p_id.join("");

        //nice_error_atom_list(&atom, &self.source, "error list".to_string(), false);
        let mut value: f64 = 0.0;
        if atom.len() > 1 {
            // What is happening here is =>
            // [=mew [mew mew mewmew]]
            // we have to assign `[mew mew mew]` to `~mew`
            // but the expression list has no function/operator
            // so first, we convert the expression to something like this `[1 2 3]`
            // then convert it to a string "123" then parse it as float;
            // finally assign it to `~mew`

            //let extracted_atom_list: Vec<Option<f64>> =
            //    atom.into_iter().map(|a| self.extract_atom(a)).collect();

            let x = atom
                .iter()
                .map(|a| extract_atom(a, &self.source))
                .into_iter()
                .flatten()
                .map(|a| a.to_string())
                .collect::<Vec<String>>();
            let temp_value = x.join("").parse::<f64>();

            match temp_value {
                Ok(v) => value = v,
                Err(_) => {
                    //eprintln!(
                    //    "Failed to join expression list and create a single value for assignment"
                    //);
                    //
                    //
                    expresion_combine_failed(identifer, &self.source, true);
                    //exit(1);
                }
            }
        } else if let Atom::Number(n) = atom[0].to_owned() {
            value = n;
        }

        symbol_table.insert(id, value);
    }
}
