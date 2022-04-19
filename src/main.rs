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
    position: (usize, (usize, usize)), //[line number, column position]
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
    source_input.push(' ');

    let _ = MewlParser::new(source_input).parse();
}

#[allow(dead_code)]
struct MewlParser {
    source: String,
    tokens: Vec<String>,
    current_atom: Atom,
}

impl MewlParser {
    fn new(source: String) -> Self {
        Self {
            source: source.replace('[', " [ ").replace(']', " ] "),
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
        let mut curp: usize = 0;
        let mut curtok: String = String::new();
        let mut line_no: usize = 1;
        while curp < raw_toks.len() {
            while raw_toks[curp] != ' ' {
                if raw_toks[curp] == '\n' {
                    line_no += 1;
                    curp += 1;
                    continue;
                }

                curtok.push(raw_toks[curp]);
                curp += 1;
            }

            if !curtok.is_empty() {
                let temp_token = MewToken {
                    lexeme: curtok.clone(),
                    position: (line_no, (curp - curtok.len(), curp)),
                };
                output.push(temp_token);
                curtok = String::new();
                //curp+=1;
            }

            curp += 1;
        }

        output
    }

    fn parse(&mut self) -> Vec<Expr> {
        let mytoks = self.get_tokens();
        //println!("{:#?}", mytoks);
        //for y in mytoks{
        //println!("{:?}" , y);
        self.show_nice_error(&mytoks[10], "This wasn't supposed to happen!".to_string());
        //}
        Vec::new()
    }

    fn show_nice_error(&self, tok: &MewToken, err_msg: String) {
        let mut xx = self.source.clone();
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

        if line_index != 0 && o.len() > line_index {
            println!("|{}| {}", line_index, o[line_index - 1])
        }

        println!("|{}| {}", line_index + 1, o[line_index]);

        if line_index < o.len() {
            println!("|{}| {}", line_index + 2, o[line_index + 1])
        }
    }

    fn evaluate(&mut self, sym_table: &mut HashMap<String, f64>) -> Atom {
        Atom::Number(0.0)
    }
}
