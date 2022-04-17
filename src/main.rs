use std::collections::HashMap ;

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

fn main() {
    let mut sym_table: HashMap<String, f64> = HashMap::new();
    sym_table.insert("age".to_string(), 20.0);
    let a = "[::[[- mewmew mewmewmew mew]]]";
    let mut source_input = a.to_string();

    source_input = source_input.replace('[', " [ ").replace(']', " ] ");
    let mut tokens = source_input
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let tok_list = read_tokens(&mut tokens);
    let _ = evaluate(&tok_list[0], &mut sym_table);
    //println!("{:?}", x);
    //println!("{:#?}", tok_list);
}

fn poptoken(tokens: &mut Vec<String>) -> String {
    tokens.drain(..1).collect()
}

fn read_tokens(tokens: &mut Vec<String>) -> Vec<Expr> {
    //let mut tokens = toks.clone();
    if tokens.is_empty() {
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
            x
        }
        "]" => {
            eprintln!("Unexpected ]");
            panic!();
        }
        _ => {
            let out: Vec<Expr> = vec![Expr::Atom(read_atom(cur_tok))];
            //poptoken(tokens);
            //println!("{:?}" , out);
            out
        }
    }
}

fn read_atom(tok: String) -> Atom {
    if tok.starts_with("mew") {
        //let t: Vec<char> = tok.chars().into_iter().collect();
        return Atom::Number(tok.chars().into_iter().count() as f64 / 3.0);
    }

    Atom::Sym(tok)
}

fn extract_atom(x: &Atom) -> Option<f64> {
    match x {
        Atom::Number(s) => Some(*s),
        _ => None,
    }
}

fn binary_op(op: &str, largs: Vec<Atom>) -> Atom {
    let mut result: Atom = Atom::Number(0.0);
    let converted: Vec<Option<f64>> = largs.iter().map(extract_atom).collect();

    let mut res: f64 = 0.0;
    match op {
        "+" => {
            res = converted
                .into_iter()
                .flatten()
                .into_iter()
                .fold(0.0, |a, b| a + b);

            //res = usable_values.into_iter().fold(0_usize, |a,b| a+b);
        }

        "-" => {
            /*
            res = converted
                .into_iter()
                .flatten()
                .into_iter()
                .fold(0.0, |a, b| a-b);
            println!("{}" , res);*/
            res = converted
                .into_iter()
                .flatten()
                .reduce(|a,b| a-b).unwrap();

        }

        "*" => {
            res = converted
                .into_iter()
                .flatten()
                .into_iter()
                .fold(1.0, |a, b| a * b);
        }

        "/" => {
            res = converted
                .into_iter()
                .flatten()
                .into_iter()
                .reduce(|a, b| b / a).unwrap();
        }

        "::" => {
            println!(
                "{}",
                converted
                    .into_iter()
                    .flatten()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            );
        }
        _ => {}
    }

    result = Atom::Number(res);
    result
}

fn evaluate<'a>(exp: &'a Expr, sym_table: &'a mut HashMap<String, f64>) -> Atom {
    let _lang_ops: [&str; 5] = ["+", "-", "*", "/", "::"];
    match exp {
        Expr::Atom(atm) => match atm {
            Atom::Number(_) => {
                atm.to_owned()
            }
            Atom::Sym(sym) => match sym.as_str() {
                "+" | "-" | "*" | "/" | "::" => {
                    atm.to_owned() 
                }
                _ => {
                    let output = Atom::Number(*sym_table.get(sym).unwrap());
                    output
                }
            },
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
                //.map(|a| a)
                .collect();
            match &operator[0] {
                Atom::Number(x) => {
                    return Atom::Number(*x);
                    //println!("{:?}", atom_list)
                }
                Atom::Sym(op) => match op.as_str() {
                    "+" | "-" | "*" | "/" | "::" => {
                        return binary_op(op.as_str(), atom_list.clone());
                    }
                    _ => {}
                },
            }

            evaluate(&lst[0], sym_table)
        }
    }
}
