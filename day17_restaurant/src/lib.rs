use front_of_house::hosting;
mod front_of_house;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn eat_at_restaurant(){
    hosting::add_to_waitlist();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
