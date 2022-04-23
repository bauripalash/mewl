use crate::mewl::eval_helpers::atomic::*;
use crate::mewl::eval_helpers::operations::comparison::do_comparison;
use crate::mewl::types::Atom;

pub fn do_binary_operation(op: &str, exp_args: Vec<Atom>, source: &str) -> Atom {
    //println!("{:?}" , exp_args);
    let extracted_atom_list: Vec<Option<f64>> = exp_args
        .into_iter()
        .map(|a| extract_atom(&a, source))
        .collect();
    let mut result: f64 = 0.0;
    match op {
        "+" => {
            result = extracted_atom_list
                .into_iter()
                .flatten()
                .into_iter()
                .fold(0.0, |a, b| a + b);
        }

        "-" => {
            result = extracted_atom_list
                .into_iter()
                .flatten()
                .reduce(|a, b| a - b)
                .unwrap();
        }

        "*" => {
            result = extracted_atom_list
                .into_iter()
                .flatten()
                .into_iter()
                .fold(1.0, |a, b| a * b);
        }

        "/" => {
            result = extracted_atom_list
                .into_iter()
                .flatten()
                .into_iter()
                .reduce(|a, b| b / a)
                .unwrap();
        }

        ">" | "<" | "==" | "!=" | "<=" | ">=" => {
            let flat_list: Vec<f64> = extracted_atom_list.into_iter().flatten().collect();

            result = match flat_list.is_empty() {
                true => 0.0,
                false => do_comparison(op, flat_list),
            };
        }

        "::" => {
            //println!("{:?}" , extracted_atom_list);
            println!(
                "{}",
                extracted_atom_list
                    .into_iter()
                    .flatten()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            );
        }

        ":::" => {
            println!(
                "{}",
                String::from_utf8_lossy(
                    &extracted_atom_list
                        .into_iter()
                        .flatten()
                        .map(|a| a as u8)
                        .collect::<Vec<u8>>()
                )
            )
        }

        _ => {}
    }
    Atom::Number(result)
}
