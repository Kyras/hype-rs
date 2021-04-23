use hype_rs::prelude::*;

#[derive(AsFeatureVector)]
#[discriminant_type(u32)]
enum Value {
    #[discriminant_value = 1]
    Foo {
        a: i32,
        b: f32,
        c: char,
    },
    Bar(i32, f32, char),
}

fn main() {
    let val = Value::Foo {
        a: 10,
        b: 10.2,
        c: 'a',
    };

    println!("{}", val.feature_size());
    println!("{:?}", val.as_feature_vector());
}
