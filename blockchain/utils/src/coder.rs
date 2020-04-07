use bincode;
use serde::{Deserialize, Serialize};
use crypto::sha3::Sha3;
use crypto::digest::Digest;

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

pub fn get_hash(value: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result_str()
}

// 测试
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
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
        assert_eq!(de, point);
    }
}