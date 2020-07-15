use automated_tests;

mod common;

// #[cfg(test)] not needed

// Only runs when all unit_test succeed
// or
// $ cargo test --test integration_test
#[test]
fn it_adds_two_integration() {
    common::setup();
    assert_eq!(4, automated_tests::add_two(2));
}
