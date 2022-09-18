use blockchainlib::*;

fn main(){
    let difficulty = 0x000fffffffffffffffffffffffffffff;
    let mut block = Block::new(0,now(),vec![0; 32], 0, "Genesis block".to_owned(), difficulty);

    block.hash = block.hash();
    
    println!("{:?}", &block);

    block.mine();
    println!("{:?}", &block);

    let mut prev_hash = block.hash().clone();
    let mut blockchain = Blockchain {
        blocks: vec![block]
    };

    for i in 1..=10 {
        let mut block = Block::new(i,now(),prev_hash, 0, "Genesis block".to_owned(), difficulty);
        
        block.mine();
        println!("{:?}", &block);
        prev_hash = block.hash.clone();
        blockchain.blocks.push(block);
    }

    println!("{}", &blockchain.verify());

    blockchain.blocks[1].hash[3] += 1;
    println!("{}", &blockchain.verify());

}