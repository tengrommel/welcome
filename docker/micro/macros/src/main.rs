#[macro_export]
macro_rules! my_vec { // 模仿vec!
    ($($x: expr), *) => {
    {
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )*
            temp_vec
    }
    }
}

pub trait HelloMacro {
    fn hello_macro();
}

fn main() {
    let v = my_vec![1,2,3];
    println!("v= {:?}", v);
    println!("Hello, world!");
}
