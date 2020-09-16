#![allow(dead_code)]

extern crate enum_default;
use enum_default::EnumDefault;

#[derive(EnumDefault, PartialEq)]
enum TestEnum {
    First,
    Second,
}

#[derive(EnumDefault, PartialEq)]
enum TestEnum2 {
    First,
    #[default]
    Second = 1337,
}

#[derive(EnumDefault, PartialEq)]
enum TestEnum3<T> {
    A,
    B(T),
}

#[derive(EnumDefault, PartialEq)]
enum TestEnum4<T> {
    A(T),
    #[default]
    B,
}

fn main() {
    assert!(TestEnum::default() == TestEnum::First);
    assert!(TestEnum2::default() == TestEnum2::Second);
    assert!(TestEnum3::<()>::default() == TestEnum3::A);
    assert!(TestEnum4::<()>::default() == TestEnum4::B);
}
