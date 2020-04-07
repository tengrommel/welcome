use chrono::prelude::*;
use utils::coder;
use serde::{Deserialize, Serialize};
// 区块头 设置序列化
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct BlockHeader {
    pub time: i64,
    pub tx_hash: String,
    pub pre_hash: String
}
// 区块
#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub hash: String,
    pub data: String,
}

impl Block {
    fn set_hash(&mut self) {
        let header = coder::my_serialize(&(self.header));
        self.hash = coder::get_hash(&header[..]);
    }
    pub fn new_block(data: String, pre_hash: String) -> Block {
        let transactions = coder::my_serialize(&data);
        let tx_hash = coder::get_hash(&transactions[..]);
        let time = Utc::now().timestamp();
        let mut block = Block{
            header: BlockHeader{
                time,
                tx_hash,
                pre_hash
            },
            hash: "".to_string(),
            data,
        };
        block.set_hash();
        block
    }
}