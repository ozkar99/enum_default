extern crate enum_default;
use enum_default::EnumDefault;

#[derive(EnumDefault, PartialEq)]
enum TestEnum {
  First,
  Second,
}


fn main() {
  if TestEnum::default() == TestEnum::First {
    println!("It's the first item")
  }
}
