fn main(){
    let string = foo();
    println!("{:p}", string.as_ptr());
}

fn foo() -> String {
    let string = String::from("Hello, World!");
    println!("{:p}", string.as_ptr());
    string
}