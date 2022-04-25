use crate::mewl::types::*;
use std::process::exit;

const ERROR_LIST: [&str; 10] = [
    "Sorry! I don't know the value of this variable!", // [0] //Undefined variable
    "Uh! I can't recognize this symbol! What to do with this?", // [1] //Unexpected symbol/char/atom
    "Please provide correct number of expression for this *loop*", // [2] // loop statement arguments wrong
    "Can not find correct number of arguments for this *if* expression", // [3] // if statement arguments wrong
    "No *expression(s)* provided after this Identifier to assign to it.", // [4] // no expression after identifier
    "Uh! I was not expecting a assignment here!", //[5] // unexpected assignment statement
    "I cannot combine the *expressions* for this assignment operation", // [6] //failed to combine expression for assignment
    "Please provide a assignment mew symbol after this read input function", // [7] //Read MewNum/Number from stdin
    "I was not expecting a number as argument for this read input function", // [8] Got a number argument for stdin function
    "Please only provide a single assignment expression as argument to stdin function", //[9] //got multiple expressions after stdin function
];

const LOOP_EXAMPLE: &str =
    "Do something like this => \n [ @ [ Condition ] [ Body ] [ Return ] ]! (Return is optional)";
const IF_EXAMPLE : &str = "Do something like this => \n [ ? [ Condition ] [ Body ] [ False/Else Body ] ]! (False/Else is optional)";

pub fn multiple_exp_after_stdin(token: &MewToken, code: &str, do_exit: bool) {
    show_nice_error(token, code, ERROR_LIST[9].to_string());

    if do_exit {
        exit(1);
    }
}

pub fn no_assignment_symbol_after_stdin(token: &MewToken, code: &str, do_exit: bool) {
    show_nice_error(token, code, ERROR_LIST[7].to_string());

    if do_exit {
        exit(1);
    }
}

pub fn number_after_stdin(token: &MewToken, code: &str, do_exit: bool) {
    show_nice_error(token, code, ERROR_LIST[8].to_string());

    if do_exit {
        exit(1);
    }
}

pub fn expresion_combine_failed(token: &MewToken, code: &str, do_exit: bool) {
    show_nice_error(token, code, ERROR_LIST[6].to_string());
    if do_exit {
        exit(1);
    }
}

pub fn undefined_var(token: &MewToken, code: &str, do_exit: bool) {
    show_nice_error(token, code, ERROR_LIST[0].to_string());

    if do_exit {
        exit(1);
    }
}

pub fn no_expression_after_id(token: &MewToken, code: &str, do_exit: bool) {
    show_nice_error(token, code, ERROR_LIST[4].to_string());

    if do_exit {
        exit(1);
    }
}

pub fn unexpected_assignment(token: &MewToken, code: &str, do_exit: bool) {
    show_nice_error(token, code, ERROR_LIST[5].to_string());

    if do_exit {
        exit(1);
    }
}

pub fn unknown_atom(token: &MewToken, code: &str, do_exit: bool) {
    show_nice_error(token, code, ERROR_LIST[1].to_string());
    if do_exit {
        exit(1);
    }
}

pub fn loop_arg_wrong(token: &MewToken, code: &str, do_exit: bool) {
    show_nice_error(token, code, format!("{}\n{}", ERROR_LIST[2], LOOP_EXAMPLE));
    if do_exit {
        exit(1);
    }
}

pub fn if_arg_wrong(token: &MewToken, code: &str, do_exit: bool) {
    show_nice_error(token, code, format!("{}\n{}", ERROR_LIST[3], IF_EXAMPLE));
    if do_exit {
        exit(1);
    }
}

//TODO
pub fn nice_error_atom_list(atom_list: &[Atom], source_code: &str, err_msg: String, do_exit: bool) {
    if atom_list.len() == 1 {
        if let Atom::Sym(s) = &atom_list[0] {
            show_nice_error(s, source_code, err_msg);
            if do_exit {
                exit(1);
            }
        }
    }
    let mut mewtok_list: Vec<MewToken> = vec![];
    for atom in atom_list {
        if let Atom::Sym(s) = atom {
            mewtok_list.push(s.to_owned());
        }
    }
    println!("{:?}", mewtok_list);
}

fn show_nice_error(tok: &MewToken, source_code: &str, err_msg: String) {
    let mut xx = source_code.to_string(); //cloning the source cause I don't want to mess up the origin source;
                                          // the parser maybe able to catch other error; so source should not be mutated; I guess;

    //checks if the next char a linefeed char `\n` for below bug
    //BUG: If there is a linefeed char after the error token -
    //the token highlight is also including the `\n`
    let newline_next =
        xx.chars().map(|s| s.to_string()).collect::<Vec<String>>()[tok.position.1 .1 - 1] == "\n";

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
