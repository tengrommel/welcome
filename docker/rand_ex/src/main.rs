use rand::prelude::*;

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
}

fn compute_mean(n: &Vec<usize>) {
    let mut sum: usize = 0;
    for i in n {
        sum += i; // 自动解引用
    }
    let median = sum/n.len();
    print!("median: {}", median);
}

