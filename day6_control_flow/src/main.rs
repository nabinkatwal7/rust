fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    
    if number<5{
        println!("Condition was true!");
    }else {
        println!("Condition was false");
    }

    if number != 0 {
        println!("Number was something other than zero");
    }

    if number%4==0{
        println!("number is divisible by 4");
    }else if number%3==0{
        println!("number is divisible by 3");
    }else if number%2==0{
        println!("number is divisible by 2");
    }else{
        println!("number is not divisible by 4, 3, or 2");
    }

    loop_example();
}

fn loop_example(){
    let mut counter = 0;
    let mut count = 0;
    let mut num = 3;
    let a = [10, 20, 30, 40, 50];
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter*2;
        }
    };
    println!("The result is {}", result);

    //Loop labels (new in Rust compared to other languages)
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    while num!=0 {
        println!("number is: {num}");
        num -=1;
    }

    println!("LIFTOFF!!!");

    for element in a {
        println!("the value is: {element}");
    }
}
