use super::*;
pub struct Blockchain {
    pub blocks: Vec<Block>,
}


    
impl Blockchain {
    pub fn verify(&self) -> bool {

        //Iterate through all blocks from start
        for (i, block) in self.blocks.iter().enumerate() {
            
            //Index mismatch control
            if block.index != i as u32 {return false;}

            //Check if hash matches block's instance values
            if !block::check_difficulty(&block.hash(),block.difficulty) {return false;}

            if block.index==0 {
                if block.prev_block_hash != vec![0; 32]  {return false;}
            }
            else{
                let prev_block = &self.blocks[i-1];
                if block.timestamp <= prev_block.timestamp {return false;}
                if block.prev_block_hash != prev_block.hash {return false;}
            }

        }
        return true;
    }
}