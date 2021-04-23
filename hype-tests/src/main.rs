use hype_rs::prelude::*;

#[derive(AsFeatureVector)]
#[discriminant_type]
enum EnumValue {
    #[discriminant_value = 1]
    Foo {
        a: i32,
        b: f32,
        c: char,
    },
    Bar(i32, f32, char),
}

#[derive(AsFeatureVector)]
struct StructValue {
    a: i32,
    b: f32,
    c: char,
}

fn main() {}
