use fileparser;

#[test]
fn parse_num_test_ok() {
    let test_str = "beep boop beep beep boop";
    let test_str_iter = test_str.split_whitespace();
    let result = parse_num(test_str_iter)?;
    assert_eq!(result,22)
}
