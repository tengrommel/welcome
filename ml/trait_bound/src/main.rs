trait GetName {
    fn get_name(&self) -> &String;
}

trait GetAge{
    fn get_age(&self) -> u32;
}

fn print_information<T: GetAge + GetName>(item: T) {
    println!("name = {}", item.get_name());
    println!("name = {}", item.get_age());
}

struct Student {
    pub name: String,
    pub age: u32
}

impl GetName for Student {
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl GetAge for Student {
    fn get_age(&self) -> u32{
        self.age
    }
}

fn produce_item_with_age() -> impl GetAge {
    Student {
        name: String::from("小明"),
        age: 12
    }
}

fn main() {
    let s = Student{
        name: "小红".to_string(),
        age: 23,
    };
    print_information(s);
}
