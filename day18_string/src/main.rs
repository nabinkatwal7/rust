fn main() {
    let mut s = String::from("initial contents");
    s.push_str("bar");
    println!("s: {}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s: {}", s);
}
