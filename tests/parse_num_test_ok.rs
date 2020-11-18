use beepboop::fileparser;

#[test]
fn parse_num_test_ok() {
    let test_str = "beep boop beep beep boop";
    let test_str_iter = test_str.split_whitespace();
    assert_eq!(fileparser::parse_num(test_str_iter),Ok(22))
}
