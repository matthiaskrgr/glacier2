use serde::Deserialize;

fn main() {
    let variant: MyEnum = serde_json::from_str("some_string").unwrap();
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum MyEnum {
    Variant1 { my_string: String },
    Variant2 { first_string: String, second_string: String }
}

#[derive(Deserialize, Debug)]
struct TestStruct;

#[derive(Deserialize, Debug)]
struct TestStruct2;
