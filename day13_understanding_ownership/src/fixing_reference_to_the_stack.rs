use std::rc::Rc;

fn main(){
    println!("{}",return_a_string());
    println!("{}",return_a_string1());
    println!("{}",return_a_string2());
    println!("{}",return_a_string3(&output));
}

// problem
// fn return_a_string() -> &String{
//     let s = String::from("Hello World");
//     &s
// }

// solutions
fn return_a_string() -> String{
    let s = String::from("Hello World");
    s
}

fn return_a_string1() -> &'static str{
    "Hello World1"
}

fn return_a_string2() -> Rc<String>{
    let s = Rc::new(String::from("Hello World2"));
    Rc::clone(&s)
}
