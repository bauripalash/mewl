use mewl::mewl::evaluator::MewlEvaluator;
use mewl::mewl::parser::MewlParser;
use mewl::mewl::types::Atom;

pub fn get_return_value(source: &str) -> f64 {
    let parser = MewlParser::new(source.to_string()).parse(false);

    let m = MewlEvaluator::new(parser.unwrap(), source.to_string()).do_eval();
    let result = if m.0.is_some() && m.1.is_none() {
        match m.0.unwrap() {
            Atom::Number(n) => n,
            _ => panic!(),
        }
    } else if m.1.is_some() && m.0.is_none() {
        println!("{:?}", m.1);
        match m.1.unwrap()[0] {
            Atom::Number(n) => n,
            _ => panic!(),
        }
    } else {
        0.0
    };

    result

    //println!("`{}` should be => {}" , source , 2.0);
    //assert_eq!(2.0_f64 , result);
}
