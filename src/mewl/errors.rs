use crate::mewl::types::*;
use std::process::exit;

const ERROR_LIST : [&str;6] = [

    "Sorry! I don't know the value of this variable!", // [0] //Undefined variable
    "Uh! I can't recognize this symbol! What to do with this?", // [1] //Unexpected symbol/char/atom
    "Please provide correct number of expression for this *loop*", // [2] // loop statement arguments wrong
    "Can not find correct number of arguments for this *if* expression", // [3] // if statement arguments wrong
    "No *expression(s)* provided after this Identifier to assign to it.", // [4] // no expression after identifier
    "Uh! I was not expecting a assignment here!" //[5] // unexpected assignment statement

];

const LOOP_EXAMPLE : &str = "Do something like this => \n [ @ [ Condition ] [ Body ] [ Return ] ]! (Return is optional)";
const IF_EXAMPLE : &str = "Do something like this => \n [ ? [ Condition ] [ Body ] [ False/Else Body ] ]! (False/Else is optional)";

pub fn undefined_var( token : &MewToken , code : &String , do_exit : bool ) {

    show_nice_error(token, code , ERROR_LIST[0].to_string());

    if do_exit{
        exit(1);
    }

}

pub fn no_expression_after_id( token : &MewToken , code : &String , do_exit : bool ) {

    show_nice_error(token, code , ERROR_LIST[4].to_string());

    if do_exit{
        exit(1);
    }

}

pub fn unexpected_assignment( token : &MewToken , code : &String , do_exit : bool ) {

    show_nice_error(token, code , ERROR_LIST[5].to_string());

    if do_exit{
        exit(1);
    }

}

pub fn unknown_atom(token : &MewToken , code : &String, do_exit : bool){

    show_nice_error(token, code, ERROR_LIST[1].to_string());
    if do_exit{
        exit(1);
    }

}

pub fn loop_arg_wrong(token : &MewToken , code : &String, do_exit : bool){

    show_nice_error(token, code, format!("{}\n{}" , ERROR_LIST[2] , LOOP_EXAMPLE));
    if do_exit{
        exit(1);
    }

}



pub fn if_arg_wrong(token : &MewToken , code : &String, do_exit : bool){

    show_nice_error(token, code, format!("{}\n{}" , ERROR_LIST[3] , IF_EXAMPLE));
    if do_exit{
        exit(1);
    }

}

fn show_nice_error(tok: &MewToken, source_code : &String  , err_msg: String) {
    let mut xx = source_code.clone(); //cloning the source cause I don't want to mess up the origin source;
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