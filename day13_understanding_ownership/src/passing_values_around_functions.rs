fn main() {
    let string = String::from("hello");
    println!("{:p}", string.as_ptr());
    let string = foo(string);
    println!("{:p}", string.as_ptr());
}

fn foo(string: String)->String{
    println!("{:p}", string.as_ptr());
    return string;
}