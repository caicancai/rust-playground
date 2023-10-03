#![allow(unused_variables)] // TODO(you): remove this lint after implementing this mod
#![allow(dead_code)] // TODO(you): remove this lint after implementing this mod

use super::Block;

/// Builds a block.
pub struct BlockBuilder {
    offsets: Vec<u16>,

    data: Vec<u8>,

    block_size: usize,
}

impl BlockBuilder {
    /// Creates a new block builder.
    pub fn new(block_size: usize) -> Self {
        Self {
            offsets:    Vec::new(),
            data:       Vec::new(),
            block_size,
        }
    }

    /// Adds a key-value pair to the block. Returns false when the block is full.
    #[must_use]
    pub fn add(&mut self, key: &[u8], value: &[u8]) -> bool {
        assert!(!key.is_empty(), "key must not be empty");

        if self.estimated_size() + key_len() + value.len() + SIZEOF_u16 * 3 > self.block_size && !self.is_empty(){
            return false;
        }
        self.offsets.push(self.data.len() as u16);
        self.data.put_u16(key.len() as u16);
        self.data.put(key);
        self.data.put_u16(value.len() as u16);
        self.data.put(value);
        true
    }

    /// Check if there is no key-value pair in the block.
    pub fn is_empty(&self) -> bool {
        self.offset.is_empty()
    }

    /// Finalize the block.
    pub fn build(self) -> Block {
        if self.is_empty(){
            panic!("block should not be empty");
        }
        Block {
            data:    self.data,
            offsets: self.offsets,
        }
    }
}
