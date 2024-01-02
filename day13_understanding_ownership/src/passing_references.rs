fn main(){
    let string = String::from("hello");
    println!("{:p}",string.as_ptr());
    foo(&string);
    println!("{:p}", string.as_ptr());
}

fn foo(string:&String){
    println!("{:p}", string.as_ptr());
    bar(&string);
}

fn bar(string:&String){
    println!("{:p}", string.as_ptr());
    baz(&string);
}

fn baz(string:&String){
    println!("{:p}", string.as_ptr());
}