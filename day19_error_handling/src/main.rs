use std::fs::File;
use std::io::ErrorKind;

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    // panic!("Error occurred, Quit!")
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = greeting_file_result.unwrap_or_else(|error| match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating the file: {:?}", e),
        },
        other_error => {
            panic!("Problem opening the file: {:?}", other_error)
        }
    });

}
