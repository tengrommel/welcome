use rand::prelude::*;
use std::collections::HashMap;

fn main() {
    // create a rand number that represents length of the list of nums
    let mut rng = rand::thread_rng();
    let length_nums = rng.gen_range(5, 25);
    println!("length of nums: {}", length_nums);
    let mut nums: Vec<usize> = Vec::new();
    for _ in 0..length_nums {
        nums.push(rng.gen_range(0, 10));
    }
    println!("nums: {:?}", nums);
    compute_mean(&mut nums);
    compute_median(&mut nums);
    compute_mode(&nums);
}

fn compute_mode(n: &Vec<usize>) {
    let mut freq = HashMap::new();
    for i in n {
        let cnt = freq.entry(i).or_insert(0);
        *cnt += 1;
    }
    let mut max_k = 0;
    let mut max_v = 0;
    for (k, v) in &freq {
        println!("{} ----> {}", **k, *v);
        if (*v > max_v) {
            max_v = *v;
            max_k = **k;
        }
    }
    println!("max: {} -> {}", max_k, max_v);
}

fn compute_mean(n: &Vec<usize>) {
    let mut sum: usize = 0;
    for i in n {
        sum += i; // 自动解引用
    }
    let median = sum / n.len();
    print!("median: {}", median);
}

fn bubble_sort_fn(nums: &mut Vec<usize>) {
    for i in 0..nums.len() {
        for j in ((i + 1)..nums.len()).rev() {
            if nums[j - 1] > nums[j] {
                swap_us(nums, j - 1, j);
            }
        }
    }
    println!("{:?}", nums);
}

fn swap_us(m_list: &mut Vec<usize>, i: usize, j: usize) {
    let temp: usize;
    temp = m_list[i];
    m_list[i] = m_list[j];
    m_list[j] = temp;
}

fn compute_median(n: &mut Vec<usize>) {
    bubble_sort_fn(n);
    let middle_pos = n.len()/2;
    println!("median: {}", n.get(middle_pos).unwrap());
}

