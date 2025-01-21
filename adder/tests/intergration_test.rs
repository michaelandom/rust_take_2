use adder;

#[test]

fn in_test_for_search() {
    let name = String::from("crog");
    assert!(adder::greeting(&name).contains("crog"), "name not found {}", name)
}
