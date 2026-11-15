fn diverges() -> ! {
    panic!("This function never returns");
}
fn print_forever() -> ! {
    loop {
        println!("永遠に。。。");
    }
}

fn answer() -> Result<i32, Error> {
    Ok(1)
}

enum Error {}

fn main() {
    let result: i32 = match answer() {
        Ok(v) => v,
        Err(err) => panic!("unexpected error"),
    };
    println!("{}", result);
}
