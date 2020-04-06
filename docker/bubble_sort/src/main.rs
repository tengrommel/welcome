fn main() {
    let mut list_nums:[i32;7] = [10, 23, 4, 5, 66, 7, -1];
    println!("{:?}", list_nums);
    // rev()反序
    // for list_num in list_nums.iter().rev() {
    //     println!("{}", list_num);
    // }
    for i in 0..list_nums.len() {
        for j in  ((i+1)..list_nums.len()).rev(){
            // println!("i={}, j={}",i, j);
            if list_nums[j-1] > list_nums[j] {
                swap_us(&mut list_nums, j-1, j);
            }
            println!("{:?}", list_nums);
        }
    }
}

fn swap_us(m_list: &mut [i32; 7], i: usize, j: usize) {
    let temp: i32;
    temp = m_list[i];
    m_list[i] = m_list[j];
    m_list[j] = temp;
}