use std::io;
use std::collections::HashMap;

fn main() {
    let temp: Vec<usize> = get_line()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let (n, k) = (temp[0], temp[1]);
    let mut nums: Vec<i32> = get_line()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    max_perm(&mut nums, n, k);
    let result_str = nums.iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", result_str);
}

fn max_perm(nums: &mut Vec<i32>, n: usize, max_swaps: usize) {
    let mut val_to_index: HashMap<i32, usize> = HashMap::new();
    for i in 0..(nums.len()) {
        val_to_index.insert(nums[i], i);
    }

    let mut num_swaps = 0;
    let mut i = 0usize;
    while num_swaps < max_swaps && i < nums.len() - 1 {
        let desired_val = (n - i) as i32;
        if nums[i] != (desired_val) {
            let swap_index = val_to_index[&desired_val];
            swap(nums, i, swap_index);
            *val_to_index.get_mut(&desired_val).unwrap() = i;
            *val_to_index.get_mut(&nums[swap_index]).unwrap() = swap_index; 
            num_swaps += 1;
        }
        i += 1;
    }
}

fn swap(nums: &mut Vec<i32>, i: usize, j: usize) {
    let temp = nums[j];
    nums[j] = nums[i];
    nums[i] = temp;
}

fn get_line() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s
}
