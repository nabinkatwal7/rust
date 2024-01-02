// when passing references, there is another condition what can cause bugs called dangling references.
// Dangling references are pointers to data that has been deallocated. This is a common source of bugs in Rust.
//THIS CODE WILL NOT COMPILE
// fn main(){
//     let string = foo();
// }
//
// fn foo()->&String{
//     let string = String::from("hello");
//     println!("{}", string);
//     &string
// }