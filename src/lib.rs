extern crate rand;

pub mod map;
pub mod entity;
pub mod coordinate_utils;
pub mod update;

pub fn lib_func() -> String {
    return String::from("Hello!");
}

pub fn lib_version() -> String {
    return String::from("1.0");
}
