use std::{collections::HashMap, usize};

#[derive(Debug, Clone)]
enum Atom {
    Sym(String),
    Number(usize),
}

#[derive(Debug, Clone)]
enum Expr {
    Atom(Atom),
    List(Vec<Expr>),
}

fn main() {
    let mut sym_table: HashMap<String, usize> = HashMap::new();
    sym_table.insert("age".to_string(), 20);
    let a = "[+ [- age mew] [ :: mewmew mewmew ]]";
    let mut source_input = a.to_string();

    source_input = source_input.replace("[", " [ ").replace("]", " ] ");
    let mut tokens = source_input
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let tok_list = read_tokens(&mut tokens);
    let x = evaluate(&tok_list[0], &mut sym_table);
    println!("{:?}", x);
    //println!("{:#?}", tok_list);
}

fn poptoken(tokens: &mut Vec<String>) -> String {
    tokens.drain(..1).collect()
}

fn read_tokens(tokens: &mut Vec<String>) -> Vec<Expr> {
    //let mut tokens = toks.clone();
    if tokens.len() < 1 {
        eprintln!("Unexpected End found");
    }
    let cur_tok = poptoken(tokens);
    match cur_tok.as_str() {
        "[" => {
            let mut outlist: Vec<Expr> = vec![];
            while tokens[0] != "]" {
                outlist.append(&mut read_tokens(tokens));
            }
            poptoken(tokens);
            let x: Vec<Expr> = vec![Expr::List(outlist)];
            return x;
        }
        "]" => {
            eprintln!("Unexpected ]");
            panic!();
        }
        _ => {
            let out: Vec<Expr> = vec![Expr::Atom(read_atom(cur_tok))];
            //poptoken(tokens);
            //println!("{:?}" , out);
            return out;
        }
    }
}

fn read_atom(tok: String) -> Atom {
    if tok.starts_with("mew") {
        let t: Vec<char> = tok.chars().into_iter().collect();
        return Atom::Number((t.len() / 3) as usize);
    }

    Atom::Sym(tok)
}

fn extract_atom(x: &Atom) -> Option<usize> {
    match x {
        Atom::Number(s) => Some(*s),
        _ => None,
    }
}

fn binary_op(op: &str, largs: Vec<Atom>) -> Atom {
    let mut result: Atom = Atom::Number(0);
    let converted: Vec<Option<usize>> = largs.iter().map(|a| extract_atom(a)).collect();

    let mut res: usize = 0;
    match op {
        "+" => {
            for atom in &converted {
                if atom.is_some() {
                    res += atom.unwrap();
                }
                //println!("Result => {}" , res);
            }
        }

        "-" => {
             
            res = converted[0].unwrap_or(0 as usize);
            for atom in &converted{
                if atom.is_some(){
                    res -= atom.unwrap();
                }

            }

        }

        "::" => {
            
            println!("{:?}" , converted);

        }
        _ => {}
    }
    
    result = Atom::Number(res);
    result
}

fn evaluate<'a>(exp: &'a Expr, sym_table: &'a mut HashMap<String, usize>) -> Atom {
    match exp {
        Expr::Atom(atm) => match atm {
            Atom::Number(_) => {
                return atm.to_owned();
            }
            Atom::Sym(sym) => {
                match sym.as_str() {
                    "+" | "-" | "::" => {
                        return atm.to_owned();
                    }
                    _ => {
                        let output = Atom::Number(*sym_table.get(sym).unwrap());
                        return output.to_owned();
                    }
                }
            }
        },
        Expr::List(lst) => {
            let _l: Vec<Expr> = lst.to_vec();
            let mut atom_list: Vec<Atom> = vec![];
            //println!("=>{:?}" , l[0]);
            for item in _l {
                let x = evaluate(&item, sym_table);
                atom_list.push(x);
            }

            let operator: Vec<Atom> = atom_list
                .drain(..1)
                .into_iter()
                .map(|a| a.to_owned())
                .collect();
            match &operator[0] {
                Atom::Number(_) => {
                    println!("{:?}", atom_list)
                }
                Atom::Sym(op) => match op.as_str() {
                    "+" | "::" => {
                        return binary_op(op.as_str(), atom_list.clone());
                    }
                    _ => {}
                },
            }

            return evaluate(&lst[0], sym_table);
        }
    }
}
