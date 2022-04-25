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

pub const OPERATORS: [&str; 24] = [
    "+", //Addition
    "-", //Substraction
    "*", //Multiplication
    "/", //Division
    "::", //Print as is (Array or just single element)
    ":::", //print as char
    ">", // A > B ==> A is greater than B
    "<", // A < B ==> A is less than B
    "==", // A == B ==> A is equal to B
    "!=", // A != B ==> A is not equal to B
    "<=",  // A < = B ==> A is less than equal to B
    ">=",  // A > = B ==> A is greater than equal to B
    "@",  // Loop / While Loop
    "?", // If statemet
    "&", // Simple True/False AND Operation
    "#", // Simple True/False  OR Operation
    "!" , // Simple True/False NOT Operation
    "^", // Bitwise XOR
    "**", // Power
    ">>" , // Bitwise Right Shift
    "<<" , // Bitwise Left Shift 
    "!!" , // Bitwise NOT 
    "##" , // Bitwise OR 
    "&&" //Bitwise AND 
];
