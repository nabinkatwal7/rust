fn main() {
    demo_function();
    another_function(5);
    print_labeled_measurements(5, 'h');
    expression();
    println!("The value of five is: {}", five());
    println!("The value of plus one is: {}", plus_one(5));
}

fn demo_function() {
    println!("This is a demo function");
}

//passing parameter
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

//multiple parameters
fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

//expressions in rs
fn expression() {
    let x = {
        let y = 3;
        y + 1
    };
    println!("The value of x is: {}", x);
}

// return functions
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
