pub fn do_comparison(op: &str, exp_args: Vec<f64>) -> f64 {
    let temp_res: Option<&f64> = match op {
        "==" => exp_args
            .windows(2)
            .all(|a| a[0] == a[1])
            .then(|| &exp_args[0]),
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
        "#" => { if exp_args.iter().any(|a| a==&(1.0 as f64)) { Some(&1.0) }else {
            None
        } },
        "&" => { if exp_args.iter().all(|a| a == &(1.0 as f64)) { Some(&1.0) } else { None } }
        "!!" => { if exp_args[0] == 1.0 { None } else {if exp_args[0] == 0.0 { Some(&1.0) } else { Some(&1.0) } } }
        _ => None,
    };
    if temp_res.is_some() {
        1.0
    } else {
        0.0
    }
}
