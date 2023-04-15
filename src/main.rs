pub struct Blockchain {
    pub blocks: Vec,
}

pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub data: String,
    pub nonce: u64,
}

const DIFFICULTY_LEVEL: &str = "00";

fn convert_hash_to_binary(hash: &[u8]) -> String {
    let mut result: String = String::default();
    for i in hash {
        result.push_str(&format!("{:b}", i));
    }
    result
}

// TODO: fn - Litecoin_hash for calculating hash.

impl Blockchain{
    fn new() -> Self{
        Self { blocks: vec![]}
    }

    fn genesis(&mut self){
        let genesis_block = Block {
            id: 0,
            timestamp: Utc::now().timestamp(),
            previous_hash: String::from("CodeX"),
            data: String::from("CodeX!"),
            nonce: 2023,
            hash: "08aaa69a539990898e95f843c04657531d79df9de0beafed85ae4e2e7df22636".to_string(),
        };
        self.blocks.push(genesis_block)
    }

    fn is_block_valid(&self, block: &Block, previous_block: &Block) -> bool {
        if block.previous_hash != previous_block.hash {
            warn!("Incorrect: Block with id- {} has wrong hash.", block.id);
            return false;
        } else if !convert_hash_to_binary(
            &hex::decode(&block.hash).expect("Unable to decode from hex"),
        )
        .starts_with(DIFFICULTY_LEVEL)
        {
            warn!("Block with id: {} has invalid difficulty level", block.id);
            return false;
        } else if block.id != previous_block.id + 1 {
            warn!(
                "Block with id: {} is not the next block after the latest: {}",
                block.id, previous_block.id
            );
            return false;
            
            // calculate hash -> Litecoin_hash

        } else if hex::encode(calculate_hash(
            block.id,
            block.timestamp,
            &block.previous_hash,
            &block.data,
            block.nonce,
        )) != block.hash
        {
            warn!("Block with id: {} has invalid hash", block.id);
            return false;
        }
        true
    }

    fn is_chain_valid(&self, chain: &[Block]) -> bool {
        for i in 0..chain.len() {
            if i == 0 {
                continue;
            }
            let first = chain.get(i - 1).expect("Has to exist");
            let second = chain.get(i).expect("Has to exist");
            if !self.is_block_valid(second, first) {
                return false;
            }
        }
        true
    }
    
    fn select_chain(&mut self, local: Vec, remote: Vec) -> Vec {
        let is_local_valid = self.is_chain_valid(&local);
        let is_remote_valid = self.is_chain_valid(&remote);
        
        if is_local_valid && is_remote_valid {
            if local.len() >= remote.len() {
                local
            } else {
                remote
            }
        } else if is_remote_valid && !is_local_valid {
            remote
        } else if !is_remote_valid && is_local_valid {
            local
        } else {
            panic!("Local and remote chains are invalid");
        }
    }

}