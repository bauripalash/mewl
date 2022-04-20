use std::collections::HashMap;
use std::process::exit;

use crate::mewl::types::*;

const OPERATORS: [&str; 6] = ["+", "-", "*", "/", "::", ":::"];

#[allow(dead_code)]
pub struct MewlParser {
    source: String, //Source string as is; could've used a simple vector but this is String for now for the show_nice_error() function to work properly
    tokens: Vec<String>, // Raw tokens as strings;
    current_atom: Atom, // not used for anything as of now;
}

impl MewlParser {
    pub fn new(mut source: String) -> Self {
        //let source_code = source.clone();
        source.push('\n');
        source.push(' '); // without these last statements are not being parser; TODO: Fix this, if possible

        Self {
            source: source.replace('[', " [ ").replace(']', " ] "), // adding space between parens to make parsing easier
            //TODO: Support all bracket types -> () {} []
            tokens: Vec::new(),
            current_atom: Atom::Number(0.0),
        }
    }

    fn poktoken(&self, tokens: &mut Vec<MewToken>) -> MewToken {
        tokens.drain(..1).next().unwrap()
    }

    fn get_tokens(&self) -> Vec<MewToken> {
        let raw_toks: Vec<char> = self.source.chars().collect();
        // println!("{:?}" , raw_toks);
        let mut output: Vec<MewToken> = vec![];
        let mut curp: usize = 0; //current position of of reader
        let mut curtok: String = String::new(); //current token; blank at first and later filled
        let mut line_no: usize = 1; //current line number; for the function show_nice_error()
        while curp < raw_toks.len() {
            // The below hack feels a little complex;
            // Skip whitespaces and tabs
            // as soon as we find a non-space char; we start pushing the next chars
            // to the `curtok` variable [continued...]

            // WHITESPACE skipping loop
            while raw_toks[curp] != ' ' && raw_toks[curp] != '\t' {
                if raw_toks[curp] == '\n' {
                    line_no += 1;
                    curp += 1;
                    continue;
                }

                curtok.push(raw_toks[curp]); //if the current char is not; the char is pushed to `curtok`
                curp += 1;
            }

            // END of WHITESPACE skipping loop

            // if we reached here; that means we have found a whitespace or tab char;
            // so now we should a filled `curtok`
            // we prepare a `Token` with position data and push the final token to the `output` variable
            if !curtok.is_empty() {
                let temp_token = MewToken {
                    lexeme: curtok.clone(),
                    position: (line_no, (curp - curtok.len(), curp)),
                };
                output.push(temp_token);
                curtok = String::new();
            }

            curp += 1;
            //we now have to advance;
            //I am not sure about this;
        }

        output
    }

    pub fn parse(&mut self) {
        let mut mytoks = self.get_tokens();
        let mut token_list: Vec<Expr> = vec![];
        while !mytoks.is_empty() {
            token_list.push(Expr::List(self.parse_raw_tokens(&mut mytoks)));
        }
        let _ = self.evaluate(&mut Expr::List(token_list), &mut HashMap::new());
        //println!("{:?}" , a);
        //println!("{:#?}" , token_list);
        //token_list
    }

    fn evaluate(
        &mut self,
        exp: &mut Expr,
        symbol_table: &mut HashMap<String, f64>,
    ) -> (Option<Atom>, Option<Vec<Atom>>) {
        match exp {
            Expr::Atom(atom) => match atom {
                Atom::Number(_) => (Some(atom.to_owned()), None),
                Atom::Sym(atom_symbol) => {
                    if OPERATORS.contains(&atom_symbol.lexeme.as_str()) {
                        return (Some(atom.to_owned()), None);
                    }

                    let variable_value = symbol_table.get(&atom_symbol.lexeme);

                    if let Some(..) = variable_value {
                        (Some(Atom::Number(*variable_value.unwrap())), None)
                    } else {
                        self.show_nice_error(
                            atom_symbol,
                            "Sorry! I dont know the value of this variable".to_string(),
                        );
                        exit(1);
                    }
                }
            },

            Expr::List(expr_list) => {
                let mut atom_list: Vec<Atom> = vec![];

                for item in expr_list.iter_mut() {
                    let evaluted_res: (Option<Atom>, Option<Vec<Atom>>) =
                        self.evaluate(item, symbol_table);

                    if evaluted_res.0.is_some() && evaluted_res.1.is_none() {
                        atom_list.push(evaluted_res.0.unwrap());
                    } else if evaluted_res.1.is_some() && evaluted_res.0.is_none() {
                        atom_list.append(&mut evaluted_res.1.unwrap());
                    }
                }
                let clone_of_atom_list = atom_list.clone();
                let current_operator: Vec<Atom> = atom_list.drain(..1).collect();

                match &current_operator[0] {
                    Atom::Number(_) => {
                        return (None, Some(clone_of_atom_list));
                    }
                    Atom::Sym(symbol) => {
                        if OPERATORS.contains(&symbol.lexeme.as_str()) {
                            return (
                                Some(self.do_binary_operation(symbol.lexeme.as_str(), atom_list)),
                                None,
                            );
                        } else {
                            self.show_nice_error(
                                symbol,
                                "Unexpected Atom; I don't know, what to do with this!".to_string(),
                            );
                        }
                    }
                }

                self.evaluate(&mut expr_list[0], symbol_table)
            }
        }
    }

    fn extract_atom(&self, atom: &Atom) -> Option<f64> {
        match atom {
            Atom::Number(atm) => Some(*atm),
            _ => None,
        }
    }

    fn do_binary_operation(&self, op: &str, exp_args: Vec<Atom>) -> Atom {
        //println!("{:?}" , exp_args);
        let extracted_atom_list: Vec<Option<f64>> = exp_args
            .into_iter()
            .map(|a| self.extract_atom(&a))
            .collect();
        let mut result: f64 = 0.0;
        match op {
            "+" => {
                result = extracted_atom_list
                    .into_iter()
                    .flatten()
                    .into_iter()
                    .fold(0.0, |a, b| a + b);
            }

            "-" => {
                result = extracted_atom_list
                    .into_iter()
                    .flatten()
                    .reduce(|a, b| a - b)
                    .unwrap();
            }

            "*" => {
                result = extracted_atom_list
                    .into_iter()
                    .flatten()
                    .into_iter()
                    .fold(1.0, |a, b| a * b);
            }

            "/" => {
                result = extracted_atom_list
                    .into_iter()
                    .flatten()
                    .into_iter()
                    .reduce(|a, b| b / a)
                    .unwrap();
            }

            "::" => {
                println!(
                    "{}",
                    extracted_atom_list
                        .into_iter()
                        .flatten()
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                );
            }

            ":::" => {
                println!(
                    "{}",
                    String::from_utf8_lossy(
                        &extracted_atom_list
                            .into_iter()
                            .flatten()
                            .map(|a| a as u8)
                            .collect::<Vec<u8>>()
                    )
                )
            }

            _ => {}
        }
        Atom::Number(result)
    }

    fn is_mewnum(&self, token: &MewToken) -> bool {
        let mut token_lexeme = token.lexeme.chars();
        //let mut result = false;

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
    fn is_identifier(&self, token: &MewToken) -> bool {
        let mut token_lexeme = token.lexeme.chars();
        //let mut result = false;
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

    fn parse_raw_atom(&self, token: &MewToken) -> Atom {
        //println!("<<<<<<<|{}|=>>>>>>{:?}" , token.lexeme , self.is_mewnum(token));
        if self.is_mewnum(token) {
            //TODO: Fix
            return Atom::Number(token.lexeme.len() as f64 / 3.0);
        }

        Atom::Sym(token.to_owned())
    }

    fn parse_raw_tokens(&mut self, raw_tokens: &mut Vec<MewToken>) -> Vec<Expr> {
        if raw_tokens.is_empty() {
            eprintln!("No expression to parse");
        }

        let current_token = self.poktoken(raw_tokens);
        match current_token.lexeme.as_str() {
            "[" => {
                let mut output_tokens: Vec<Expr> = vec![];
                while raw_tokens[0].lexeme != *"]" {
                    output_tokens.append(&mut self.parse_raw_tokens(raw_tokens));
                }
                self.poktoken(raw_tokens);
                let output: Vec<Expr> = vec![Expr::List(output_tokens)];
                output
            }
            "]" => {
                self.show_nice_error(&current_token, "Unexpected Closing bracket!".to_string());
                exit(1);
            }
            _ => {
                let out: Vec<Expr> = vec![Expr::Atom(self.parse_raw_atom(&current_token))];
                out
            }
        }
    }

    fn show_nice_error(&self, tok: &MewToken, err_msg: String) {
        let mut xx = self.source.clone(); //cloning the source cause I don't want to mess up the origin source;
                                          // the parser maybe able to catch other error; so source should not be mutated; I guess;

        //checks if the next char a linefeed char `\n` for below bug
        //BUG: If there is a linefeed char after the error token -
        //the token highlight is also including the `\n`
        let newline_next = xx.chars().map(|s| s.to_string()).collect::<Vec<String>>()
            [tok.position.1 .1 - 1]
            == "\n";

        xx.insert_str(
            if newline_next {
                tok.position.1 .1 - 1
            } else {
                tok.position.1 .1
            },
            " <-\x1b[0m",
        );

        xx.insert_str(tok.position.1 .0 - 1, " \x1b[96;1m-> ");

        let o: Vec<String> = xx
            .split_terminator('\n')
            .into_iter()
            .map(|i| i.trim().to_string().replace('\n', ""))
            .collect();

        let mut line_index = tok.position.0;
        if newline_next {
            line_index -= 2
        } else {
            line_index -= 1
        }

        if !err_msg.is_empty() {
            eprintln!("\x1b[95m[Eh!] : {} \x1b[0m\n", err_msg);
        }

        //line before the error line
        if line_index != 0 && o.len() > line_index {
            println!("|{}| {}", line_index, o[line_index - 1])
        }
        //error token's line
        println!("|{}| {}", line_index + 1, o[line_index]);

        //next line after error
        if line_index < o.len() {
            println!("|{}| {}", line_index + 2, o[line_index + 1])
        }

        //exit(1);
    }
}
