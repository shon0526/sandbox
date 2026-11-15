use std::convert::Infallible;

fn main() {
    let p: Infallible = loop {
        println!("never fail but print forever")
    };
}
