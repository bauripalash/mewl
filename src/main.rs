use std::usize;


#[derive(Debug)]
enum Atom{
    
    Sym(String),
    Number(usize)
}

#[derive(Debug)]
enum Expr {

   Atom(Atom),
   List(Vec<Expr>)
    
}



fn main() {
    let a = "[+ mew mewmew mewmewmewmew [- mew mew]]";
    //let b = "[ধরি বয়স ২০]";
    let mut source_input = a.to_string();


    source_input = source_input.replace("[", " [ ").replace("]", " ] ");
    let mut tokens = source_input.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
    let tok_list = read_tokens(&mut tokens);
    evaluate(&tok_list[0]);
    println!("{:#?}" , tok_list);
    
}

fn poptoken(tokens : &mut Vec<String>) -> String{
    tokens.drain(..1).collect()
}

fn read_tokens(tokens : &mut Vec<String>) -> Vec<Expr>{
    //let mut tokens = toks.clone();
    if tokens.len() < 1{
        eprintln!("Unexpected End found");
    }
    //tokens.pop().unwrap();
    let cur_tok = poptoken(tokens);

    //println!("{}<=>{:?}" , cur_tok ,tokens);
    //println!("CUR=>{}" , cur_tok);
    if cur_tok == "["{
        let mut outlist : Vec<Expr> = vec![];
        while tokens[0] != "]"{
            outlist.append(&mut read_tokens(tokens));
        }
        poptoken(tokens);
        let x : Vec<Expr> = vec![Expr::List(outlist)];
        return x;

    }else if cur_tok == "]"{
        eprintln!("Unexpected ]");
        panic!();
    }else{
        let out : Vec<Expr> = vec![Expr::Atom(read_atom(cur_tok))];
        //poptoken(tokens);
        //println!("{:?}" , out);
        return out;
    }
    

}


fn read_atom(tok : String) -> Atom{
    
    if tok.starts_with("mew"){
        
        let t : Vec<char> = tok.chars().into_iter().collect();
        return Atom::Number((t.len() / 3) as usize);

    }

    Atom::Sym(tok)

}


fn evaluate(exp : &Expr){
    match exp{
        Expr::List(l) => {
            
            match &l[0]{
                Expr::Atom(a) => {
                    match a {
                Atom::Number(n) => { println!("Found number -> {}" , n) }

                Atom::Sym(s) => { 
                    
                    match s.as_str(){
                        "?" => {}

                        _ => { println!("Do something with => {:?}" , &l[1..]) }
                    }

                }

            }}
                _=>{}
            }

        }
        _ => {}
    };
    //println!("{:?}" , exp[0])

}
