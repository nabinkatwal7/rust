fn main() {
    println!("Enter the nth term of sequence");
    let mut number = String::new();
    std::io::stdin().read_line(&mut number).expect("Failed to read line");
    let number: i64 = number.trim().parse().expect("Please type a number");
    fibonacci(number);
}

fn fibonacci(number:i64){
    let mut a = 0;
    let mut b = 1;
    let mut c;
    println!("{a}");
    println!("{b}");
    for _ in 0..number{
        c = a + b;
        a = b;
        b=c;
        println!("{c}");
    }
}