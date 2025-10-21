#![warn(clippy::pedantic)]
use ctor::ctor;

use crate::menu::{get_registry, show_menu};
mod menu;

fn main() {
    show_menu();
}

#[ctor]
fn register_test_function_1() {
    get_registry().insert("Test Function 1", test_function_1);
}
pub fn test_function_1() -> String {
    String::from("Hello folks!")
}

#[ctor]
fn register_test_function_2() {
    get_registry().insert("Test Function 2", test_function_2);
}
pub fn test_function_2() -> String {
    String::from("Hello folks from test 2!")
}
