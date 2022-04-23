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
        _ => None,
    };
    if temp_res.is_some() {
        1.0
    } else {
        0.0
    }
}
