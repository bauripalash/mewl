#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_variables)]
use std::{collections::HashMap, process::exit};

#[derive(Debug, Clone)]
enum Atom {
    Sym(String),
    Number(f64),
}

#[derive(Debug, Clone)]
enum Expr {
    Atom(Atom),
    List(Vec<Expr>),
}

#[derive(Debug, Clone)]
struct MewToken {
    lexeme: String,
    position: (usize, (usize, usize)), //[line number, [start position, end position ]]
}

fn main() {
    let mut sym_table: HashMap<String, f64> = HashMap::new();
    sym_table.insert("age".to_string(), 20.0);
    let a = "[:: [+ mew mewmew]]
    mewmew pop
    palash
    hello";
    let mut source_input = a.to_string();
    source_input.push('\n');
    source_input.push(' '); // without these last statements are not being parser; TODO: Fix this

    let _ = MewlParser::new(source_input).parse();
}

#[allow(dead_code)]
struct MewlParser {
    source: String, //Source string as is; could've used a simple vector but this is String for now for the show_nice_error() function to work properly
    tokens: Vec<String>, // Raw tokens as strings;
    current_atom: Atom, // not used for anything as of now;
}

impl MewlParser {
    fn new(source: String) -> Self {
        Self {
            source: source.replace('[', " [ ").replace(']', " ] "), // adding space between parens to make parsing easier
            //TODO: Support all bracket types -> () {} []
            tokens: Vec::new(),
            current_atom: Atom::Number(0.0),
        }
    }

    fn poktoken(&mut self) -> String {
        self.tokens.drain(..1).collect()
    }

    fn read_atom(&self) -> Atom {
        unimplemented!()
    }

    fn extract_atom(&self) -> Option<f64> {
        unimplemented!();
    }

    fn get_tokens(&self) -> Vec<MewToken> {
        let raw_toks: Vec<char> = self.source.chars().collect();
        //println!("{:?}" , raw_toks);
        let mut output: Vec<MewToken> = vec![];
        let mut curp: usize = 0; //current position of of reader
        let mut curtok: String = String::new(); //current token; blank at first and later filled
        let mut line_no: usize = 1; //current line number; for the function show_nice_error()
        while curp < raw_toks.len() {
            //The below hack feels a little complex;
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

            //END of WHITESPACE skipping loop

            //if we reached here; that means we have found a whitespace or tab char;
            //so now we should a filled `curtok`
            //we prepare a `Token` with position data and push the final token to the `output` variable
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

    fn parse(&mut self) -> Vec<Expr> {
        let mytoks = self.get_tokens();
        self.show_nice_error(&mytoks[10], "This wasn't supposed to happen!".to_string());
        Vec::new() // parser will be the main function of the parser; returns are unnecessary
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

        let mut line_index = 0;
        if newline_next {
            line_index = tok.position.0 - 2
        } else {
            line_index = tok.position.0 - 1
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

        exit(1);
    }

    fn evaluate(&mut self, sym_table: &mut HashMap<String, f64>) -> Atom {
        Atom::Number(0.0)
    }
}
