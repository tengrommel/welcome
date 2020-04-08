unsafe trait Foo {
    fn foo(&self);
}

struct Bar();

unsafe impl Foo for Bar {
    fn foo(&self) {
        println!("foo");
    }
}

// pub trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

pub trait my_iterator<T> {
    fn next(&mut self) -> Option<T>;
}

struct A {
    value: i32
}

impl my_iterator<i32> for A {
    fn next(&mut self) -> Option<i32> {
        println!("in i32");
        if self.value > 3 {
            self.value += 1;
            Some(self.value)
        } else {
            None
        }
    }
}

impl my_iterator<String> for A {
    fn next(&mut self) -> Option<String> {
        println!("in i32");
        if self.value > 3 {
            self.value += 1;
            Some(self.value.to_string())
        } else {
            None
        }
    }
}

trait AB {
    fn print(&self);
}

trait B {
    fn print(&self);
}

struct MyType;

impl AB for MyType {
    fn print(&self) {
        println!("AB trait for MyType");
    }
}

// impl B for MyType {
//     fn print(&self) {
//         println!("B trait for MyType");
//     }
// }
//
// impl MyType {
//     fn print(&self) {
//         println!("MyType");
//     }
// }


fn main() {
    let a = Bar();
    a.foo();
    let mut a = A{
        value: 3
    };
    <A as my_iterator<i32>>::next(&mut a);
    let my_type = MyType;
    my_type.print();
    AB::print(&my_type);
    println!("Hello, world!");
}
