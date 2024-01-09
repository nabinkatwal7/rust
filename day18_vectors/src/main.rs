enum SpreadsheetCell{
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    // let mut v:Vec<i32>= Vec::new();
    // v.push(4);
    // v.push(5);
    // v.push(6);
    // v.push(7);

    let row = vec![SpreadsheetCell::Int(3),SpreadsheetCell::Text(String::from("blue")),SpreadsheetCell::Float(10.12)];


    let v = vec![1,2,3,4,5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    for n_ref in &v{
        let n_plus_one:i32 = *n_ref+1;
        println!("{}",n_plus_one);
    }
}
