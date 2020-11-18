use beepboop::fileparser;
use beepboop::types;

#[test]
fn parse_num_test_ok() {
    let test_str = "beep boop beep beep boop";
    let test_str_iter = test_str.split_whitespace();
    assert_eq!(fileparser::parse_num(test_str_iter),Ok(22))
}
#[test]
fn parse_num_test_err() {
    let test_str = "beep beep bo";
    let test_str_iter = test_str.split_whitespace();
    assert_eq!(fileparser::parse_num(test_str_iter),Err(types::BeepboopError::ParseError))
}
