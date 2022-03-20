fn main() {
    println!("Hello, world!");
    calc(1,200);
}

fn calc(begin:i32, end:i32) {
    let days = end - begin;
    println!("There are {} days between {} and {}", days, begin, end);
}
