fn main() {
    let offers = ["a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five golden rings",
        "six geese-a-laying",
        "seven swans-a-swimming",
        "eight maids-a-milking",
        "nine Ladies Dancing",
        "ten Lords-a-Leaping",
        "eleven pipers piping",
        "twelve drummers drumming"];

    for i in 0..offers.len() {
        println!("On the {}th day of Christmas my true love gave to me {}, {} {}", i,offers[i], if i == 0 {","} else {"and"}, if i == 0 {"a Partridge in a Pear Tree"} else {offers[i-1]});
    }
}
