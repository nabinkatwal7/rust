fn main() {
    let mut x = 5;
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");
    println!("{x}");
    x = 6;
    println!("{x}");
}
