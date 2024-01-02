fn main(){

}

// problem
// fn add_big_strings(dst: &mut Vec<String>, src: &[String]){
//     let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();

//     for s in src{
//         if s.len() >largest.len(){
//             dst.push(s.clone())
//         }
//     }
// }

fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: String = dst.iter().max_by_key(|s| s.len()).unwrap().clone();
    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone());
        }
    }
}

fn add_big_strings1(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
    let to_add: Vec<String> =
        src.iter().filter(|s| s.len() > largest.len()).cloned().collect();
    dst.extend(to_add);
}

fn add_big_strings2(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}
