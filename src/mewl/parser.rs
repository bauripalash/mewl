use std::process::exit;

use crate::mewl::evaluator::MewlEvaluator;
use crate::mewl::types::*;

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
                        self.show_nice_error(
                            &current_token,
                            "Cannot find closing bracket for this opening bracket".to_string(),
                        );
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
                self.show_nice_error(&current_token, "Unexpected Closing bracket!".to_string());
                exit(1);
            }
            _ => {
                let out: Expr = Expr::Atom(self.parse_raw_atom(&current_token));
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

    fn parse_raw_atom(&self, token: &MewToken) -> Atom {
        //println!("<<<<<<<|{}|=>>>>>>{:?}" , token.lexeme , self.is_mewnum(token));
        if self.is_mewnum(token) {
            //TODO: Fix
            return Atom::Number(token.lexeme.len() as f64 / 3.0);
        }

        Atom::Sym(token.to_owned())
    }

    fn is_mewnum(&self, token: &MewToken) -> bool {
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
}
