use super::*;

#[test]
fn test_read_from_shared_memory() {
    let result = read_from_wmi();
    let datas: &Vec<Data> = result.as_ref().clone().unwrap();
    assert_ne!(datas[0].label, "");
    assert!(result.is_ok());
}
