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
    position: (usize, usize), //[line number, column position]
}

fn main() {
    let mut sym_table: HashMap<String, f64> = HashMap::new();
    sym_table.insert("age".to_string(), 20.0);
    let a = "[:: [+ mew mewmew]]
    mewmew
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
                    position: (line_no, curp - curtok.len()),
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
        self.show_nice_error(&mytoks[9]);
        //}
        Vec::new()
    }

    fn show_nice_error(&self, tok: &MewToken) {
        //println!("{}" , self.source);
        let mut xx = self.source.clone();

        xx.insert_str(tok.position.1 + if tok.lexeme.len() > 1 {tok.lexeme.len()-1}else{tok.lexeme.len()}, "<-\x1b[0m");
        xx.insert_str(if tok.lexeme.len() > 1 { tok.position.1-1} else {tok.position.1}, " \x1b[93m->");

        let o: Vec<String> = xx.split_terminator('\n').into_iter().map(|i| i.to_string()).collect();

        let line_index = tok.position.0;
        println!("{}-{}->{:?}" , line_index , o.len() , o);
        if line_index > 1 {
            println!("|{}| {}", line_index - 1, o[line_index - 2]);
        }

        println!(
            "{}",
            format!("|{}|", tok.position.0) + &o[tok.position.0 - 1]
        );
        
        /*
        if line_index < o.len() && line_index != o.len() {
            println!("|{}| {}", line_index + 1, o[line_index + 1]);
        }*/
        
        
        //println!("{:?}" , tok)
    }

    fn evaluate(&mut self, sym_table: &mut HashMap<String, f64>) -> Atom {
        Atom::Number(0.0)
    }
}
