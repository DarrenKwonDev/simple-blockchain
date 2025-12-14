use blockchainlib::{Block, Hashable};

fn main() {
    let mut block = Block::new(
        0,
        1,
        vec![0; 32],
        0,
        "Genesis block".to_owned(),
        0x0000ffffffffffffffffffffffffffff,
    );

    block.hash = block.hash();

    println!("{:?}", &block);
    block.mine();
    println!("{:?}", &block);
}
