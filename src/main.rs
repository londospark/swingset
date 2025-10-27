#![warn(clippy::pedantic)]
use ctor::ctor;

use crate::menu::{insert_into_registry, show_menu};

mod menu;

fn main() {
    show_menu();
}

// NOTE: the "register_*" now uses `insert_into_registry` instead
// as this doesn't hold a lock or a ref for longer than necessary

#[ctor]
fn register_test_function_1() {
    insert_into_registry("Test Function 1", test_function_1);
}

#[must_use]
pub fn test_function_1() -> String {
    String::from("Hello folks!")
}

#[ctor]
fn register_test_function_2() {
    insert_into_registry("Test Function 2", test_function_2);
}

#[must_use]
pub fn test_function_2() -> String {
    String::from("Hello folks from test 2!")
}
