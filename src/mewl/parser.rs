use std::process::exit;

use crate::mewl::evaluator::MewlEvaluator;
use crate::mewl::types::*;

use super::errors::{no_closing_bracket, unexpected_closing_bracket};
use super::eval_helpers::atomic::convert_from_mewnum;
use super::eval_helpers::mewcheck::is_this_mewnum;

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

                // if we find `//` anywhere, all the tokens are skipped unless we find a newline
                if raw_toks[curp] == '/' && raw_toks[curp + 1] == '/' {
                    line_no += 1; //increment the line index;
                    curp += 2; //skip the comment chars
                    while raw_toks[curp] != '\n' {
                        //skip everything until we find a newline
                        curp += 1;
                    }
                    curp += 1; //skip the newline;
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
            token_list.push(self.parse_raw_tokens(&mut mytoks));
        }
        //let mut st: HashMap<String, f64> = HashMap::new();
        //st.insert("~mew".to_string(), 3.0);
        //
        let mut evaluator = MewlEvaluator::new(Expr::List(token_list), self.source.clone());
        evaluator.do_eval();

        //let _ = self.evaluate(&mut Expr::List(token_list), &mut st);
        //println!("{:?}" , a);
        //println!("{:#?}" , token_list);
        //token_list
    }

    fn parse_raw_tokens(&mut self, raw_tokens: &mut Vec<MewToken>) -> Expr {
        //println!("{:?}" , raw_tokens);
        let current_token = self.poktoken(raw_tokens);
        if raw_tokens.is_empty() {
            eprintln!("No expression to parse");
        }

        match current_token.lexeme.as_str() {
            "[" => {
                //`println!("{:?}" , raw_tokens);
                let mut output_tokens: Vec<Expr> = vec![];
                while raw_tokens[0].lexeme != *"]" {
                    //                    if raw_tokens.first().unwrap().lexeme != *"]"{

                    output_tokens.push(self.parse_raw_tokens(raw_tokens));
                    /*                    }
                    else if current_token.lexeme == *"]" {
                        self.show_nice_error(&current_token, "[ found".to_string());
                        exit(1);
                    }

                    //self.poktoken(raw_tokens);
                    */
                    if raw_tokens.is_empty() {
                        // if tokens are empty; that might suggest that some brackets are still open; so error!
                        no_closing_bracket(&current_token, &self.source, false);
                        exit(1);
                    }
                }

                //println!("{:?}" , raw_tokens);
                if !raw_tokens.is_empty() {
                    self.poktoken(raw_tokens);
                }
                let output: Expr = Expr::List(output_tokens);
                output
            }
            "]" => {
                //self.show_nice_error(&current_token, "Unexpected Closing bracket!".to_string());
                unexpected_closing_bracket(&current_token, &self.source, false);
                exit(1);
            }
            _ => {
                let out: Expr = Expr::Atom(self.parse_raw_atom(&current_token));
                out
            }
        }
    }

    fn parse_raw_atom(&self, token: &MewToken) -> Atom {
        //println!("<<<<<<<|{}|=>>>>>>{:?}" , token.lexeme , self.is_mewnum(token));
        if is_this_mewnum(token) {
            //TODO: Fix
            return Atom::Number(convert_from_mewnum(token.lexeme.as_str()));
        }

        Atom::Sym(token.to_owned())
    }
}
