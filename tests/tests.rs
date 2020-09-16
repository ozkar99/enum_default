#![allow(dead_code)]

extern crate enum_default;
use enum_default::EnumDefault;

#[test]
pub fn it_derives_default_for_enum_from_first_variant() {
    #[derive(EnumDefault, PartialEq)]
    enum TestEnum {
        A,
        B,
    }

    assert!(TestEnum::default() == TestEnum::A);
}

#[test]
pub fn it_derives_default_for_enum_from_explicit_variant() {
    #[derive(EnumDefault, PartialEq)]
    enum TestEnum {
        A,
        #[default]
        B,
    }

    assert!(TestEnum::default() == TestEnum::B);
}

#[test]
pub fn it_derives_default_for_enum_with_generics_from_first_variant() {
    #[derive(EnumDefault, PartialEq)]
    enum TestEnum<T> {
        A,
        B(T),
    }

    assert!(TestEnum::<()>::default() == TestEnum::A);
}

#[test]
pub fn it_derives_default_for_enum_with_generics_from_explicit_variant() {
    #[derive(EnumDefault, PartialEq)]
    enum TestEnum<T> {
        A(T),
        #[default]
        B,
    }

    assert!(TestEnum::<()>::default() == TestEnum::B);
}
