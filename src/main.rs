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
    // max_perm(&mut nums, k);
    max_perm2(&mut nums, n, k);
    let result_str = nums.iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", result_str);
}

fn max_perm2(nums: &mut Vec<i32>, n: usize, max_swaps: usize) {
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
            *val_to_index.entry(desired_val).or_insert(0) = i;
            *val_to_index.entry(nums[swap_index as usize]).or_insert(0) = swap_index; 
            num_swaps += 1;
        }
        i += 1;
    }
}

fn max_perm(nums: &mut Vec<i32>, max_swaps: usize) {
    let mut num_swaps = 0;
    let mut i = 0usize;
    while num_swaps < max_swaps && i < nums.len() - 1 {
        let mut max_index = i + 1;
        for j in i..nums.len() {
            if nums[j] > nums[max_index] {
                max_index = j;
            }
        }
        if nums[max_index] > nums[i] {
            num_swaps += 1;
            swap(nums, max_index, i);
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
