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

fn main() {
    if TestEnum::default() == TestEnum::First {
        println!("It's the first item")
    }

    if TestEnum2::default() == TestEnum2::Second {
        println!("It's the second item")
    }
}
