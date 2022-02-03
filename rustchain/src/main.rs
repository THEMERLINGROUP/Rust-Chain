use blockchainlib::*;
fn main() {
    let difficulty = 0x0002ba87e65f73e46cc95a0f25fe8f1d;
    let mut genesis_block = Block::new(0,now(), vec![0; 32], vec![
        Transactions {
            inputs: vec![],
            outputs: vec![TransactionData::Output {
                to_adddr: "John".to_owned(),
                val: 13,
            },
            TransactionData::Output {
                to_adddr: "Tim".to_owned(),
                val: 3, 
            },
        ],
    },
],difficulty);
genesis_block.mine();
println!("Mined genesis block {:?}", &genesis_block);
let mut last_hash = genesis_block.hash.clone();
let mut blockchain = Blockchain::new();
blockchain.update_chain(genesis_block).expect("Failed to add genesis block");
let mut block = Block::new(1, now(), last_hash, vec![
    Transactions {
            inputs: vec![blockchain.blocks[0].transactions[0].outputs[0].clone(), ],
        outputs: vec![TransactionData::Output {
            to_adddr: "Jasmine".to_owned(),
            val: 7,
        },
        TransactionData::Output {
            to_adddr: "Rob".to_owned(),
            val: 3, 
        },
    ],
},   
], difficulty);

block.mine();

println!("Mined block {:?}", &block);

last_hash = block.hash.clone();

blockchain.update_chain(block).expect("Failed to add genesis block");

}