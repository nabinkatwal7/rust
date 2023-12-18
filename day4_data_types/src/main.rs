fn main() {
    // Integer type
    let x: i32 = 10;
    println!("x = {}", x);

    // Float type
    let x = 10.3; // f64
    let y:f32 = 3.2; // f32

    println!("x = {}, y = {}", x, y);

    //addition
    let sum = 5 + 10;
    println!("5 + 10 = {}", sum);

    //subtraction
    let diffrence = 10.5 - 5.3;
    println!("10.5 - 5.3 = {}", diffrence);

    //multiplication
    let product = 4 * 30;
    println!("4 * 30 = {}", product);

    //division
    let quotient = 56.7 / 32.2;
    let truncated = 56.7 % 32.2;
    println!("56.7 / 32.2 = {}", quotient);
    println!("56.7 % 32.2 = {}", truncated);

    //remainder
    let remainder = 43 % 5;
    println!("43 % 5 = {}", remainder);

    // the boolean type 
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("t = {}, f = {}", t, f);

    // the character type 
    let c = 'z';
    let z:char = 'Z'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("c = {}, z = {}, heart_eyed_cat = {}", c, z, heart_eyed_cat);

    //Compound type
    // the tuple type
    let tup:(i32, f64, u8) = (500, 6.4, 1);
    println!("tup = {:?}", tup);
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("five_hundred = {}, six_point_four = {}, one = {}", five_hundred, six_point_four, one);

    // the array type
    let a = [1,2,3,4,5];
    println!("a = {:?}", a);
    let months = ["January", "February", "March", "April", "May", "June", "July","August", "September", "October", "November", "December"];
    println!("months = {:?}", months);
}
