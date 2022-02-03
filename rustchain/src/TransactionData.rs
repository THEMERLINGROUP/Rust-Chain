use super::*;
use std::collections::HashSet;
#[derive(Clone)]
pub struct Output {
    pub to_adddr: Address,
    pub val: u64,
}
impl Hashable for Output {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.to_adddr.as_bytes());
        bytes.extend(&u64_bytes(&self.val));
        bytes
    }
}
pub struct Transactions {
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>
}
impl Transactions {
    pub fn input_value (&self) -> u64 {
        self.inputs.iter().map(|input|input.val).sum()
    }
    pub fn output_value (&self) -> u64 {
        self.inputs.iter().map(|output|output.val).sum()
    }
    pub fn input_hashes (&self) -> HashSet<Hash> {
        self.inputs.iter().map(|input|input.hash()).collect::<HashSet<Hash>>()
    }
    pub fn output_hashes (&self) -> HashSet<Hash> {
        self.outputs.iter().map(|output|output.hash()).collect::<HashSet<Hash>>()
    }
}
impl Hashable for Transactions {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.inputs.iter().flat_map(|input|input.bytes()).collect::<Vec<u8>>());
        bytes.extend(self.outputs.iter().flat_map(|output|output.bytes()).collect::<Vec<u8>>());
     
        bytes
    }
    
}