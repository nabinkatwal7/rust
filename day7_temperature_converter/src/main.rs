fn main() {
    println!("Please enter a temperature in Celsius:");
    let mut celsius = String::new();
    let mut farenheit = String::new();
    std::io::stdin().read_line(&mut celsius).expect("Failed to read line");
    let celsius: f64 = celsius.trim().parse().expect("Please type a number!");
    println!("Temperature in Farenheit is {} degrees", convert_to_fahrenheit(celsius));
    println!("Please enter a temperature in Farenheit:");
    std::io::stdin().read_line(&mut farenheit).expect("Failed to read line");

    let farenheit: f64 = farenheit.trim().parse().expect("Please type a number!");
    println!("Temperature in Celsius is {} degrees", convert_to_celsius(farenheit));
}

fn convert_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn convert_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}