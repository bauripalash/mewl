#[derive(Debug, Clone)]
pub enum Atom {
    Sym(MewToken),
    Number(f64),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Atom(Atom),
    List(Vec<Expr>),
}

#[derive(Debug, Clone)]
pub struct MewToken {
    pub lexeme: String,
    pub position: (usize, (usize, usize)), //[line number, [start position, end position ]]
}
