use std::collections::HashMap;
use std::io::Read;
use crate::bit_reader::BitReader;

#[derive(Debug)]
pub struct HuffmanNode {
    weight: u32,
    byte: u8,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

impl HuffmanNode {
    pub fn new() -> Self {
        Self {
            weight: 0,
            byte: 0b0,
            left: None,
            right: None,
        }
    }

    fn create_node(byte: u8, weight: u32) -> Self {
        let mut node = Self::new();

        node.byte = byte;
        node.weight = weight;

        node
    }

    pub fn from_frequencies(frequencies: HashMap<u8, i32>) -> Self {
        let mut freq: Vec<_> = frequencies
            .iter()
            .map(|(byte, freq)| Self::create_node(*byte, *freq as u32))
            .collect();

        freq.sort_by(|a, b| b.weight.cmp(&a.weight));

        while freq.len() > 1 {
            let left = freq.pop().unwrap();
            let right = freq.pop().unwrap();

            let mut node = Self::new();

            node.set_right(right);
            node.set_left(left);

            let index = freq
                .iter()
                .position(|r| r.weight < node.weight)
                .or_else(|| Option::from(freq.len()))
                .unwrap();

            freq.insert(index, node);
        }

        freq.pop().unwrap()
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    pub fn set_left(&mut self, left: HuffmanNode) {
        self.weight += left.weight;
        self.left = Some(Box::new(left));
    }

    pub fn set_right(&mut self, right: HuffmanNode) {
        self.weight += right.weight;
        self.right = Some(Box::new(right));
    }

    pub fn get_dictionary(&self, v: Option<u16>) -> HashMap<u8, u16> {
        let mut dictionary = HashMap::new();

        if self.is_leaf() {
            if v.is_some() {
                dictionary.insert(self.byte, v.unwrap());
            }
        }

        if self.left.is_some() {
            let left_v = if v.is_some() { v.unwrap() << 1 } else { 0 };
            let left_dictionary = self.left.as_ref().unwrap().get_dictionary(Some(left_v));
            dictionary.extend(left_dictionary);
        }

        if self.right.is_some() {
            let right_v = if v.is_some() { v.unwrap() << 1 | 1 } else { 1 };
            let right_dictionary = self.right.as_ref().unwrap().get_dictionary(Some(right_v));
            dictionary.extend(right_dictionary);
        }

        dictionary
    }

    pub fn decode_byte<R: Read>(&self, bit_reader: &mut BitReader<R>) -> Option<u8> {
        if self.is_leaf() {
            return Some(self.byte);
        }
        
        let bit = bit_reader.read_bit()?;

        if bit == 0 {
            if let Some(ref left_node) = self.left {
                left_node.decode_byte(bit_reader)
            } else {
                None
            }
        } else {
            if let Some(ref right_node) = self.right {
                right_node.decode_byte(bit_reader)
            } else {
                None
            }
        }
    }
}
