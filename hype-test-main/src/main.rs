use hype_rs::prelude::*;

#[derive(AsFeatureVector)]
pub enum MyType<T> {
    Struct {
        x: f32,
        det: T,
    },
    Tuple(f32, f32, T),
    Unit,
}

fn main() {
    let x = MyType::Struct { x: 1.0, det: 2 };
    println!("{:?}", x.as_feature_vector());
    let x = MyType::Tuple(1.0, 2.0, 'c');
    println!("{:?}", x.as_feature_vector());
    let x = MyType::<char>::Unit;
    println!("{:?}", x.as_feature_vector());
}
