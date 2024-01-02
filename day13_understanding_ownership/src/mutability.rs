fn main(){
    let mut string = String::from("hello");
    println!("{:p}", string.as_ptr());
    foo(&mut string);
    println!("{:p}", string.as_ptr());
}

fn foo(string: &mut String){
    println!("{:p}", string.as_ptr());
    string.push_str(".")
}