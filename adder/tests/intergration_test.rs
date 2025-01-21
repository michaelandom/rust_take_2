use adder;
mod common;
#[test]
fn in_test_for_search() {
    common::set_up();
    let name = String::from("crog");
    assert!(adder::greeting(&name).contains("crog"), "name not found {}", name)
}

