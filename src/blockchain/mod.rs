pub mod block;
use block::Block;

pub struct Chain {
    pub blocks: Vec<Block>,
    pub difficulty: usize,
}

impl Chain {
    pub fn new(difficulty: usize) -> Self {
        let genesis = Block::new(0, "Genesis Block".to_string(), "0".repeat(64));
        Self {
            blocks: vec![genesis],
            difficulty,
        }
    }

    pub fn add_block(&mut self, data: String) {
        let prev_hash = self.blocks.last().unwrap().hash.clone();
        let mut new_block = Block::new(self.blocks.len() as u32, data, prev_hash);
        new_block.mine(self.difficulty);
        self.blocks.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.blocks.size() {
            let current = &self.blocks[i];
            let prev = &self.blocks[i-1];
            if current.hash != current.calculate_hash() || current.previous_hash != prev.hash {
                return false;
            }
        }
        true
    }
}