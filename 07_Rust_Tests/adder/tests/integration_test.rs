use adder;

mod common;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two2(2));
}

#[test]
fn it_adds_two2() {
    common::setup();
    assert_eq!(4, adder::add_two2(2));
}
