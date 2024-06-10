use super::*;

#[test]
fn test_read_from_reg() {
    let result = read_from_reg();
    let datas: &Vec<Data> = result.as_ref().clone().unwrap();
    assert_ne!(datas[0].label, "");
    assert!(result.is_ok());
}
