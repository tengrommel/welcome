pub trait GetInformation {
    fn get_name(&self) -> &String;
    fn get_age(&self) -> u32;
}

pub struct Student {
    pub name: String,
    pub age: u32
}

impl GetInformation for Student {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_age(&self) -> u32 {
        *&self.age
    }
}

pub struct Teacher {
    pub name: String,
    pub age: u32,
    pub subject: String
}

impl GetInformation for Teacher {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_age(&self) -> u32 {
        *&self.age
    }
}

// trait bounds 指定范围
// trait作为参数

fn get_information(item: impl GetInformation) {
    println!("name = {}", item.get_name());
    println!("age = {}", item.get_age());
}

fn main() {
    let s = Student{name: String::from("小明"), age: 10};
    let t = Teacher{name: String::from("小黄"), age: 10, subject: String::from("数学")};
    println!("student, name = {}, age = {}", s.get_name(), s.get_age());
    println!("teacher, name = {}, age = {}", t.get_name(), t.get_age());
    get_information(s);
    get_information(t);
}
