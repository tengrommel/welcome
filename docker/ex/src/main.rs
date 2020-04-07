use std::collections::{VecDeque, HashMap, BTreeMap, HashSet, BTreeSet};

#[derive(Debug, PartialEq)]
struct People {
    name: &'static str,
    gender: u32
}

impl People {
    fn new(name: &'static str, gender: u32) -> Self {
        return People{
            name,
            gender
        }
    }
    fn name(&self) {
        println!("name: {:?}", self.name);
    }
    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }
    fn gender(&self) {
        let gender = if self.gender == 1 {"boy"} else {"girl"};
        println!("gender: {}", gender);
    }
}

fn main() {
    let alex = People::new("Alex", 1);
    alex.name();
    alex.gender();
    assert_eq!(alex, People{name:"Alex", gender: 1});
    let mut alice = People::new("Alice", 0);
    alice.name();
    alice.gender();
    assert_eq!(alice, People{name: "Alice", gender: 0});
    alice.set_name("Rose");
    assert_eq!(alice, People{name: "Rose", gender: 0});

    let mut v1 = vec![];
    v1.push(1);
    v1.push(2);
    v1.push(3);
    assert_eq!(v1, [1, 2, 3]);
    let mut buf = VecDeque::new();
    buf.push_front(1);
    buf.push_front(2);
    assert_eq!(buf.get(0), Some(&2));
    assert_eq!(buf.get(1), Some(&1));
    buf.push_back(3);
    assert_eq!(buf.get(2), Some(&3));
    let mut hmap = HashMap::new();
    let mut bmap = BTreeMap::new();
    hmap.insert(3, "c");
    hmap.insert(1, "a");
    hmap.insert(2, "b");
    hmap.insert(5, "e");
    bmap.insert(4, "a");
    println!("{:?}", hmap);
    println!("{:?}", bmap);
    let mut hbooks = HashSet::new();
    let mut bbooks = BTreeSet::new();
    hbooks.insert("A Song of Ice and Fire");
    bbooks.insert("A Song of Ice and Fire");
}
