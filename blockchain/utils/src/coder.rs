use bincode;
use serde::{Deserialize, Serialize};

// 编译时类型不确定
pub fn my_serialize<T: ?Sized>(value: &T) -> Vec<u8>
    where T: Serialize, {
    let seialized = bincode::serialize(value).unwrap();
    seialized
}

pub fn my_deserialize<'a, T>(bytes: &'a[u8]) -> T
    where T: Deserialize<'a>
{
    let deserialized = bincode::deserialize(bytes).unwrap();
    deserialized
}

// 测试
#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use crate::coder::Point;
    use crate::coder::{my_deserialize, my_serialize};

    #[test]
    fn it_works() {
        let point = Point{x:1, y:1};
        let se = my_serialize(&point);
        let de: Point = my_deserialize(&se);
        assert_eq!(de.y, point.y);
    }
}