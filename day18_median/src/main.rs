// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.
fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6];
    let (median, mode) = median_and_mode(&nums);
    println!("Median: {}, Mode: {}", median, mode);
}

fn median_and_mode(nums: &Vec<i32>) -> (i32, i32) {
    let mut median = 0;
    let mut mode = 0;
    median = ((nums.len() +1) / 2) as i32;
    mode = nums[median as usize];
    (median, mode)
}