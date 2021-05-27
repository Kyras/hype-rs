#![feature(async_closure)]

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

#[tokio::main]
async fn main() {
    let x = async || {
        println!("Calling first closure");
        tokio::time::sleep(std::time::Duration::new(2, 0)).await;
    };

    let y = || async {
        println!("Calling second closure");
        tokio::time::sleep(std::time::Duration::new(2, 0)).await;
    };

    let x1 = x();
    println!("Created first closure");
    x1.await;
    println!("First closure done");


    println!("------------------");

    let y1 = y();
    println!("Created second closure");
    y1.await;
    println!("Second closure done");
}
