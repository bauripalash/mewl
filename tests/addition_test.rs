mod common;

#[test]
fn with_multiplication_test() {
    assert_eq!(2.0, common::get_return_value("[+ mew [* mew mew]]"));
    assert_eq!(
        10.0,
        common::get_return_value("[* mewmew [+ mew [* mewmew mewmew] ]]")
    );
    assert_eq!(5.0, common::get_return_value("[+ mew [* mewmew mewmew]]"));
}

#[test]
fn addition_test() {
    assert_eq!(
        13.0,
        common::get_return_value("[+ mewmew.mew.mewmew [` mew mew]]").floor()
    );
    assert_eq!(
        112.0,
        common::get_return_value("[+ mew [` mew [` mew mew]]]")
    );
    assert_eq!(112.0, common::get_return_value("[+ mew [` mew mew mew]]"));
    assert_eq!(3.0, common::get_return_value("[+ [+ [mew mew]] mew]"));
    assert_eq!(3.0, common::get_return_value("[+ mew mew mew]"));
    assert_eq!(2.0, common::get_return_value("[+ mew mew]"))
}
