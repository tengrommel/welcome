use core::block_chain;
fn main() {
    let mut bc = block_chain::BlockChain::new_block_chain();
    bc.add_block("a->b: 5btc".to_string());
    bc.add_block("c->d: 1btc".to_string());
    for block in bc.blocks {
        println!("++++++++++++++++++++++++++++++++++++++++++++++++++++");
        println!("{:#?}", block);
    }
}
