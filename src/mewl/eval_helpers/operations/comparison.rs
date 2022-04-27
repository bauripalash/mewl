pub fn do_comparison(op: &str, exp_args: Vec<f64>) -> f64 {
    //println!("=> ->{}<- COMP => {:?}" , op ,exp_args);
    let temp_res: Option<&f64> = match op {
        "==" => exp_args
            .windows(2)
            .all(|a| a[0] == a[1])
            .then(|| &1_f64),
        "!=" => exp_args
            .windows(2)
            .all(|a| a[0] != a[1])
            .then(|| &exp_args[0]),
        "<" => exp_args
            .windows(2)
            .all(|a| a[0] < a[1])
            .then(|| &exp_args[0]),
        ">" => exp_args
            .windows(2)
            .all(|a| a[0] > a[1])
            .then(|| &exp_args[0]),
        ">=" => exp_args
            .windows(2)
            .all(|a| a[0] >= a[1])
            .then(|| &exp_args[0]),
        "<=" => exp_args
            .windows(2)
            .all(|a| a[0] <= a[1])
            .then(|| &exp_args[0]),
        "#" => {
            if exp_args.iter().any(|a| a == &(1.0_f64)) {
                Some(&1.0)
            } else {
                None
            }
        }
        "&" => {
            if exp_args.iter().all(|a| a == &(1.0_f64)) {
                Some(&1.0)
            } else {
                None
            }
        }
        "!" => {
            if exp_args[0] == 1.0 {
                None
            } else {
                Some(&1.0)
            }
        }
        _ => None,
    };
    //println!("{:?}" , temp_res);
    if temp_res.is_some() {
        
        1.0
    } else {
        0.0
    }
}
