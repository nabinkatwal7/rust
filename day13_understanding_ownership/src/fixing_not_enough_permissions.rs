fn main(){
    let name = vec![String::from("Ferris")];
    let first = &name[0];
    stringify_name_with_title(&name);
    println!("{}", first);
}

//problem
// fn stringify_name_with_title(name: &Vec<String>)->String{
//     name.push(String::from("Esq."));
//     let full = name.join(" ");
//     full
// }

fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}