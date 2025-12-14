use blockchainlib::{Block, Hashable};

fn main() {
    let mut block = Block::new(0, 1, vec![0; 32], 0, "Genesis block".to_owned());

    let h = block.hash();
    println!("{:?}", &h);
    block.hash = h; // ownership transferred

    println!("{:?}", &block);
}
