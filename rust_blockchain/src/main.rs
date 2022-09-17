use blockchainlib::*;

fn main(){
    let difficulty = 0x000fffffffffffffffffffffffffffff;
    let mut block = Block::new(0,now(),vec![0; 32], 0, "Genesis block".to_owned(), difficulty);

    block.hash = block.hash();
    
    println!("{:?}", &block);

    block.mine();
    println!("{:?}", &block);
}