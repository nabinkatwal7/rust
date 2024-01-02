mod giving_ownership;
mod passing_values_around_functions;
mod passing_references;
mod mutability;
mod dangling_references;
mod fixing_reference_to_the_stack;
mod fixing_not_enough_permissions;
mod aliasing_and_mutating_a_data_structure;

fn main() {
    let string = "Hello, Foo!";
    let bar_string = String::from("Hello, Bar!");
    // copy_value();
    // move_value();
    // clone_value();
    foo(string);
    bar(bar_string.clone());

}

fn foo(string: &str) {
    println!("{}", string);
}

fn bar(string: String) {
    println!("{:p}", string.as_ptr());
}

// fn copy_value() {
//     let hello = "Hello, World!"; // string literal
//     let hello1 = hello; // copy the value of hello and bind it to hello1
//     println!("{}", hello1);
// }

//double free explained
// This means that the program can behave completely arbitrarily and all bets are off about what happens.
// That’s certainly a bad thing to have happen! In practice, double-freeing a block of memory will corrupt the state of the memory manager,
// which might cause existing blocks of memory to get corrupted or for future allocations to fail in bizarre ways

//this function does not compile because hello is moved, making 2 owners pointing to the same value
// This violates rule #2: “There can only be one owner at a time”
// fn move_value(){
//     let hello = String::from("Hello, World!");//String type
//     let hello1 = hello; //move the data of hello into hello1
//     println!("{}", hello);
// }

// fn clone_value(){
//     let hello = String::from("Hello, World!"); //String type
//     let hello1 = hello.clone(); //clone the data of hello into hello1
//     println!("{}", hello);
// }

